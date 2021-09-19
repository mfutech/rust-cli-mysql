// cli arguments
//use structconf::{clap, StructConf, Error};
//use structconf::clap::Arg;
use confy;
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use tindercrypt::cryptors::RingCryptor;
use tindercrypt::errors::Error;

use base64;

pub mod fixed_key;
pub mod params;

/// generate a random key, used to initialize config file random key
fn random_key() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    rand_string
}


// Structure holding configuration of the application
// it is linked to configuration file and is updated by the application when using -s/--save option
// managed by confy
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Configuration {
    pub db_hostname: String,    // database hostname
    pub db_port: u16,           // database port
    pub db_username: String,    // database username
    pub db_cpassword: String,   // database password, encrypted
    pub db_database: String,    // database name
    secret: String,         // secret used for protecting password
    #[serde(skip)]
    db_password: String
}

impl ::std::default::Default for Configuration {
    fn default () -> Configuration {
        let mut config = Configuration {
            db_hostname: String::from("localhost"),
            db_port: 3306,
            db_username: String::from("user"),
            db_cpassword: String::from(""),
            db_database: String::from("test"),
            secret: random_key(),
            db_password: String::from(""),
        };

        config.crypt_password(&String::from("password")).expect("failed to crypt default password");
        config
    }
}

impl Configuration {
    fn make_key ( &self ) -> String {
        let mut key = self.secret.to_owned();
        key.push_str(&fixed_key::fixed_key());
        key
    }   

    fn decrypt_password ( &self ) -> Result<String, Error> {
        let cpass = base64::decode(self.db_cpassword.to_owned()).expect("error password decoding");
        let cryptor = RingCryptor::new();
        
        let password = cryptor.open(self.make_key().as_bytes(), &cpass)?;
        Ok(String::from_utf8(password).unwrap())
    }   

    fn crypt_password ( &mut self, password: &String ) -> Result<(), Error>{
        let plaintext = password.as_bytes();
        let cryptor = RingCryptor::new();
        self.db_cpassword = base64::encode(cryptor.seal_with_passphrase(self.make_key().as_bytes(), &plaintext)?);
        Ok(())
    }

    pub fn get_db_password (&self) -> String {
        self.db_password.clone()
    }
}

#[derive(Clone, Debug)]
pub struct ConfigParams {
    pub conf : Configuration,
    pub command : String,
}

pub fn get_config() -> Result<ConfigParams, confy::ConfyError> {
    let params = params::get_params();
    let mut save_path : Option<String> = None;

    let mut config : Configuration = match params.config {
        Some(config_path) => { 
            save_path = Some(config_path.clone());
            confy::load_path(config_path)?},
        None => { confy::load("mysql-cli")?}
    };

    match params.database {
        Some(db) => { config.db_database = db },
        None => {}
    };

    match params.hostname {
        Some(host) => { config.db_hostname = host },
        None => {}
    };

    match params.port {
        Some(port) => { config.db_port = port },
        None => {}
    };

    match params.username {
        Some(username) => { config.db_username = username },
        None => {}
    };

    config.db_password = match params.password {
        Some(password) => { password },
        None => { config.decrypt_password ().expect("decrypt error") }
    };

    if params.save {
        println!("needs saving");
        // crypt and store password in config
        let db_pass = config.db_password.clone();
        config.crypt_password(&db_pass).expect("crypt error");
        match save_path {
            Some(path) => { confy::store_path(path, config.clone())? },
            None => { confy::store("mysql-cli", config.clone())? }
        };
        // println!("config saved");
    }
    else {
        // println!("NOT saving");
    }

    //println!("{:?}", config);
    // decrypt password
    assert_ne!(fixed_key::fixed_key(), "a very long unique key"); // just make sure we are still usin the sample key.
    // check config

    let res = ConfigParams {
        conf:  config.clone(),
        command: params.command.clone(),
    };
    
    Ok(res)
    /*
    let path = match args.value_of("config_path") {
        Some(path) => path.to_string(),
        None => String::from("config.ini"),
    };
    println!("{:?}", args.clone());
    let params = Params::parse_file(&args, "./config.ini").unwrap();
    let res = params.clone();
    println!("{:?}", res);
    Ok(params)
    */
}

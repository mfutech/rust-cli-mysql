// cli arguments
use structconf::{clap, StructConf, Error};


#[derive(Debug,StructConf,Clone)]
pub struct Params {
    /// command to be executed
    #[conf(no_file, help="command to run")]
    pub command: String,
    /// database server
    #[conf(help="database server address, default to 127.0.0.1", default=String::from("localhost"))]
    pub hostname: String,
    //#[conf(short="C", help="config file")]
    /// config let you define which configfile to load
    //config : String,
    #[conf(help="port of data base, default to 3306(mysql)", default="3306")]
    pub port: i64,
    #[conf(help="database username")]
    pub username: String,
    #[conf(short="P", help="password for connection database")]
    pub password: String,
    #[conf(help="database name")]
    pub database: String
}

pub fn get_config() -> Result<Params, Error> {
    // take care of CLI arguments
    let app = clap::App::new("app");
    //let args = Params::parse_args(app);
    let args = Params::parse(app, "./config.ini")?;
    Ok(args)
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

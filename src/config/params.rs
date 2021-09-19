// cli arguments
use structopt::*;

#[derive(Debug,StructOpt,Clone)]
pub struct Params {
    // command to fire
    pub command: String,
    #[structopt(short, long)] //, parse(from_os_str))]
    /// path to configuraiotn file to use 
    pub config: Option<String>,
    #[structopt(short, long)]
    /// hostname to connect to
    pub hostname: Option<String>,
    #[structopt(short, long)]
    /// port to connect to
    pub port: Option<u16>,
    #[structopt(short, long)]
    /// username to connect to database
    pub username: Option<String>,
    #[structopt(long)]
    /// password to connect to databaes
    pub password: Option<String>,
    #[structopt(short, long)]
    /// database name
    pub database: Option<String>,
    #[structopt(short, long)]
    /// save paramters into configuration file
    pub save : bool
}

#[derive(Debug, StructOpt,Clone)]
pub enum Command {
    Test,
    Toto,
}

pub fn get_params() -> Params {
    // take care of CLI arguments
    let args = Params::from_args();

    println!("{:?}", args.clone());
    args
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

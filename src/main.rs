
// cli arguments
use structconf::{clap, StructConf};
use anyhow::{Context, Result};

// logging 
use env_logger;
// reading configuration files
//use std::collections::HashMap;
//use config::*;

// mysql database
use mysql::*;

mod cmd_test;
mod config;

#[derive(Debug)]
struct ConfigError(String);


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize logging environement
    env_logger::init();
    let params = config::get_config().unwrap();

    println!("Hello, world!");
    println!("Command: {}", params.command);

    let connect_string = format!(
        "mysql://{}:{}@{}:{}/{}", 
        params.username, params.password, params.hostname, params.port, params.database
    );
    
    let mysql_opts = Opts::from_url(&connect_string)?;

    let pool = mysql::Pool::new(mysql_opts).unwrap();
    let mut conn = pool.get_conn()?;
    
    match &params.command as &str {
        "test" => {
            cmd_test::cmd_test(conn)?
        },
        _ => {
            println!("current known commands : test")
        }
    } 

    Ok(())
}

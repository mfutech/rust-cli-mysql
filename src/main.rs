
//use anyhow::{Context, Result};
use anyhow::Result;

// logging 
use env_logger;

// mysql database
use mysql::*;

// configuration module
mod config;

// commands modules
mod cmd_test; // test command

#[derive(Debug)]
struct ConfigError(String);


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize logging environement
    env_logger::init();
    let config = config::get_config()?;

    println!("Command: {}", config.command.clone());

    //let conf = config.conf.clone();
    let opts = OptsBuilder::new()
        .user(Some(&config.conf.db_username))
        .pass(Some(config.conf.get_db_password()))
        .ip_or_hostname(Some(&config.conf.db_hostname))
        .tcp_port(config.conf.db_port)
        .db_name(Some(&config.conf.db_database));
    
    let pool = mysql::Pool::new(opts).unwrap();
    let conn = pool.get_conn()?;
    
    match &config.command as &str {
        "test" => {
            cmd_test::cmd_test(conn)?
        },
        _ => {
            println!("current known commands : test")
        }
    } 

    Ok(())
}

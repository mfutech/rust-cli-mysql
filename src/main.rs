
use anyhow::{Context, Result};

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
    let params = config::get_config().unwrap();

    println!("Hello, world!");
    println!("Command: {}", params.command);

    let opts = OptsBuilder::new()
        .user(Some(params.username))
        .pass(Some(params.password))
        .ip_or_hostname(Some(params.hostname))
        .tcp_port(params.port)
        .db_name(Some(params.database));
    
    let pool = mysql::Pool::new(opts).unwrap();
    let conn = pool.get_conn()?;
    
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

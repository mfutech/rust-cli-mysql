
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
use mysql::prelude::*;

mod cmd_test;
mod config;

#[derive(Debug)]
struct ConfigError(String);

#[derive(Debug,PartialEq,Eq,Clone)]
struct TestRow{
    id: i64,
    key: String,
    value: String
}

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
    println!("connecting to {}", connect_string.clone());
    let mysql_opts = Opts::from_url(&connect_string)?;

    let pool = mysql::Pool::new(mysql_opts).unwrap();
    let mut conn = pool.get_conn()?;
    let _qres = conn.query_map(
        "SELECT id, mykey, myvalue FROM Test",
        |(id, key, value) | {
            let row = TestRow { id, key, value };
            println!("{:?}", row.clone());
        })?;
    
    Ok(())
}

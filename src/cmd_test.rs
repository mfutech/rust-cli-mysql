// logging 
//use env_logger;

// mysql database
//use mysql::*;
use mysql::prelude::*;


#[derive(Debug,PartialEq,Eq,Clone)]
struct TestRow{
    id: i64,
    key: String,
    value: String
}

pub fn cmd_test(mut conn : mysql::PooledConn) -> mysql::Result<()> {
    let _qres = conn.query_map(
        "SELECT id, mykey, myvalue FROM Test",
        |(id, key, value) | {
            let row = TestRow { id, key, value };
            println!("{:?}", row.clone());
        })?;
    
    Ok(())
}

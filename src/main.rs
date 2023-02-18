use mysql::*;
use mysql::prelude::*;
use std::env;
extern crate args;
extern crate getopts;

use getopts::Occur;

use args::{Args,ArgsError};

fn main() {
    let config = match parse_args(env::args().collect()) {
        Err(e) => panic!("Args: {}",e),
        Ok(cfg) => cfg
    };

    let url: String = match env::var("ACDB_URL") {
        Err(_) => String::from("mysql://root:password@127.0.0.1:3306/acore_auth"),
        Ok(conn_string) => conn_string
    };

    let pool = match Pool::new(url.as_str()) {
        Err(e) => panic!("Pool creation failed: {}",e),
        Ok(pool)=>pool
    };

    let mut conn = match pool.get_conn() {
        Err(e) => panic!("Connect handle failed: {e:?}"),
        Ok(conn) => conn
    };
    
    let stmt = conn.prep("UPDATE realmlist set address = :address, localAddress = :local_address, name = :name").unwrap();

    let address = config.address;
    let local_address = config.local_address;
    let name = config.name;

    conn.exec_first::<u32, _, _>(&stmt,params!{ address, local_address, name }).unwrap();

}

#[derive(Debug)]
struct Config {
    address: String,
    local_address: String,
    name: String
}

fn parse_args<'a>(params:Vec<String>) -> core::result::Result<Config,ArgsError> {
    let mut args = Args::new("acdb","A utility for working with the AzerothCore database");

    args.option("a","address","IP Address of the server","IP",Occur::Req,None);
    args.option("l","local_address","Local IP Address of the server","IP",Occur::Req,None);
    args.option("n","name","Name of the server","NAME",Occur::Req,None);

    args.parse(params)?;

    let address = args.value_of("address").unwrap();
    let local_address = args.value_of("local_address").unwrap();
    let name = args.value_of("name").unwrap();

    Ok(Config{address:address,local_address:local_address,name:name})
}
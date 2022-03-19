extern crate capnp_rpc;
use std::{concat, env, include};

pub mod hello_world_capnp {
    include!(concat!(env!("OUT_DIR"), "/src/hello_world_capnp.rs"));
}

pub mod client;
pub mod server;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() >= 2 {
        let sc = args[1].as_str();
        match sc {
            "client" => return client::main().await,
            "server" => return server::main().await,
            _ => panic!("unknown"),
        }
    }

    println!("usage: {} [server | client] ADDRESS", args[0]);
    Ok(())
}

use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};
use std::net::ToSocketAddrs;

use crate::hello_world_capnp::hello_world;

struct HelloWorldImpl;

impl hello_world::Server for HelloWorldImpl {}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

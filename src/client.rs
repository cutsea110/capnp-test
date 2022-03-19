use crate::hello_world_capnp::hello_world;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};
use std::net::ToSocketAddrs;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO
    Ok(())
}

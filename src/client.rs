use crate::hello_world_capnp::hello_world;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};
use std::error::Error;
use std::net::{SocketAddr, ToSocketAddrs};

pub async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 4 {
        println!("usage: {} client HOST:PORT MESSAGE", args[0]);
        return Ok(());
    }

    let addr = args[2]
        .to_socket_addrs()?
        .next()
        .expect("could not parse address");
    let msg = &args[3];

    tokio::task::LocalSet::new()
        .run_until(try_main(addr, msg))
        .await
}

async fn try_main(addr: SocketAddr, msg: &str) -> Result<(), Box<dyn Error>> {
    let stream = tokio::net::TcpStream::connect(&addr).await?;
    stream.set_nodelay(true)?;
    let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
    let rpc_network = Box::new(twoparty::VatNetwork::new(
        reader,
        writer,
        rpc_twoparty_capnp::Side::Client,
        Default::default(),
    ));
    let mut rpc_system = RpcSystem::new(rpc_network, None);
    let hello_world: hello_world::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);

    tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));

    {
        let mut request = hello_world.say_hello_request();
        request.get().init_request().set_name(&msg);
        let reply = request.send().promise.await?;
        println!("received: {}", reply.get()?.get_reply()?.get_message()?);
    }

    Ok(())
}

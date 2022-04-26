use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};
use std::error::Error;
use std::net::{SocketAddr, ToSocketAddrs};

use crate::hello_world_capnp::hello_world;

struct HelloWorldImpl;

impl hello_world::Server for HelloWorldImpl {
    fn say_hello(
        &mut self,
        params: hello_world::SayHelloParams,
        mut results: hello_world::SayHelloResults,
    ) -> Promise<(), capnp::Error> {
        let request = pry!(pry!(params.get()).get_request());
        let name = pry!(request.get_name());
        let message = format!("Hello, {}!", name);

        println!("received: {}", message);
        results.get().init_reply().set_message(&message);

        Promise::ok(())
    }
}

pub async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} server ADDRESS[:PORT]", args[0]);
        return Ok(());
    }

    let addr = args[2]
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");

    tokio::task::LocalSet::new().run_until(try_main(addr)).await
}

async fn try_main(addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let hello_world_client: hello_world::Client = capnp_rpc::new_client(HelloWorldImpl);

    loop {
        let (stream, _) = listener.accept().await?;
        stream.set_nodelay(true)?;
        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
        let network = twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Server,
            Default::default(),
        );

        let rpc_system = RpcSystem::new(Box::new(network), Some(hello_world_client.clone().client));

        tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));
    }
}

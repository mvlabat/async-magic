// A tiny async echo server with tokio-core
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate bytes;

use tokio_proto::TcpServer;

mod codec;
mod proto;
mod service;

fn main() {
    let address = "0.0.0.0:12345".parse().unwrap();
    let server = TcpServer::new(proto::LineProto, address);
    server.serve(|| Ok(service::Counter));
}

#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate bytes;
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_timer;

use tokio_proto::TcpServer;
use futures_cpupool::CpuPool;

mod codec;
mod proto;
mod service;

fn main() {
    let address = "0.0.0.0:12345".parse().unwrap();
    let server = TcpServer::new(proto::LineProto, address);
    let pool = CpuPool::new(2);
    server.serve(move || Ok(service::TimeoutService::new(pool.clone())));
}

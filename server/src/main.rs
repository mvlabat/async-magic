#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate bytes;
extern crate clap;
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
mod config;

fn main() {
    let config = config::Config::parse();
    let address = config.address.parse().unwrap();
    let server = TcpServer::new(proto::LineProto, address);
    let pool = CpuPool::new(config.threads);
    server.serve(move || Ok(service::TimeoutService::new(pool.clone(), config.timeout)));
}

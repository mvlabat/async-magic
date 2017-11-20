extern crate clap;
extern crate futures;
extern crate rand;
extern crate tokio_core;
extern crate tokio_io;

use futures::Future;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use rand::Rng;

mod read_lines;
mod send_requests;
mod config;

fn main() {
    let config = config::Config::parse();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = config.address.parse().unwrap();

    let socket = TcpStream::connect(&addr, &handle);

    let mut random = rand::thread_rng();
    let min = config.min;
    let max = config.max;
    let mut rand = || random.gen_range(min, max).to_string() + "\n";

    let requests_data = {
        let mut data = Vec::new();
        for _ in 0..config.requests {
            data.push(Vec::from(rand().as_bytes()));
        }
        data
    };

    let request = socket.and_then(move |socket| {
        send_requests::SendRequests::new(socket, requests_data)
    });

    let response = request.and_then(move |stream| {
        read_lines::ReadLines::new(stream, config.requests)
    });

    let (_, data) = core.run(response).unwrap();
    print!("{}", String::from_utf8_lossy(&data));
}

extern crate server;

extern crate bytes;
extern crate clap;
extern crate futures;
extern crate rand;
extern crate tokio_core;
extern crate tokio_io;

use futures::{Future, Sink};
use futures::stream::Stream;
use tokio_core::net::TcpStream;
use tokio_io::AsyncRead;
use tokio_core::reactor::Core;
use rand::Rng;
use std::io;

use server::Serializable;

mod codec;
mod config;
mod requests_stream;

fn main() {
    let config = config::Config::parse();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = config.address.parse().unwrap();

    let mut random = rand::thread_rng();
    let min = config.min;
    let max = config.max;
    let mut rand = || random.gen_range(min, max);

    let requests_data = {
        let mut data = Vec::new();
        for _ in 0..config.requests {
            data.push(server::Request::Number(rand()));
        }
        data
    };

    let send_requests = TcpStream::connect(&addr, &handle).and_then(|stream| {
        let (writer, reader) = stream.framed(codec::Codec).split();
        writer
            .send_all(requests_stream::RequestsStream::new(&requests_data))
            .map(move |(writer, _)| (writer, reader))
    });

    let response = send_requests.and_then(|(_writer, reader)| {
        let mut n = 0;
        reader.for_each(move |response| {
            println!("{}", response.serialize());

            // If all the requests are processed, drop out with the error,
            // which is processed by Core.
            n += 1;
            if n < config.requests {
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "all the requests are processed",
                ))
            }
        })
    });

    core.run(response)
        .or_else(|error| match error.kind() {
            io::ErrorKind::Other => Ok(()),
            _ => Err(error),
        })
        .unwrap();
}

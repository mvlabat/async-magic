use tokio_core::net::TcpStream;
use tokio_io::io::{write_all, WriteAll};
use futures::{Async, Future, Poll};
use std::io;
use std::mem;

pub struct SendRequests {
    state: State,
    requests: Vec<Vec<u8>>,
}

enum State {
    Writing {
        write_all_future: WriteAll<TcpStream, Vec<u8>>,
    },
    Finished { stream: TcpStream },
    Empty,
}

impl Future for SendRequests {
    type Item = TcpStream;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<TcpStream, io::Error> {
        loop {
            self.state = match self.state {
                State::Writing {
                    ref mut write_all_future,
                } => match write_all_future.poll() {
                    Ok(Async::Ready((stream, _))) => State::Finished { stream },
                    Ok(Async::NotReady) => return Ok(::futures::Async::NotReady),
                    _ => unreachable!(),
                },
                State::Finished { .. } => unreachable!(),
                State::Empty => panic!("poll SendRequests after it's done"),
            };

            self.state = match mem::replace(&mut self.state, State::Empty) {
                State::Finished { stream } => {
                    if self.requests.is_empty() {
                        return Ok(stream.into());
                    }

                    State::Writing {
                        write_all_future: write_all(stream, self.requests.remove(0)),
                    }
                }
                _ => unreachable!(),
            };

            // This is an ugly hack to make server think that these are separate requests.
            use std::thread::sleep;
            use std::time::Duration;
            sleep(Duration::new(0, 50_000_000));
        }
    }
}

impl SendRequests {
    pub fn new(stream: TcpStream, mut requests: Vec<Vec<u8>>) -> SendRequests {
        SendRequests {
            state: State::Writing {
                write_all_future: write_all(stream, requests.remove(0)),
            },
            requests,
        }
    }
}

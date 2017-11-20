use tokio_core::net::TcpStream;
use futures::{Async, Future, Poll};
use std::{io, mem};
use std::io::Read;

pub struct ReadLines {
    state: State,
    lines_to_read: u64,
}

enum State {
    Reading {
        stream: TcpStream,
        buf: Vec<u8>,
        lines_read: u64,
    },
    Empty,
}

impl Future for ReadLines {
    type Item = (TcpStream, Vec<u8>);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(TcpStream, Vec<u8>), io::Error> {
        match self.state {
            State::Reading {
                ref mut stream,
                ref mut buf,
                ref mut lines_read,
            } => loop {
                let mut buffer: [u8; 1] = [0; 1];
                match stream.read_exact(&mut buffer) {
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        return Ok(Async::NotReady)
                    }
                    _ => {}
                }
                buf.push(buffer[0]);

                if buffer[0] == b'\n' {
                    *lines_read += 1;
                    if *lines_read == self.lines_to_read {
                        break;
                    }
                }
            },
            State::Empty => panic!("poll ReadLine after it's done"),
        }

        match mem::replace(&mut self.state, State::Empty) {
            State::Reading { stream, buf, .. } => Ok((stream, buf).into()),
            State::Empty => unreachable!(),
        }
    }
}

impl ReadLines {
    pub fn new(stream: TcpStream, lines_to_read: u64) -> ReadLines {
        ReadLines {
            state: State::Reading {
                buf: Vec::new(),
                stream,
                lines_read: 0,
            },
            lines_to_read,
        }
    }
}

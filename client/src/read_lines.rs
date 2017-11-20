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
                // Read per one byte, and stop at \n byte,
                // so we don't capture other response data.
                let mut buffer: [u8; 1] = [0; 1];
                match stream.read_exact(&mut buffer) {
                    // If reading is blocked, we can return Async::NotReady too:
                    // the core will poll our future again.
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        return Ok(Async::NotReady)
                    }
                    _ => {}
                }
                buf.push(buffer[0]);

                // Exit the loop if all the lines are read.
                if buffer[0] == b'\n' {
                    *lines_read += 1;
                    if *lines_read == self.lines_to_read {
                        break;
                    }
                }
            },
            State::Empty => panic!("poll ReadLine after it's done"),
        }

        // We reach this code only if all the lines are read (see the break from the loop).
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

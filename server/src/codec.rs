use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};

pub struct NumberCodec;

fn decode_error(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message)
}

impl Decoder for NumberCodec {
    type Item = u64;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<u64>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let line = buf.split_to(i);
            buf.clear();

            match str::from_utf8(&line) {
                Ok(s) => Ok(s.to_string()),
                Err(_) => Err(decode_error("invalid number (couldn't parse as  UTF-8 string)")),
            }.and_then(|line| {
                line.trim().parse::<u64>().map_err(|_| decode_error("invalid number"))
            }).map(|number| Some(number))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for NumberCodec {
    type Item = u64;
    type Error = io::Error;

    fn encode(&mut self, number: u64, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(number.to_string().as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

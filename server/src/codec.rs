use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};

pub struct NumberCodec;

fn decode_error(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

impl Decoder for NumberCodec {
    type Item = u64;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let line = buf.split_to(i);
            buf.clear();

            let result = match str::from_utf8(&line) {
                Ok(s) => Ok(s.to_string()),
                Err(_) => Err(decode_error(
                    "invalid number (couldn't parse as  UTF-8 string)",
                )),
            }.and_then(|line| {
                line.trim()
                    .parse::<u64>()
                    .map_err(|_| decode_error("invalid number"))
            });
            result.map(Some)
        } else {
            Ok(None)
        }
    }
}

impl Encoder for NumberCodec {
    type Item = Option<u64>;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        let response = match item {
            Some(number) => number.to_string(),
            None => String::from("error: timeout"),
        };
        buf.extend(response.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};
use serializable::{Request, Response, Serializable};

pub struct NumberCodec;

fn decode_error(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

impl Decoder for NumberCodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let line = buf.split_to(i);
            buf.split_to(1);

            let result = match str::from_utf8(&line) {
                Ok(s) => Ok(s.to_string()),
                Err(_) => Err(decode_error(
                    "invalid number (couldn't parse as UTF-8 string)",
                )),
            }.and_then(|line| {
                line.trim()
                    .parse::<u64>()
                    .map_err(|_| decode_error("invalid number"))
            });
            result.map(|number| Some(Request::Number(number)))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for NumberCodec {
    type Item = Response;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(item.serialize().as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

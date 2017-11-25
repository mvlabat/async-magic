use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};

use server::{Deserializable, Request, Response, Serializable};

pub struct Codec;

fn decode_error(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

impl Encoder for Codec {
    type Item = Request;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        let request = item.serialize();
        buf.extend(request.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

impl Decoder for Codec {
    type Item = Response;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let line = buf.split_to(i);
            buf.split_to(1);

            str::from_utf8(&line)
                .map(|result| Response::deserialize(String::from(result.trim())))
                .map_err(|_| decode_error("invalid UTF-8 string"))
        } else {
            Ok(None)
        }
    }
}

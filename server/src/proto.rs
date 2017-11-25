use std::io;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_proto::pipeline::ServerProto;

pub struct LineProto;

use super::codec::NumberCodec;
use serializable::{Request, Response};

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    // For this protocol style, `Request` matches the `Item` type of the codec's `Decoder`
    type Request = Request;

    // For this protocol style, `Response` matches the `Item` type of the codec's `Encoder`
    type Response = Response;

    // A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, NumberCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(NumberCodec))
    }
}

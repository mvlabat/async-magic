use std::io;
use tokio_service::Service;
use futures::{future, Future};

pub struct Counter;

impl Service for Counter {
    type Request = u64;
    type Response = u64;

    type Error = io::Error;

    type Future = Box<Future<Item = Self::Response, Error =  Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        Box::new(future::ok(req))
    }
}

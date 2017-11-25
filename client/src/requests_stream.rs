use futures::{Async, Poll, Stream};
use std::io;
use std::slice::Iter;

use server::Request;

pub struct RequestsStream<'a> {
    iter: Iter<'a, Request>,
}

impl<'a> Stream for RequestsStream<'a> {
    type Item = Request;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let r = self.iter.next().and_then(|item: &Request| Some(*item));
        Ok(Async::Ready(r))
    }
}

impl<'a> RequestsStream<'a> {
    pub fn new(requests: &'a [Request]) -> RequestsStream {
        RequestsStream {
            iter: requests.iter(),
        }
    }
}

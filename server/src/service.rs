use std::io;
use tokio_service::Service;
use tokio_core::reactor::{Handle, Timeout};
use futures::{future, Future};
use std::time::Duration;
use tokio_timer::{Timer, TimeoutError};
use futures_cpupool::CpuPool;

pub struct TimeoutService {
    pool: CpuPool,
    timer: Timer,
    delay: Duration,
}

impl TimeoutService {
    pub fn new(pool: CpuPool) -> TimeoutService {
        TimeoutService {
            pool,
            timer: Timer::default(),
            delay: Duration::new(2, 0)
        }
    }
}

impl Service for TimeoutService {
    type Request = u64;
    type Response = Option<u64>;

    type Error = io::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let task = self.pool.spawn_fn(|| -> Result<Self::Response, Self::Error> {
            for _ in 0..30000000 {};
            Ok(Some(0))
        });

        let timeout = self.timer.timeout(task, self.delay)
            .or_else(|_| Ok(None));

        Box::new(timeout)
    }
}

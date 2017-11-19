use std::io;
use tokio_service::Service;
use tokio_core::reactor::{Handle, Timeout};
use futures::{future, Future};
use std::time::{Instant, Duration};
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
        let time_started = Instant::now();

        let task = self.pool.spawn_fn(move || -> Result<Self::Response, Self::Error> {
            for _ in 0..30000000 {};
            let now = Instant::now().duration_since(time_started);
            Ok(Some(now.as_secs() * 1_000_000_000 + now.subsec_nanos() as u64))
        });

        let timeout = self.timer.timeout(task, self.delay)
            .or_else(|_| Ok(None));

        Box::new(timeout)
    }
}

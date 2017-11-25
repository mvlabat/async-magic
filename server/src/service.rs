use std::io;
use tokio_service::Service;
use futures::Future;
use std::time::{Duration, Instant};
use tokio_timer::Timer;
use futures_cpupool::CpuPool;
use serializable::{Request, Response};

pub struct TimeoutService {
    pool: CpuPool,
    timer: Timer,
    delay: Duration,
}

impl TimeoutService {
    pub fn new(pool: CpuPool, timeout: u64) -> TimeoutService {
        TimeoutService {
            pool,
            timer: Timer::default(),
            delay: Duration::new(timeout, 0),
        }
    }
}

impl Service for TimeoutService {
    type Request = Request;
    type Response = Response;

    type Error = io::Error;

    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let number = match req {
            Request::Number(n) => n,
        };
        let time_started = Instant::now();

        let task = self.pool
            .spawn_fn(move || -> Result<Self::Response, Self::Error> {
                for _ in 0..number {}
                let now = Instant::now().duration_since(time_started);
                Ok(Response::Time(
                    now.as_secs() * 1_000_000_000 + u64::from(now.subsec_nanos()),
                ))
            });

        let timeout = self.timer
            .timeout(task, self.delay)
            .or_else(|_| Ok(Response::TimeoutError));

        Box::new(timeout)
    }
}

# async-magic

## Running server
Append `-- --help` to the end of the command to read the help.
```bash
cargo run --package server
```

## Running client
Append `-- --help` to the end of the command to read the help.
```bash
cargo run --package client
```

## Info
The server is implemented with the combination of tokio proto, codec and service.

On each request a new `TimeoutService` is instantiated. It stores a clone of `CpuPool`,
so on any new request it spawns a task, wrapped in a `Timer::timeout` future.
The method works well, but with one drawback: if a timeout expires, the task will remain in the cpupool,
even if it has not started yet. The queue gets overfilled, so new upcoming tasks may fail with the timeout error too.

In order to fix the problem I tried to call `forget` on `CpuFuture`, but I wasn't able to get the `Timeout` error,
that can be destructured, as it gets converted to `std::io::Error`.

In order to implement the client, I used `TcpStream` and Sink-Stream pair. Sending requests is done with consuming custom `RequestsStream` with the sink.

For parsing command line arguments I used `clap` crate.

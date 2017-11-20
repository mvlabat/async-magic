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

In order to implement the client, I used `TcpStream` and called write/read functions directly.
I wrote my own futures for sending requests and reading the response: `SendRequests` and `ReadLines` respectively.

As I used the same connection for sending different requests, I had to use `sleep` between writing data to the stream,
otherwise the server read the incoming bytes at once and treated several requests as a single one. I realize,
this is an ugly solution, but I haven't found any other.

Reading server response is done with a single future too, so results of all the requests are displayed at the very end.
(Though, they can be theoretically streamed to the stdout right at the time they are accepted.)
As we use only one tcp connection and there is no other way to distinguish different requests,
I saw it as the only way to implement reading the response.

For parsing command line arguments I used `clap` crate.

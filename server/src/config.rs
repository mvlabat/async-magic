use clap::{App, Arg};

pub struct Config {
    pub address: String,
    pub threads: usize,
    pub timeout: u64,
}

impl Config {
    pub fn parse() -> Config {
        let matches = App::new("async-magic server")
            .version("0.1.0")
            .author("Vladyslav Batyrenko <mvlabat@gmail.com>")
            .about("Just learning async magic")
            .arg(
                Arg::with_name("bind")
                    .short("b")
                    .long("bind")
                    .value_name("IP")
                    .help("Binds server to the specified IP. Default: 127.0.0.1")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .value_name("port")
                    .help("Runs server on the specified port. Default: 12345")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("threads")
                    .short("t")
                    .long("threads")
                    .value_name("threads")
                    .help("Creates a pool with the specified number of threads. Default: 2")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("timeout")
                    .short("T")
                    .long("timeout")
                    .value_name("secs")
                    .help("Returns an error after the specified timeout expires. Default: 5")
                    .takes_value(true),
            )
            .get_matches();

        let ip = matches.value_of("bind").unwrap_or("127.0.0.1");
        let port = matches.value_of("port").unwrap_or("12345");

        Config {
            address: format!("{}:{}", ip, port),
            threads: matches
                .value_of("threads")
                .unwrap_or("2")
                .parse::<usize>()
                .unwrap(),
            timeout: matches
                .value_of("timeout")
                .unwrap_or("5")
                .parse::<u64>()
                .unwrap(),
        }
    }
}

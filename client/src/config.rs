use clap::{App, Arg};

pub struct Config {
    pub address: String,
    pub requests: u64,
    pub min: u64,
    pub max: u64,
}

impl Config {
    pub fn parse() -> Config {
        let matches = App::new("async-magic client")
            .version("0.1.0")
            .author("Vladyslav Batyrenko <mvlabat@gmail.com>")
            .about("Just learning async magic")
            .arg(
                Arg::with_name("bind")
                    .short("b")
                    .long("bind")
                    .value_name("IP")
                    .help("Connects to the server with the specified IP. Default: 127.0.0.1")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .value_name("port")
                    .help("Connects to the server with the specified port. Default: 12345")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("requests")
                    .short("r")
                    .long("requests")
                    .value_name("COUNT")
                    .help("Sends the specified number of requests to the server. Default: 10")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("min")
                    .short("n")
                    .long("min")
                    .value_name("NUMBER")
                    .help("Minimal number for the request body. Default: 100000")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("max")
                    .short("x")
                    .long("max")
                    .value_name("NUMBER")
                    .help("Maximal number for the request body. Default: 60000000")
                    .takes_value(true),
            )
            .get_matches();

        let ip = matches.value_of("bind").unwrap_or("127.0.0.1");
        let port = matches.value_of("port").unwrap_or("12345");

        Config {
            address: format!("{}:{}", ip, port),
            requests: matches
                .value_of("requests")
                .unwrap_or("10")
                .parse::<u64>()
                .unwrap(),
            min: matches
                .value_of("min")
                .unwrap_or("100000")
                .parse::<u64>()
                .unwrap(),
            max: matches
                .value_of("max")
                .unwrap_or("60000000")
                .parse::<u64>()
                .unwrap(),
        }
    }
}

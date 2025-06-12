use std::env;
use std::net::IpAddr;

pub fn ip_address() -> IpAddr {
    let addr = env::var("IP").expect(
        "Failed to read 'IP' environment variable. Set to 0.0.0.0 to run in a container environment",
    );

    let addr: IpAddr = addr
        .parse()
        .expect("Failed to parse address in IP environment variable");

    addr
}

pub fn port() -> u16 {
    let port =
        env::var("PORT").expect("Failed to read 'PORT' environment variable. Set it to 8080");

    let port = port.parse::<u16>().expect(
        "Failed to parse value of 'PORT' environment variable. Should be a valid positive integer",
    );

    port
}

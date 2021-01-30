use clap::{App, Arg};
use colored::*;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

// Reverse beacon network endpoint information
const RBN_SERVER: &str = "telnet.reversebeacon.net";
const RBN_STANDARD_PORT: u16 = 7000;

fn main() {
    let matches = App::new("Reverse Beacon Network Client")
        .version("0.0.1")
        .author("Evan Pratten <ewpratten@gmail.com>")
        .arg(
            Arg::with_name("callsign")
                .short("c")
                .long("callsign")
                .takes_value(true)
                .help("Your callsign (used to authenticate with RBN)")
                .required(true),
        )
        .get_matches();

    // Get the callsign
    let callsign = matches.value_of("callsign").unwrap().to_uppercase();
    println!("Welcome {}!", callsign.italic());

    // Set up required tcp connection to the remote server
    let endpoint = format!("{}:{}", RBN_SERVER, RBN_STANDARD_PORT);
    println!(
        "{}{}",
        "Connecting to: tcp://".bright_black(),
        endpoint.bright_black()
    );
    let mut stream = TcpStream::connect(endpoint).expect("Couldn't connect to the server...");

    // Read login header from remote
    let mut tmp =[0; 24];
     stream.read(&mut tmp).unwrap();
    println!("{}: {}", "Server asked for authentication".bright_black(), str::from_utf8(&tmp).unwrap());

    //
}

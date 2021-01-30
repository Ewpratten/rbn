use clap::{App, Arg};
use colored::*;

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

    


}

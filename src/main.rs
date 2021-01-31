use clap::{App, Arg};
use colored::*;
use hambands::band::types::{Band, Hertz, KiloHertz};
use hambands::search::get_band_by_name;
use pad::{Alignment, PadStr};
use regex::Regex;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::str;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// Reverse beacon network endpoint information
const RBN_SERVER: &str = "telnet.reversebeacon.net";
const RBN_STANDARD_PORT: u16 = 7000;

// Regex pattern used for parsing raw data
const REGEX_PATTERN: &str = r"DX de (?P<spotter>[A-Z\d\\/-]+)-#:\s*(?P<frequency>[\d.]+)\s+(?P<spotted>[A-Z\d\\/-]+)\s+(?P<mode>[A-Z\d]+)\s+(?P<snr>[\d-]+) dB\s+(?P<speed>\d+) [WPMBPS]+\s+(?P<message>[A-Za-z\\d ]+)\s*(?P<time>[0-9]{4})Z";

fn main() {
    let matches = App::new("Reverse Beacon Network Client")
        .version("0.1.2")
        .author("Evan Pratten <ewpratten@gmail.com>")
        .arg(
            Arg::with_name("callsign")
                .short("c")
                .long("callsign")
                .takes_value(true)
                .help("Your callsign (used to authenticate with RBN)")
                .required(true),
        )
        .arg(
            Arg::with_name("filtercall")
                .short("f")
                .long("filtercall")
                .takes_value(true)
                .help("Callsign to filter by")
                .required(false),
        )
        .arg(
            Arg::with_name("band")
                .short("b")
                .long("band")
                .takes_value(true)
                .help("Band name to filter by. This can be used multiple times to filter multiple bands")
                .required(false)
                .multiple(true),
        )
        .get_matches();

    // Get the callsign
    let callsign = matches.value_of("callsign").unwrap().to_uppercase();
    println!("Welcome {}!", callsign.italic().bright_blue());

    // Get the filtercall
    let has_filtercall = matches.is_present("filtercall");
    let mut filtercall = "".to_string();
    if has_filtercall {
        filtercall = matches.value_of("filtercall").unwrap().to_uppercase();
        println!(
            "Filtering by callsign: {}",
            filtercall.italic().bright_blue()
        );
    }

    // Get all bands to filter by
    let has_band_filter = matches.is_present("band");
    let mut band_filter: Vec<&Band> = Vec::new();
    if has_band_filter {
        for band_name in matches.values_of("band").unwrap().collect::<Vec<_>>() {
            // Get reference to actual band definition
            let band = get_band_by_name(band_name);

            // If this band is valid, add it to the filter
            if band.is_ok() {
                let band = band.unwrap();
                band_filter.push(band);
                println!("Adding band filter for: {}", band.name.white().italic());
            }
        }
    }

    // Set up required tcp connection to the remote server
    let endpoint = format!("{}:{}", RBN_SERVER, RBN_STANDARD_PORT);
    println!(
        "{}{}",
        "Connecting to: tcp://".bright_black(),
        endpoint.bright_black()
    );
    let mut stream = TcpStream::connect(endpoint).expect("Couldn't connect to the server...");

    // Read login header from remote
    #[allow(unused_must_use)]
    {
        stream.read(&mut [0; 24]);
    }
    println!("{}", "Server asked for authentication".bright_black());

    // Send authentication info
    stream
        .write(&format!("{}\r\n", callsign).as_bytes())
        .expect("Failed to authenticate");

    // Set up exit handler
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Set up regex matcher
    let regex_matcher = Regex::new(REGEX_PATTERN).unwrap();

    // Begin input loop
    println!("{}", "Press CTRL+C to stop".bright_black());
    let mut stream_buffer = BufReader::new(stream);
    let mut next_line = String::new();
    while running.load(Ordering::SeqCst) {
        // Consume message from RBN
        stream_buffer
            .read_line(&mut next_line)
            .expect("unable to read");

        // Handle each element
        for capture in regex_matcher.captures_iter(&next_line) {
            // Get data
            let spotter = capture["spotter"]
                .pad_to_width_with_alignment(8, Alignment::Right)
                .green()
                .bold();
            let spotted = capture["spotted"]
                .pad_to_width_with_alignment(8, Alignment::Right)
                .bright_blue()
                .bold();
            let frequency = capture["frequency"]
                .pad_to_width_with_alignment(10, Alignment::Right)
                .white()
                .bold();

            // Filtering logic
            if has_filtercall {
                if capture["spotter"] != filtercall && capture["spotted"] != filtercall {
                    continue;
                }
            }
            if has_band_filter {
                let frequency: KiloHertz = capture["frequency"].parse().unwrap();
                let frequency: Hertz = (frequency * 1000.0) as Hertz;
                let mut valid = false;
                for band in band_filter.iter() {
                    if band.low_frequency <= frequency && frequency <= band.high_frequency {
                        valid = true;
                        break;
                    }
                }
                if !valid {
                    continue;
                }
            }

            // PPrint the entry
            println!("{} spotted {} on {} KHz", &spotter, &spotted, &frequency)
        }

        // Clear the next line
        next_line.clear();
    }

    println!("{}", "\nExiting..".bright_black());
}

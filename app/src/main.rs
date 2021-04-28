use clap::{App, Arg};
use colored::*;
use hambands::band::types::{Band, Hertz};
use hambands::search::get_band_by_name;
use pad::{Alignment, PadStr};
use rbn_lib::RbnClient;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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

    // Set up exit handler
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Begin input loop
    println!("{}", "Press CTRL+C to stop".bright_black());

    // Connect to Rbn
    let mut rbn = RbnClient::new_default_addr(callsign);
    let _thread = rbn.start(Arc::new(move |data| {
        // Skip if callsign is being filtered
        if has_filtercall && (data.spotter != filtercall && data.spotted != filtercall) {
            return;
        }

        // Skip if there is a band filter
        let frequency: Hertz = (data.frequency * 1000.0) as Hertz;
        if has_band_filter {
            let mut valid = false;
            for band in band_filter.iter() {
                if band.low_frequency <= frequency && frequency <= band.high_frequency {
                    valid = true;
                    break;
                }
            }
            if !valid {
                return;
            }
        }

        // Print the entry
        println!(
            "{} spotted {} on {} KHz",
            &data
                .spotter
                .pad_to_width_with_alignment(8, Alignment::Right)
                .green()
                .bold(),
            &data
                .spotted
                .pad_to_width_with_alignment(8, Alignment::Right)
                .bright_blue()
                .bold(),
            &frequency
                .to_string()
                .pad_to_width_with_alignment(10, Alignment::Right)
                .white()
                .bold()
        )
    }));

    // Wait for kill
    loop {
        if !running.load(Ordering::SeqCst) {
            break;
        }
    }

    println!("{}", "\nExiting..".bright_black());
}

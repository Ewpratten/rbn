//! Structures representing raw data as parsed from an RBN spotter

use std::str::FromStr;

use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};

// Regex pattern used for parsing raw data
const REGEX_PATTERN: &str = r"DX de (?P<spotter>[A-Z\d\\/-]+)-#:\s*(?P<frequency>[\d.]+)\s+(?P<spotted>[A-Z\d\\/-]+)\s+(?P<mode>[A-Z\d]+)\s+(?P<snr>[\d-]+) dB\s+(?P<speed>\d+) [WPMBPS]+\s+(?P<message>[A-Za-z\\d ]+)\s*(?P<time>[0-9]{4})Z";

/// A packet of data about a single spot
#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize, Serialize)]
pub struct RbnPacket {
    /// Callsign of the spotter
    pub spotter: String,
    /// Frequency in KHz
    pub frequency: f32,
    /// Callsign of the spotted station
    pub spotted: String,
    /// Mode used
    pub mode: String,
    /// Signal strength in dB
    pub snr: f32,
    /// Signal speed
    pub speed: u8,
    /// Message
    pub message: String,
    /// Time in UTC of the packet
    pub time: String,
}

impl RbnPacket {
    /// Creates a string to identify a CQ call. Can be used to filter based on spotter (only record a CQ once, not N times due to multiple reports)
    pub fn dirty_hash(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.spotted, self.frequency, self.mode, self.time
        )
    }
}

impl FromStr for RbnPacket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(REGEX_PATTERN).unwrap();
        let mut captures = regex.captures_iter(s);
        let first = captures.next();
        if first.is_none() {
            return Err(());
        }
        let first = first.unwrap();

        // Build the output
        Ok(Self {
            spotter: first["spotter"].to_string(),
            frequency: first["frequency"].parse().unwrap_or(0.0),
            spotted: first["spotted"].to_string(),
            mode: first["mode"].to_string(),
            snr: first["snr"].parse().unwrap_or(0.0),
            speed: first["speed"].parse().unwrap_or(0),
            message: first["message"].to_string(),
            time: first["time"].to_string(),
        })
    }
}

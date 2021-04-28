//! Structures representing raw data as parsed from an RBN spotter

use regex::Captures;
use serde::{Deserialize, Serialize};

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

    /// Convert regex data into an RbnPacket
    pub fn from_regex(capture: Captures) -> Self {
        Self {
            spotter: capture["spotter"].to_string(),
            frequency: capture["frequency"].parse().unwrap_or(0.0),
            spotted: capture["spotted"].to_string(),
            mode: capture["mode"].to_string(),
            snr: capture["snr"].parse().unwrap_or(0.0),
            speed: capture["speed"].parse().unwrap_or(0),
            message: capture["message"].to_string(),
            time: capture["time"].to_string(),
        }
    }
}

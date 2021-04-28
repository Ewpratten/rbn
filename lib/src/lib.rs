pub mod packet;

use failure::Error;
use regex::Regex;
use std::{
    io::{BufRead, BufReader, Read, Write},
    sync::{mpsc::TryRecvError, Arc},
    thread::JoinHandle,
};
use std::{
    net::TcpStream,
    sync::mpsc::{self, Sender},
    thread,
};

use crate::packet::RbnPacket;

// Regex pattern used for parsing raw data
const REGEX_PATTERN: &str = r"DX de (?P<spotter>[A-Z\d\\/-]+)-#:\s*(?P<frequency>[\d.]+)\s+(?P<spotted>[A-Z\d\\/-]+)\s+(?P<mode>[A-Z\d]+)\s+(?P<snr>[\d-]+) dB\s+(?P<speed>\d+) [WPMBPS]+\s+(?P<message>[A-Za-z\\d ]+)\s*(?P<time>[0-9]{4})Z";

pub struct RbnClient {
    bind_addr: String,
    callsign: String,
    chan_to_thread: Option<Sender<bool>>,
}

impl RbnClient {
    pub fn new(bind_addr: String, callsign: String) -> Self {
        Self {
            bind_addr,
            callsign,
            chan_to_thread: None,
        }
    }

    pub fn new_default_addr(callsign: String) -> Self {
        RbnClient::new("telnet.reversebeacon.net:7000".to_string(), callsign)
    }

    pub fn start(
        &mut self,
        callback: Arc<dyn Fn(packet::RbnPacket) + Send + Sync>,
    ) -> Result<JoinHandle<()>, Error> {
        // Set up mpsc to allow control of the thread
        let (tx, rx) = mpsc::channel();
        self.chan_to_thread = Some(tx);

        // Get the stream for moving
        let mut stream = TcpStream::connect(self.bind_addr.clone())?;
        let callsign = self.callsign.clone();

        Ok(thread::spawn(move || {
            // Handle login
            stream.read(&mut [0; 24]).unwrap();
            stream
                .write(&format!("{}\r\n", callsign).as_bytes())
                .unwrap();

            // Configure regex for parsing incoming data
            let incoming_regex = Regex::new(REGEX_PATTERN).unwrap();

            // Handle data
            let mut stream_buffer = BufReader::new(stream);
            let mut next_line = String::new();
            loop {
                // Check if we need to stop running
                match rx.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }

                // Consume data from RBN
                stream_buffer.read_line(&mut next_line).unwrap();

                // Handle packets
                for capture in incoming_regex.captures_iter(&next_line) {
                    callback(RbnPacket::from_regex(capture));
                }
            }
        }))
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        if self.chan_to_thread.is_some() {
            self.chan_to_thread.as_ref().unwrap().send(true)?;
        }
        Ok(())
    }
}

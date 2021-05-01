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
                next_line.clear();
                stream_buffer.read_line(&mut next_line).unwrap();

                // Handle packets
                let packet = next_line.parse();
                if packet.is_ok() {
                    callback(packet.unwrap());
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

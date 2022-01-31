#![crate_name = "tcp_port_scanner"]

//! Mutlithreaded TCP port scanner
//! # Command Line Arguments
//! * `-h` or `--help` - Prints the help menu
//! * `-j` or `--threads` - Number of threads to use, max is 65535, default is 4
//! * `IpAddr` - The IPv4 or IPv6 Address you want to scan in
//! 
//! # Examples
//! ./ip_sniffer 192.168.1.1
//! ./ip_sniffer -j 192.168.1.1
//! 
//! # Notes
//! The command line arguments must come before the IP Address

mod args;
use args::Args;

use std::net::{IpAddr, TcpStream};
use std::env::args;
use std::io::{self, Write};
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 65535;

/// Function used to scan a port
/// Returns a port number to the thread being used
/// # Arguments
/// * `tx` - This is the thread that is currently being used
/// * `start_port` - This is the port that we start on
/// * `addr` - The IP Address we want to scan
/// * `num_threads` - The number of threads requested (Default: 4)
pub fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    // Make sure the port does not start at 0
    let mut port: u16 = start_port + 1;

    loop {
        // On connect, print a '.' for every port found and send port to RX
        // We don't care what happens if port is closed
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            },
            Err(_) => {}
        }

        // Thread will break out of loop because there are no more ports to scan
        if(MAX - port) < num_threads {
            break;
        }

        // Increase the current port for current thread by the number of threads
        port += num_threads;
    }

}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = args().collect();

    // If arguements parsed are successful, create threads 
    match Args::new(&args) {
        Ok(arguments) => {
            let num_threads = arguments.threads;
            let (tx, rx) = channel();
            for i in 0..num_threads {
                let tx = tx.clone();
                thread::spawn(move || {
                    scan(tx, i, arguments.ipaddr, num_threads);
                });
            }

            // Create vector for recieved connections
            let mut out = vec![];
            drop(tx);
            for p in rx {
                out.push(p);
            }

            // Sort ports then print them out
            println!("");
            out.sort();
            let mut counter = 0;
            for v in out {
                println!("{} is open", v);
                counter += 1;
            }

            println!("");
            println!("{} ports are open", counter);

            Ok(())
        },

        // Return error string from args.rs
        Err(e) => return Err(e)
    }
}
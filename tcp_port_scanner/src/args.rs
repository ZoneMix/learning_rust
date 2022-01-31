use std::str::FromStr;
use std::net::IpAddr;

/// Args struct for the command line arguments of the program
pub struct Args {
    pub flag: String,
    pub ipaddr: IpAddr,
    pub threads: u16
}

impl Args {
    /// Returns a struct with a valid flag, IPv4 or IPv6 Address, and amount of threads
    /// Arguments are the ones described for the program
    pub fn new(args: &[String]) -> Result<Self, &'static str> {

        // Check if there are at least 2 arguments, but no more than 4
        if args.len() < 2 {
            return Err("Not enough arguments")
        } else if args.len() > 4 {
            return Err("Too many arguments")
        }

        // Grab either the flag or IP Address
        let f = args[1].clone();

        // If args[1] is an IP Address, return the struct Args
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            Ok(Args { flag: String::from(""), ipaddr, threads: 4} )
        } else {

            // Check to see what flag was passed in by user
            // -h will print the help menu
            // -j will grab the amount of threads wanted
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!("Usage:\n");
                println!("\t-j or --threads to select how many threads you want\n");
                println!("\t-h or --help to show this help message\n");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("Too many arguments")
            } else if flag.contains("-j") || flag.contains("--threads") {
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse the thread number")
                };
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IP Address; must be IPv4 or IPv6")
                };
                return Ok(Args {flag, threads, ipaddr})
            } else {
                return Err("Invalid Syntax")
            }
        }
    }
}
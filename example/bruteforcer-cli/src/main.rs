extern crate clap;
extern crate rustify_bruteforcer;

use std::io;
use clap::{Arg, App};
use rustify_bruteforcer::prelude::*;

fn main() -> Result<(), io::Error> {
    let matches = App::new("Wifi Briteforcer")
        .version("0.0.1")
        .author("Tochukwu Nkemdilim")
        .about("Bruteforce any wifi network asynchronously!!!🎉")
        .arg(Arg::with_name("ssid")
            .short("s")
            .long("ssid")
            .multiple(false)
            .required(true)
            .takes_value(true)
            .help("SSID of network to bruteforce")
        )
        .arg(Arg::with_name("pattern")
            .short("p")
            .long("pattern")
            .multiple(false)
            .help("Pattern to use to bruteforce network")
        )
        .get_matches();
    
    
    // Get pattern
    let pattern = matches.value_of("pattern");

    // Get ssid
    if let Some(ssid) = matches.value_of("ssid") {
        // Perform bruteforce
        let result = bruteforce(Config::new(ssid, pattern))?;

        match result {
            Some(password) => println!("Congratulations 🎉🎉🎉, the password is: {}", password),
            None => println!("Don't know what went wrong, but was unable to buteforce the network")
        }
    }

    Ok(())
}

fn bruteforce(config: Config) -> Result<Option<String>, io::Error> {
    Ok(WifiBruteforcer::new(config)?.perform_attack()?)
}

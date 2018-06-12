extern crate clap;
extern crate rustify_bruteforcer;

use clap::{Arg, ArgMatches, App, SubCommand};
use rustify_bruteforcer::prelude::*;

fn main() {
    let matches = App::new("Wifi Briteforcer")
        .version("0.0.1")
        .author("Tochukwu Nkemdilim")
        .about("Bruteforce any wifi network asynchronously!!!")
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
    
    if let ssid = matches.value_of("ssid") {
        let pattern = None;

        if let selectedPattern = matches.value_of("pattern") {
            pattern = Some(selectedPattern);
        }

        // perorms bruteforce
        bruteforce(Config::new(ssid, pattern));
    }
}

fn bruteforce(config: Config) {
    let mut bruteforcer = match WifiBruteforcer::new(config) {
        Ok(client) => client,
        Err(err) => panic!("{}", err),
    };

    println!("Starting the bruteforcer.");
    match bruteforcer.perform_attack() {
        Ok(password) => println!("The password is: {}", password.unwrap()),
        Err(err) => eprintln!("{}", err),
    }
}

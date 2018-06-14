extern crate rustify_bruteforcer;

use std::io;
use rustify_bruteforcer::prelude::*;

fn main() -> Result<(), io::Error> {
    let config = Config::new("AndroidAp", Some("belm2453"));
    let mut bruteforcer = WifiBruteforcer::new(config)?;

    match bruteforcer.perform_attack()? {
        Some(password) => println!("The password is: {}", password),
        None => println!("Don't know what went wrong, but was unable to buteforce the network")
    }

    Ok(())
}

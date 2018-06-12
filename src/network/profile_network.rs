use network::providers::{Linux, Windows};
use network::Network;
use wifi::WifiBruteforcer;

use std::io::{Error, ErrorKind};

pub(crate) struct ProfileNetwork {
  handler: Box<Network>,
}

/// Profile Network handler responsible to connect to a wireless network.
impl ProfileNetwork {
  pub fn new(name: &str) -> Result<Self, Error> {
    if cfg!(target_os = "windows") {
      let handler = Windows::new(name.into())?;
      return Ok(ProfileNetwork {
        handler: Box::new(handler),
      });
    } else if cfg!(target_os = "linux") {
      let handler = Linux::new(name.into())?;
      return Ok(ProfileNetwork {
        handler: Box::new(handler),
      });
    }

    Err(Error::new(
      ErrorKind::Other,
      "The Specified OS is not supported",
    ))
  }

  // pub fn connect(&self) -> bool {
  //   self.handler.connect()
  // }

  pub fn perform_attack(&self, bruteforcer: &mut WifiBruteforcer) -> Result<Option<String>, Error> {
    self.handler.perform_attack(bruteforcer)
  }
}

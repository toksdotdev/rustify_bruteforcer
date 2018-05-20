use network::providers;
use network::Network;

use std::io;

pub(crate) struct ProfileNetwork {
  handler: Box<Network>,
}

/// Profile Network handler responsible to connect to a wireless network.
impl ProfileNetwork {
  pub fn new(name: &str) -> Result<Self, io::Error> {
    // later match the target os here

    match providers::Windows::new(name.into()) {
      Ok(handler) => Ok(ProfileNetwork {
        handler: Box::new(handler),
      }),
      Err(err) => Err(err),
    }
  }

  pub fn connect(&self) -> bool {
    self.handler.connect()
  }
}

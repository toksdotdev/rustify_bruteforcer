use network::providers::{Linux, Windows};
use network::{Network, NetworkType};

use std::io::{Error, ErrorKind};

pub(crate) struct ProfileNetwork {
  handler: Box<Network>,
}

/// Profile Network handler responsible to connect to a wireless network.
impl ProfileNetwork {
  pub fn new(name: &str) -> Result<Self, Error> {
    // later match the target os here

    if cfg!(target_os = "windows") {
      let handler = Windows::new(name.into())?;
      return Ok(ProfileNetwork {
        handler: Box::new(handler),
      });
    } else if cfg!(target_os = "linux") {
      let handler = Linux::new(name.into())?;

      // refactor to use and_then() & map_Err
      let handler = match handler.check_if_web_or_wpa() {
        Ok(t) => match t {
          NetworkType::WEP => Linux {
            name: handler.name,
            network_type: NetworkType::WEP,
          },
          _ => Linux {
            name: handler.name,
            network_type: NetworkType::WPA,
          },
        },
        Err(_) => return Err(Error::new(ErrorKind::Other, "Failed to parse")), // usethe NetworkTypeParseError::IoError here
      };

      return Ok(ProfileNetwork {
        handler: Box::new(handler),
      });
    }

    Err(Error::new(
      ErrorKind::Other,
      "The Specified OS is not supported",
    ))
  }

  pub fn connect(&self) -> bool {
    self.handler.connect()
  }
}

use network::{Network, NetworkType, NetworkTypeParseError};
use std::io;
use std::process::{Command, Output};

pub struct Linux {
  pub name: String,
  pub network_type: NetworkType,
}

impl Linux {
  pub fn new(name: String) -> Result<Self, io::Error> {
    // detect the network type here (and call: check_if_web_or_wpa here)

    Ok(Linux {
      name,
      network_type: NetworkType::WEP,
    })
  }

  pub fn check_if_web_or_wpa(&self) -> Result<NetworkType, NetworkTypeParseError> {
    let command = format!(
      "nmcli con list id \"{}\" | awk '/key-mgmt/ {{ print $2 }}' ",
      self.name
    );

    Command::new("sh")
      .args(&["/C", &command[..]])
      .output()
      .map_err(|err| NetworkTypeParseError::IoError(err))
      .and_then(|output| {
        String::from_utf8(output.stdout)
          .map_err(|err| NetworkTypeParseError::FromUtf8Error(err))
          .and_then(|result| match result.as_ref() {
            "wpa-psk" => Ok(NetworkType::WPA),
            _ => Ok(NetworkType::WEP),
          })
      })
  }

  pub fn connect_to_wep_network(&self) -> Result<Output, io::Error> {
    let command = format!(
      "iwconfig wlan0 essid {} key s:wehhnhkjjb2jh3jh2h4",
      self.name
    ); // using some dummy password here

    Command::new("sh").args(&[&command[..]]).output()
  }

  pub fn connect_to_wpa_network(&self) -> Result<Output, io::Error> {
    let command = format!(
      "wpa_passphrase {} wehhnhkjjb2jh3jh2h4 > wpa.conf", // dynamically generate differennt version of file (if running sync)
      self.name
    ); // using some dummy password here

    Command::new("sh").args(&[&command[..]]).output()
  }
}

impl Network for Linux {
  fn connect(&self) -> bool {
    match self.network_type {
      NetworkType::WEP => self
        .connect_to_wep_network()
        .map_err(|_err| false)
        .unwrap()
        .status
        .success(),
      _ => self
        .connect_to_wep_network()
        .map_err(|_err| false)
        .unwrap()
        .status
        .success(),
    }
  }
}

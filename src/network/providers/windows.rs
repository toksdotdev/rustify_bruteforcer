use network::Network;
use std::io;
use std::process::Command;

pub struct Windows {
  name: String,
}

impl Windows {
  pub fn new(name: String) -> Result<Self, io::Error> {
    let profile_file_name = format!("netsh wlan add profile filename=\"{}\"", name);

    Command::new("cmd")
      .args(&["/C", &profile_file_name[..]])
      .output()?;

    Ok(Windows { name })
  }
}

impl Network for Windows {
  // type Machine = Self;
  fn connect(&self) -> bool {
    let ssid = format!("netsh wlan connect name=\"{}\"", self.name);
    let output = Command::new("cmd").args(&["/C", &ssid[..]]).output();

    match output {
      Ok(res) => res.status.success(),
      Err(_) => false,
    }
  }
}

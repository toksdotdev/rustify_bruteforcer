use std::process::Command;
use std::io;

pub(crate) struct ProfileNetwork<'a> {
  name: &'a str,
}

impl<'a> ProfileNetwork<'a> {
  pub fn new(name: &'a str) -> Result<Self, io::Error> {
    let profile_file_name = format!("netsh wlan add profile filename=\"{}\"", name);

    let output = Command::new("cmd")
      .args(&["/C", &profile_file_name[..]])
      .output();

    match output {
      Ok(_) => Ok(ProfileNetwork { name }), // return res.status.success();
      Err(err) => Err(err),
    }
  }

  pub fn connect(&self) -> bool {
    let ssid = format!("netsh wlan connect name=\"{}\"", self.name);
    let output = Command::new("cmd").args(&["/C", &ssid[..]]).output();

    match output {
      Ok(res) => res.status.success(),
      Err(_) => false,
    }
  }
}

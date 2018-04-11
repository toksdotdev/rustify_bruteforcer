use config::bruteforce_patterns::BruteforcePattern;

pub struct Config<'a> {
  pub ssid: &'a str,
  pub pattern_password: String,
  // bruteforce_pattern: BruteforcePattern,
}

impl<'a> Config<'a> {
  pub fn new(ssid: &'a str, pattern_password: Option<String>) -> Self {
    let (password, _pattern) = match pattern_password {
      Some(expr) => (expr, BruteforcePattern::Pattern),
      None => (
        String::from("0123456789AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz <SP>!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"),
        BruteforcePattern::Aggressive
        )
    };

    Config {
      ssid,
      pattern_password: password,
      // bruteforce_pattern: pattern,
    }
  }
}
const DEFAULT_PATTERN: &str = 
    "0123456789AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz <SP>!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub ssid: &'a str,
    pub pattern_password: String,
}

impl<'a> Config<'a> {
    pub fn new(ssid: &'a str, pattern_password: Option<&'a str>) -> Self {
        let password = match pattern_password {
            Some(expr) => expr,
            None => DEFAULT_PATTERN
        };

        Config {
            ssid,
            pattern_password: String::from(password)
        }
    }
}

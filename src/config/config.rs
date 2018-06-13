#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub ssid: &'a str,
    pub pattern_password: String,
}

impl<'a> Config<'a> {
    pub fn new(ssid: &'a str, pattern_password: Option<&'a str>) -> Self {
        let password = match pattern_password {
            Some(expr) => expr,
            None => "0123456789AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz <SP>!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"
        };

        Config {
            ssid,
            pattern_password: String::from(password)
        }
    }
}

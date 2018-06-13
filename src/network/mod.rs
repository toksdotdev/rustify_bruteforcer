mod profile_network;
pub mod providers;

use std::{fmt, io};
use std::string::FromUtf8Error;
pub(crate) use self::profile_network::ProfileNetwork;

pub trait Network {
    /// Makes an attempt to connect to a selected wireless network with password specified.
    fn connect(&self, password: &str) -> bool;
}

// Improve upon this.
impl fmt::Debug for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Network")
    }
}

pub enum NetworkType {
    WEP,
    WPA,
    WPA2,
}

pub enum NetworkTypeParseError {
    FromUtf8Error(FromUtf8Error),
    IoError(io::Error),
}

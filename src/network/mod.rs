mod profile_network;
pub mod providers;

pub(crate) use self::profile_network::ProfileNetwork;
use std::string::FromUtf8Error;
use std::io;

pub trait Network {
  fn connect(&self) -> bool;
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

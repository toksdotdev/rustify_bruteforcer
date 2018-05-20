mod profile_network;
pub mod providers;

pub(crate) use self::profile_network::ProfileNetwork;

pub trait Network {

  fn connect(&self) -> bool;
}

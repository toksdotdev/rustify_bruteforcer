#![feature(toowned_clone_into)]

mod network;
mod file_writer;
mod config;
mod handlers;
mod wifi;
mod stubs;

pub use wifi::*;
pub use config::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        
    }
}

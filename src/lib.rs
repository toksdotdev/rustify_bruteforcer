#![feature(toowned_clone_into)]
#![feature(associated_type_defaults)]
#![allow(dead_code)]

mod config;
mod handlers;
mod network;
mod stubs;
mod wifi;

pub mod prelude {
    pub use config::Config;
    pub use wifi::WifiBruteforcer;
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }

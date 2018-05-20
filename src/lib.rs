#![feature(toowned_clone_into)]
#![feature(associated_type_defaults)]

mod config;
mod file_writer;
mod handlers;
mod network;
mod stubs;
mod wifi;

#[cfg(all(target_os = "windows"))]
pub mod prelude {
    pub use config::Config;
    pub use wifi::WifiBruteforcer;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

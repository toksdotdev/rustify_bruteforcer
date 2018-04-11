use std::fs::File;
use std::io::{self, Write};

pub(crate) struct FileWriter;

impl FileWriter {
  pub fn write(file_path: &str, data: &str) -> Result<(), io::Error> {
    // Recreate the file and dump the processed contents to it
    let mut dst = File::create(file_path)?;
    dst.write(data.as_bytes())?;

    Ok(())
  }
}
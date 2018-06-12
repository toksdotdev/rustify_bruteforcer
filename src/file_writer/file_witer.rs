use std::{io, fs::{self}};

pub(crate) struct FileWriter;

impl FileWriter {
  pub fn write(file_path: &str, data: &str) -> Result<(), io::Error> {
    // Recreate the file and dump the processed contents to it
    fs::write(&file_path, data.as_bytes())?;
    Ok(())
  }
}

use std::io::{self, Write};
use std::fs::File;
use stubs::windows_wifi_profile;

pub(crate) struct NetworkXmlProfileHandler {
  pub(crate) content: Option<String>,
}

impl NetworkXmlProfileHandler {
  pub fn new() -> Result<Self, io::Error> {
    let mut handler = NetworkXmlProfileHandler {
      content: None,
    };

    match handler.read_from_stub() {
      Ok(content) => {
        handler.content = Some(content);
      }
      Err(err) => {
        return Err(err);
      }
    };

    Ok(handler)
  }

  fn read_from_stub(&self) -> Result<String, io::Error> {
    Ok(windows_wifi_profile::get_wifi_profile())
  }
  
  pub fn to_file(&mut self, file_path: &str) -> Result<(), io::Error> {
    // Recreate the file and dump the processed contents to it
    let mut dst = File::create(file_path)?;

    if self.content == None {
      self.content = Some(self.read_from_stub()?);
    }
    dst.write(self.content.as_ref().unwrap().as_bytes())?;

    Ok(())
  }
}
use image::ImageReader;
use std::fmt::Display;

#[derive(Debug)]
struct EncodeError {
  msg: String,
}

impl Display for EncodeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl std::error::Error for EncodeError {}

pub fn encode(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
  let image = ImageReader::open(path)?.decode()?;

  let image = image.as_rgba8().ok_or_else(|| EncodeError {
    msg: format!("Could not load image {}", path.to_str().unwrap_or("")),
  })?;

  println!("{:#?}", image.get_pixel(0, 0).0);

  Ok(())
}

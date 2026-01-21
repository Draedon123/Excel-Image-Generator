use image::{GenericImageView, ImageReader};
use rust_xlsxwriter::{Color, Format, workbook::Workbook};
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

const CELL_SIZE: u32 = 10;
pub fn encode(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
  let image = ImageReader::open(path)?.decode()?;

  let (width, height) = image.dimensions();

  if width > u16::MAX.into() || height > u16::MAX.into() {
    return Err(Box::new(EncodeError {
      msg: format!("Image exceeds maximum side length of {}", u16::MAX),
    }));
  };

  let image = image.as_rgba8().ok_or_else(|| EncodeError {
    msg: format!("Could not load image {}", path.to_str().unwrap_or("")),
  })?;

  let mut workbook = Workbook::new();
  let worksheet = workbook.add_worksheet();

  for x in 0..width {
    let _ = worksheet.set_column_width_pixels(x.try_into().unwrap(), CELL_SIZE);

    for y in 0..height {
      let _ = worksheet.set_row_height_pixels(y, CELL_SIZE);

      let pixel = image.get_pixel(x, y).0;

      let a = (pixel[3] as f32) / 255.0;
      let r = ((pixel[0] as f32) * a) as u32;
      let g = ((pixel[1] as f32) * a) as u32;
      let b = ((pixel[2] as f32) * a) as u32;

      let format = Format::new().set_background_color(Color::RGB(r << 16 | g << 8 | b));

      let _ = worksheet.set_cell_format(y, x.try_into().unwrap(), &format);
    }
  }

  let _ = workbook.save("output.xlsx");

  Ok(())
}

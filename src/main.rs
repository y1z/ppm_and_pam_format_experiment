mod color;
mod make_patterns;
mod pixel_buffer;
mod vector2;
mod yy_random_func;
pub mod util {
  pub mod util_traits;
}

use make_patterns::make_checker_pattern;
use pixel_buffer::PixelBufferRGB;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::string::*;

const DEFAULT_PATH_PPM: &'static str = "default_output.ppm";
const DEFAULT_PATH_PAM: &'static str = "default_output.pam";

fn main() {
  let mut buffer = PixelBufferRGB::create(Some(color::RGB::WHITE), 128, 128, None);
  let buffer_width = buffer.get_width() as usize;
  let buffer_height = buffer.get_height() as usize;

  {
    let mut mut_slice = buffer.get_buffer_as_slice_mut();
    make_patterns::make_circle_pattern(
      mut_slice,
      buffer_width,
      buffer_height,
      color::RGB::RED,
      None,
    );
  }
  let circle_pattern_result = save_as_ppm(
    buffer_width,
    buffer_height,
    buffer.get_buffer_as_slice(),
    String::from("circle_pattern.ppm"),
    false,
  );

  if circle_pattern_result.is_err() {
    panic!("{}", circle_pattern_result.unwrap_err());
  }
}

pub fn save_as_ppm(
  width: usize,
  height: usize,
  color: &[color::RGB],
  path: String,
  be_silent: bool,
) -> std::io::Result<()> {
  let mut file = File::create(&path)?;
  write!(file, "P6 {} {} 255\n", width, height)?;

  if !be_silent {
    println!("saving file to path {}", path);
  }

  let mut writer = BufWriter::new(file);
  for y in 0..height {
    for x in 0..width {
      let extract_color = color[(width * y) + x];
      let final_extract_color: [u8; 3] = [
        (extract_color.red),
        (extract_color.green),
        (extract_color.blue),
      ];
      writer.write(&final_extract_color)?;
    }
  }
  Ok(())
}

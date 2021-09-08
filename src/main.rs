mod color;
mod pixel_buffer;
use pixel_buffer::PixelBuffer;
use std::fs::File;
use std::io::{BufWriter, Write};

const DEFAULT_PATH_PPM: &'static str = "default_output.ppm";
const DEFAULT_PATH_PAM: &'static str = "default_output.pam";
const DEFAULT_WIDTH: usize = 16;
const DEFAULT_HEIGHT: usize = 16;
const DEFAULT_SIZE: usize = DEFAULT_WIDTH * DEFAULT_HEIGHT;

fn main() {
  let mut new_buffer =
    pixel_buffer::PixelBufferRGB::create(Some(color::RGB::WHITE), 128, 128, None);

  let width = new_buffer.get_width() as usize;
  let height = new_buffer.get_height() as usize;
  let mut mut_slice = new_buffer.get_buffer_as_slice_mut();
  make_checker_pattern(
    &mut mut_slice,
    width,
    height,
    color::RGB::BLUE,
    color::RGB::BLACK,
  );
  println!("saving image");
  let result = save_as_ppm(
    new_buffer.get_width() as usize,
    new_buffer.get_height() as usize,
    new_buffer.get_buffer_as_slice(),
  );
  if result.is_err() {
    panic!("An error occurred when saving to a file");
  }
}

pub fn save_as_ppm(width: usize, height: usize, color: &[color::RGB]) -> std::io::Result<()> {
  let mut file = File::create(DEFAULT_PATH_PPM)?;
  write!(file, "P6 {} {} 255\n", width, height)?;

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

pub fn make_checker_pattern(
  buffer_output: &mut [color::RGB],
  width: usize,
  height: usize,
  forground_color: color::RGB,
  background_color: color::RGB,
) {
  for y in 0..height {
    let indice_to_draw_at = (y % 2 == 0) as usize;
    for x in 0..width {
      if x % 2 == indice_to_draw_at {
        buffer_output[(width * y) + x] = forground_color;
      } else {
        buffer_output[(width * y) + x] = background_color;
      }
    }
  }
}

mod color;
mod make_patterns;
mod pam_format_descriptor;
mod pixel_buffer;
mod vector2;
mod yy_random_func;
pub mod util {
  pub mod util_traits;
}

use pam_format_descriptor::PamFormatDescriptor;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::string::*;

//const DEFAULT_PATH_PPM: &'static str = "default_output.ppm";
//const DEFAULT_PATH_PAM: &'static str = "default_output.pam";

fn main() {
  let patterns = make_patterns::make_one_of_each_pattern(
    128,
    128,
    Some(color::RGB::PURPLE),
    Some(color::RGB::GREEN),
  );

  let mut index = 0;
  for pat in patterns.iter() {
    let buff_width = pat.0.get_width();
    let buff_height = pat.0.get_height();
    let desc = PamFormatDescriptor::make_rgb_descriptor(buff_width, buff_height, None);

    let file_name = format!("{}.pam", pat.1);
    let result = save_as_pam_rgb(pat.0.get_buffer_as_slice(), desc, file_name);
    index = index + 1;
    if result.is_err() {
      print!("error : [ {} ]", result.unwrap_err());
    }
  }

  for pat in patterns.iter() {
    let width = pat.0.get_width();
    let height = pat.0.get_height();
    let result = save_as_ppm_rgb(
      width,
      height,
      pat.0.get_buffer_as_slice(),
      pat.1.clone(),
      false,
    );
    if result.is_err() {
      print!("error : [ {} ]", result.unwrap_err());
    }
  }
}

pub fn save_as_ppm_rgb(
  width: usize,
  height: usize,
  color: &[color::RGB],
  path: String,
  be_silent: bool,
) -> std::io::Result<()> {
  let final_path = match path.ends_with(".ppm") {
    true => path,
    false => path + ".ppm",
  };

  let mut file = File::create(&final_path)?;
  write!(file, "P6 {} {} 255\n", width, height)?;

  if !be_silent {
    println!("saving file to path {}", final_path);
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

/**
*     P7
      WIDTH 227
      HEIGHT 149
      DEPTH 3
      MAXVAL 255
      TUPLTYPE RGB
      ENDHDR
*/
pub fn save_as_pam_rgb(
  color_buffer: &[color::RGB],
  descriptor: PamFormatDescriptor,
  name_of_file: String,
) -> std::io::Result<()> {
  let width = descriptor.width;
  let height = descriptor.height;
  let type_type = descriptor.tuple_type;
  let depth = match type_type.has_alpha() {
    true => 1 + descriptor.depth,
    false => descriptor.depth,
  };

  let type_string = type_type.to_string();
  println!(
    "saving file of type {}\nName of file is \"{}\"\n",
    type_string, name_of_file
  );
  let mut file = File::create(&name_of_file)?;

  writeln!(file, "P7")?;
  writeln!(file, "WIDTH {}", width)?;
  writeln!(file, "HEIGHT {}", height)?;
  writeln!(file, "DEPTH {}", depth)?;
  writeln!(file, "MAXVAL {}", descriptor.max_val)?;
  writeln!(file, "TUPLTYPE  {}", descriptor.tuple_type.to_string())?;
  writeln!(file, "ENDHDR")?;

  const TEMP_ALPHA_VALUE: [u8; 1] = [255];

  let mut writer = BufWriter::new(file);
  for z in 0..depth {
    for y in 0..height {
      for x in 0..width {
        if z == 0 {
          let temp: [u8; 1] = [color_buffer[(width * y) + x].red];
          writer.write(&temp)?;
        } else if z == 1 {
          let temp: [u8; 1] = [color_buffer[(width * y) + x].green];
          writer.write(&temp)?;
        } else if z == 2 {
          let temp: [u8; 1] = [color_buffer[(width * y) + x].blue];
          writer.write(&temp)?;
        } else {
          writer.write(&TEMP_ALPHA_VALUE)?;
        }
      }
    }
  }

  Ok(())
}

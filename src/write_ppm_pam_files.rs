use crate::color;
use crate::pam_format_descriptor::PamFormatDescriptor;
use std::fs::File;
use std::io::{BufWriter, Write};

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

pub fn write_pam_header(
  descriptor: &PamFormatDescriptor,
  file: &mut std::fs::File,
) -> std::io::Result<()> {
  writeln!(file, "P7")?;
  writeln!(file, "WIDTH {}", descriptor.width)?;
  writeln!(file, "HEIGHT {}", descriptor.height)?;
  writeln!(file, "DEPTH {}", descriptor.depth)?;
  writeln!(file, "MAXVAL {}", descriptor.max_val)?;
  writeln!(file, "TUPLTYPE  {}", descriptor.tuple_type.to_string())?;
  writeln!(file, "ENDHDR")?;

  Ok(())
}

fn write_pam_rgba(
  descriptor: &PamFormatDescriptor,
  file: &mut std::fs::File,
  color_buffer: &[color::RGBA],
) -> std::io::Result<()> {
  let mut writer = BufWriter::new(file);
  for z in 0..descriptor.depth {
    for y in 0..descriptor.height {
      for x in 0..descriptor.width {
        if z == 0 {
          let temp: [u8; 1] = [color_buffer[(descriptor.width * y) + x].rgb.red];
          writer.write(&temp)?;
        } else if z == 1 {
          let temp: [u8; 1] = [color_buffer[(descriptor.width * y) + x].rgb.green];
          writer.write(&temp)?;
        } else if z == 2 {
          let temp: [u8; 1] = [color_buffer[(descriptor.width * y) + x].rgb.blue];
          writer.write(&temp)?;
        } else if z == 3 {
          let temp: [u8; 1] = [color_buffer[(descriptor.width * y) + x].alpha];
          writer.write(&temp)?;
        }
      }
    }
  }

  Ok(())
}

pub fn write_pam_rgb(
  descriptor: &PamFormatDescriptor,
  file: &mut std::fs::File,
  color_buffer: &[color::RGB],
) -> std::io::Result<()> {
  let mut writer = BufWriter::new(file);

  for z in 0..descriptor.depth {
    for y in 0..descriptor.height {
      for x in 0..descriptor.width {
        if z == 0 {
          let temp: [u8; 1] = [color_buffer[(descriptor.width * y) + x].red];
          writer.write(&temp)?;
        } else if z == 1 {
          let temp: [u8; 1] = [color_buffer[(descriptor.width * y) + x].green];
          writer.write(&temp)?;
        } else if z == 2 {
          let temp: [u8; 1] = [color_buffer[(descriptor.width * y) + x].blue];
          writer.write(&temp)?;
        }
      }
    }
  }

  Ok(())
}

fn check_for_pam_extention(name_of_file: &String) -> bool {
  name_of_file.ends_with(".pam")
}

/**
  P7
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
  let type_string = descriptor.tuple_type.to_string();
  println!(
    "saving file of type {}\nName of file is \"{}\"\n",
    type_string, name_of_file
  );

  let final_name_of_file = match check_for_pam_extention(&name_of_file) {
    true => name_of_file,
    false => name_of_file + ".pam",
  };

  let mut file = File::create(&final_name_of_file)?;
  write_pam_header(&descriptor, &mut file)?;
  write_pam_rgb(&descriptor, &mut file, &color_buffer)?;
  Ok(())
}

pub fn save_as_pam_rgba(
  color_buffer: &[color::RGBA],
  descriptor: PamFormatDescriptor,
  name_of_file: String,
) -> std::io::Result<()> {
  let type_string = descriptor.tuple_type.to_string();
  println!(
    "saving file of type {}\nName of file is \"{}\"\n",
    type_string, name_of_file
  );

  let final_name_of_file = match check_for_pam_extention(&name_of_file) {
    true => name_of_file,
    false => name_of_file + ".pam",
  };

  let mut file = File::create(&final_name_of_file)?;

  write_pam_header(&descriptor, &mut file)?;
  write_pam_rgba(&descriptor, &mut file, &color_buffer);

  Ok(())
}

mod color;
mod make_patterns;
mod pam_format_descriptor;
mod pixel_buffer;
mod vector2;
mod write_ppm_pam_files;
mod yy_random_func;
pub mod util {
  pub mod util_traits;
}

use crate::write_ppm_pam_files::*;
use std::string::*;

//const DEFAULT_PATH_PPM: &'static str = "default_output.ppm";
//const DEFAULT_PATH_PAM: &'static str = "default_output.pam";

fn main() {
  // let patterns = make_patterns::make_one_of_each_pattern(
  //   128,
  //   128,
  //   Some(color::RGB::PURPLE),
  //   Some(color::RGB::GREEN),
  // );

  let checker_pattern_ppm = make_patterns::make_checker_pattern_rgb(
    255,
    255,
    color::RGBA::PURE_BLUE,
    color::RGBA::create(u8::MAX, 0, 0, (u8::MAX / 2) / 2),
  );

  let checker_pattern_pam = make_patterns::make_checker_pattern_rgba(
    255,
    255,
    color::RGBA::PURE_GREEN,
    color::RGBA::create(u8::MAX, 0, 0, (u8::MAX / 2) / 2),
  );

  {
    println!("");
    let res = save_as_ppm_rgb(
      checker_pattern_ppm.get_width(),
      checker_pattern_ppm.get_height(),
      checker_pattern_ppm.get_buffer_as_slice(),
      String::from("checker_pattern_test"),
      false,
    );
    if let Err(x) = res {
      panic!("\n\n\n\n error : [ {} ]", x);
    }
  }
  {}

  // let mut index = 0;
  // for pat in patterns.iter() {
  //   let buff_width = pat.0.get_width();
  //   let buff_height = pat.0.get_height();
  //   let desc = PamFormatDescriptor::make_rgb_descriptor(buff_width, buff_height, None);

  //   let file_name = format!("{}.pam", pat.1);
  //   let result = save_as_pam_rgb(pat.0.get_buffer_as_slice(), desc, file_name);
  //   index = index + 1;
  //   if result.is_err() {
  //     print!("error : [ {} ]", result.unwrap_err());
  //   }
  // }

  // for pat in patterns.iter() {
  //   let width = pat.0.get_width();
  //   let height = pat.0.get_height();
  //   let result = save_as_ppm_rgb(
  //     width,
  //     height,
  //     pat.0.get_buffer_as_slice(),
  //     pat.1.clone(),
  //     false,
  //   );
  //   if result.is_err() {
  //     print!("error : [ {} ]", result.unwrap_err());
  //   }
  // }
}

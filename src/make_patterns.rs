use crate::color;
use crate::yy_random_func;
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

pub fn make_random_rainbow_pattern(buffer_output: &mut [color::RGB], width: usize, height: usize) {
  const COLOR_BUFFER_COUNT: usize = 8;
  const COLOR_BUFFER: [color::RGB; COLOR_BUFFER_COUNT] = [
    color::RGB::BLUE,
    color::RGB::RED,
    color::RGB::GREEN,
    color::RGB::CYAN,
    color::RGB::MAGENTA,
    color::RGB::YELLOW,
    color::RGB::PINK,
    color::RGB::PURPLE,
  ];
  let delta_between_ranges = (u32::MAX as usize) / COLOR_BUFFER.len();
  for y in 0..height {
    for x in 0..width {
      let rand_num = unsafe { yy_random_func::msws32_next() };
      let mut index = rand_num as usize / delta_between_ranges;
      if index >= COLOR_BUFFER_COUNT {
        index = COLOR_BUFFER_COUNT - 1;
      }
      buffer_output[(width * y) + x] = COLOR_BUFFER[index];
    }
  }
}

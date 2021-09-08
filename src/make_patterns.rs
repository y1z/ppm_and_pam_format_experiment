use crate::color;
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

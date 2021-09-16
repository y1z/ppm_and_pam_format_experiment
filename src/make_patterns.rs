use crate::color;
use crate::pixel_buffer;
use crate::vector2::*;
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
  const COLOR_BUFFER: [color::RGB; 8] = [
    color::RGB::BLUE,
    color::RGB::RED,
    color::RGB::GREEN,
    color::RGB::CYAN,
    color::RGB::MAGENTA,
    color::RGB::YELLOW,
    color::RGB::PINK,
    color::RGB::PURPLE,
  ];
  let color_buffer_len = COLOR_BUFFER.len();
  let delta_between_ranges = (u32::MAX as usize) / color_buffer_len;
  for y in 0..height {
    for x in 0..width {
      let rand_num = unsafe { yy_random_func::msws32_next() };
      let mut index = rand_num as usize / delta_between_ranges;
      if index >= color_buffer_len {
        index = color_buffer_len - 1;
      }
      buffer_output[(width * y) + x] = COLOR_BUFFER[index];
    }
  }
}

// TODO : Make the triangle less scuffed
pub fn make_triangle_pattern(
  buffer_output: &mut [color::RGB],
  width: usize,
  height: usize,
  triangle_color: color::RGB,
) {
  let triangle_top = Vector2f::create((width / 2) as f32, height as f32);
  let triangle_bottom_right = Vector2f::create(width as f32, 0.0f32);

  let triangle_right_middle_point = (triangle_bottom_right - triangle_top).mul_scalar(0.5);
  let triangle_left_middle_point = triangle_right_middle_point.rotate(std::f32::consts::PI * 0.5);

  let triangle_right_norm = triangle_right_middle_point
    .rotate(std::f32::consts::PI * 0.5)
    .normalize();

  let triangle_left_norm = triangle_left_middle_point
    .rotate(-std::f32::consts::PI * 0.5)
    .normalize();

  for row in 0..height {
    for col in 0..width {
      let dir_to_current_postion = {
        let mut current_triangle_side = triangle_left_middle_point;
        let current_position = Vector2f::create(col as f32, row as f32);
        if col >= width / 2 {
          current_triangle_side = triangle_right_middle_point;
        }
        (current_position - current_triangle_side).normalize()
      };

      let how_similar_right = triangle_right_norm.dot_product(dir_to_current_postion);
      let how_similar_left = triangle_left_norm.dot_product(dir_to_current_postion);
      if how_similar_right > 0.0 || how_similar_left > 0.0 {
        buffer_output[(row * width) + col] = triangle_color;
      }
    }
  }
}

pub fn make_circle_pattern(
  buffer_output: &mut [color::RGB],
  width: usize,
  height: usize,
  circle_color: color::RGB,
  optional_radius: Option<f32>,
) {
  let circle_center = Vector2f::create((width / 2) as f32, (height / 2) as f32);
  let radius = {
    let result: f32;
    if optional_radius.is_some() {
      result = optional_radius.unwrap();
    } else {
      let left_most_point = Vector2f::create(width as f32, (height / 2) as f32);
      result = (circle_center - left_most_point).mag();
    }
    result
  };

  for y in 0..height {
    for x in 0..width {
      let current_point = Vector2f::create(x as f32, y as f32);
      let distance = (current_point - circle_center).mag();
      if distance < radius {
        buffer_output[(y * width) + x] = circle_color;
      }
    }
  }
}

pub fn make_one_of_pattern(
  width: usize,
  height: usize,
  forground_color: Option<color::RGB>,
  background_color: Option<color::RGB>,
) -> [(pixel_buffer::PixelBufferRGB, String); 4] {
  let mut checker_pattern =
    pixel_buffer::PixelBufferRGB::create(Some(color::RGB::WHITE), width, height, None);

  let mut random_rainbow_pattern =
    pixel_buffer::PixelBufferRGB::create(Some(color::RGB::WHITE), width, height, None);

  let mut triangle_pattern =
    pixel_buffer::PixelBufferRGB::create(Some(color::RGB::WHITE), width, height, None);

  let mut circle_pattern =
    pixel_buffer::PixelBufferRGB::create(Some(color::RGB::WHITE), width, height, None);

  let mut used_forground_color: color::RGB = color::RGB::BLUE;
  if forground_color.is_some() {
    used_forground_color = forground_color.unwrap();
  }

  let mut used_background_color: color::RGB = color::RGB::RED;
  if background_color.is_some() {
    used_background_color = background_color.unwrap();
  }

  make_checker_pattern(
    checker_pattern.get_buffer_as_slice_mut(),
    width as usize,
    height as usize,
    used_forground_color,
    used_background_color,
  );

  make_random_rainbow_pattern(
    random_rainbow_pattern.get_buffer_as_slice_mut(),
    width as usize,
    height as usize,
  );

  make_triangle_pattern(
    triangle_pattern.get_buffer_as_slice_mut(),
    width as usize,
    height as usize,
    used_forground_color,
  );

  make_circle_pattern(
    circle_pattern.get_buffer_as_slice_mut(),
    width as usize,
    height as usize,
    used_forground_color,
    None,
  );

  [
    (checker_pattern, String::from("checker_pattern.ppm")),
    (random_rainbow_pattern, String::from("random_pattern.ppm")),
    (triangle_pattern, String::from("triangle_pattern.ppm")),
    (circle_pattern, String::from("circle_pattern.ppm")),
  ]
}

use crate::color::RGB;
use std::option::Option;
pub struct PixelBuffer<T> {
  buffer: Vec<T>,
  width: u32,
  height: u32,
  current_row_amount: u32,
  // TODO : implement the ratio variable
  _ratio: [u32; 2],
}

impl<T> PixelBuffer<T>
where
  T: Clone + Copy,
{
  /**
   * @param[in] default_value controls what the default value of the pixel buffer
   */
  pub fn create(
    default_value: Option<T>,
    width_: u32,
    height_: u32,
    _ratio_: Option<[u32; 2]>,
  ) -> PixelBuffer<T> {
    assert_ne!(width_, 0);
    assert_ne!(height_, 0);

    let size_of_buffer = (width_ * height_) as usize;
    if default_value.is_none() {
      return PixelBuffer::<T> {
        buffer: Vec::<T>::with_capacity(size_of_buffer),
        width: width_,
        height: height_,
        current_row_amount: 0,
        _ratio: [0u32, 0u32],
      };
    }

    let mut new_buffer = Vec::<T>::new();
    new_buffer.resize(size_of_buffer, default_value.unwrap());
    PixelBuffer::<T> {
      buffer: new_buffer,
      width: width_,
      height: height_,
      current_row_amount: 0,
      _ratio: [0u32, 0u32],
    }
  }

  // updates internal data of the pixel buffer
  fn update(&mut self) {
    if self.current_row_amount == self.width {
      self.current_row_amount = 0;
      self.height += 1;
    }
  }

  fn calculate_index(&self, x_pos: u32, y_pos: u32) -> usize {
    assert!(y_pos < self.height, "trying to get out of bounds");
    ((self.width * y_pos) + x_pos) as usize
  }

  pub fn add_element(&mut self, element: T) {
    self.current_row_amount += 1u32;
    self.buffer.push(element);
    self.update();
  }

  pub fn get_element_mut(&mut self, x_pos: u32, y_pos: u32) -> &mut T {
    let index = self.calculate_index(x_pos, y_pos);
    &mut self.buffer[index]
  }

  pub fn get_element(&self, x_pos: u32, y_pos: u32) -> T {
    let index = self.calculate_index(x_pos, y_pos);
    self.buffer[index]
  }

  pub fn set_element(&mut self, x_pos: u32, y_pos: u32, value: T) {
    let index = self.calculate_index(x_pos, y_pos);
    self.buffer[index] = value;
  }

  pub fn get_buffer_as_slice(&self) -> &[T] {
    self.buffer.as_slice()
  }

  pub fn get_buffer_as_slice_mut(&mut self) -> &mut [T] {
    self.buffer.as_mut_slice()
  }

  pub fn get_width(&self) -> u32 {
    self.width
  }

  pub fn get_height(&self) -> u32 {
    self.height
  }
}

pub type PixelBufferRGB = PixelBuffer<RGB>;

use crate::color::RGB;
use std::option::Option;
pub struct PixelBuffer<T> {
  buffer: Vec<T>,
  width: usize,
  height: usize,
  current_row_amount: usize,
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
    width_: usize,
    height_: usize,
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

  fn calculate_index(&self, x_pos: usize, y_pos: usize) -> usize {
    assert!(y_pos < self.height, "trying to get out of bounds");
    (self.width * y_pos) + x_pos
  }

  pub fn add_element(&mut self, element: T) {
    self.current_row_amount += 1_usize;
    self.buffer.push(element);
    self.update();
  }

  pub fn get_element_mut(&mut self, x_pos: usize, y_pos: usize) -> &mut T {
    let index = self.calculate_index(x_pos, y_pos);
    &mut self.buffer[index]
  }

  pub fn get_element(&self, x_pos: usize, y_pos: usize) -> T {
    let index = self.calculate_index(x_pos, y_pos);
    self.buffer[index]
  }

  pub fn set_element(&mut self, x_pos: usize, y_pos: usize, value: T) {
    let index = self.calculate_index(x_pos, y_pos);
    self.buffer[index] = value;
  }

  pub fn get_buffer_as_slice(&self) -> &[T] {
    self.buffer.as_slice()
  }

  pub fn get_buffer_as_slice_mut(&mut self) -> &mut [T] {
    self.buffer.as_mut_slice()
  }

  pub fn get_width(&self) -> usize {
    self.width
  }

  pub fn get_height(&self) -> usize {
    self.height
  }
}

pub type PixelBufferRGB = PixelBuffer<RGB>;

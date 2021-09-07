use crate::color::RGB;
pub struct PixelBuffer<T> {
  buffer: Vec<T>,
  width: u32,
  height: u32,
  ratio: [u32; 2],
}

impl<T> PixelBuffer<T> {
  pub fn add_element(element: T) {}
}

type PixelBufferRGB = PixelBuffer<RGB>;

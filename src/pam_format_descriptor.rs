use std::fmt;
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TupleType {
  BLACKANDWHITE,
  GRAYSCALE,
  RGB,
  ALPHA = (1 << 16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PamFormatDescriptor {
  pub width: usize,
  pub height: usize,
  pub tuple_type: TupleType,
  pub depth: u16,
  pub max_val: u8,
}

impl TupleType {
  pub fn has_alpha(&self) -> bool {
    let raw_value = *self as u32;
    raw_value & (1 << 16) > 0
  }
}

impl fmt::Display for TupleType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let raw_value: u32 = (*self) as u32;
    let has_alpha: bool = raw_value & (1 << 16) > 0;
    if has_alpha {
      write!(f, "{:?}_ALPHA", self);
    }
    write!(f, "{:?}", self)
  }
}

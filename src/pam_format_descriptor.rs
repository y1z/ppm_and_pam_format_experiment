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
  pub max_val: u16,
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

impl PamFormatDescriptor {
  pub const fn make_rgb_descriptor(
    width_: usize,
    height_: usize,
    max_val_: Option<u16>,
  ) -> PamFormatDescriptor {
    let final_max_val = match max_val_ {
      Some(x) => x,
      None => u8::MAX as u16,
    };

    PamFormatDescriptor {
      width: width_,
      height: height_,
      tuple_type: TupleType::RGB,
      depth: 3,
      max_val: final_max_val,
    }
  }
}

use std::ops::{Add, Sub};

/// represents 8-bit red green blue color space
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct RGB {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

/// represents 8-bit red green blue color space with alpha
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct RGBA {
  pub rgb: RGB,
  pub alpha: u8,
}

impl RGB {
  pub fn create_copy(color: &RGB) -> RGB {
    RGB::create(color.red, color.green, color.blue)
  }

  pub fn create(red_: u8, green_: u8, blue_: u8) -> RGB {
    RGB {
      red: red_,
      green: green_,
      blue: blue_,
    }
  }

  pub fn new() -> RGB {
    RGB::create(0u8, 0u8, 0u8)
  }

  pub const PURE_RED: RGB = RGB {
    red: u8::MAX,
    green: 0,
    blue: 0,
  };

  pub const PURE_GREEN: RGB = RGB {
    red: 0,
    green: u8::MAX,
    blue: 0,
  };

  pub const PURE_BLUE: RGB = RGB {
    red: 0,
    green: 0,
    blue: u8::MAX,
  };

  pub const RED: RGB = RGB {
    red: u8::MAX - (u8::MAX / 4),
    green: 0,
    blue: 0,
  };

  pub const GREEN: RGB = RGB {
    red: 0,
    green: u8::MAX - (u8::MAX / 4),
    blue: 0,
  };

  pub const BLUE: RGB = RGB {
    red: 0,
    green: 0,
    blue: u8::MAX - (u8::MAX / 4),
  };

  pub const CYAN: RGB = RGB {
    red: 0,
    green: u8::MAX,
    blue: u8::MAX,
  };

  pub const MAGENTA: RGB = RGB {
    red: u8::MAX,
    green: 0,
    blue: u8::MAX,
  };

  pub const YELLOW: RGB = RGB {
    red: u8::MAX,
    green: u8::MAX,
    blue: 0,
  };

  pub const PINK: RGB = RGB {
    red: u8::MAX,
    green: 0,
    blue: 0xAB,
  };

  // purple #8024AB
  pub const PURPLE: RGB = RGB {
    red: 0x80,
    green: 0x24,
    blue: 0xAB,
  };

  pub const WHITE: RGB = RGB {
    red: u8::MAX,
    green: u8::MAX,
    blue: u8::MAX,
  };

  pub const BLACK: RGB = RGB {
    red: 0,
    green: 0,
    blue: 0,
  };
}

impl RGBA {
  pub fn create(red_: u8, green_: u8, blue_: u8, alpha_: u8) -> RGBA {
    RGBA {
      rgb: RGB::create(red_, green_, blue_),
      alpha: alpha_,
    }
  }

  pub fn new() -> RGBA {
    RGBA {
      rgb: RGB::new(),
      alpha: u8::MAX,
    }
  }

  pub const PURE_RED: RGBA = RGBA {
    rgb: RGB::PURE_RED,
    alpha: u8::MAX,
  };

  pub const PURE_GREEN: RGBA = RGBA {
    rgb: RGB::PURE_GREEN,
    alpha: u8::MAX,
  };

  pub const PURE_BLUE: RGBA = RGBA {
    rgb: RGB::PURE_BLUE,
    alpha: u8::MAX,
  };

  pub const RED: RGBA = RGBA {
    rgb: RGB::RED,
    alpha: u8::MAX,
  };

  pub const GREEN: RGBA = RGBA {
    rgb: RGB::GREEN,
    alpha: u8::MAX,
  };

  pub const BLUE: RGBA = RGBA {
    rgb: RGB::BLUE,
    alpha: u8::MAX,
  };

  pub const CYAN: RGBA = RGBA {
    rgb: RGB::CYAN,
    alpha: u8::MAX,
  };

  pub const MAGENTA: RGBA = RGBA {
    rgb: RGB::MAGENTA,
    alpha: u8::MAX,
  };

  pub const YELLOW: RGBA = RGBA {
    rgb: RGB::YELLOW,
    alpha: u8::MAX,
  };

  pub const PINK: RGBA = RGBA {
    rgb: RGB::PINK,
    alpha: u8::MAX,
  };

  pub const PURPLE: RGBA = RGBA {
    rgb: RGB::PURPLE,
    alpha: u8::MAX,
  };

  pub const WHITE: RGBA = RGBA {
    rgb: RGB::WHITE,
    alpha: u8::MAX,
  };

  pub const BLACK: RGBA = RGBA {
    rgb: RGB::BLACK,
    alpha: u8::MAX,
  };
}

impl Add for RGB {
  type Output = RGB;
  fn add(self, other: Self) -> Self::Output {
    RGB {
      red: self.red + other.red,
      green: self.green + other.green,
      blue: self.blue + other.blue,
    }
  }
}

impl Sub for RGB {
  type Output = RGB;
  fn sub(self, other: Self) -> Self::Output {
    RGB {
      red: self.red - other.red,
      green: self.green - other.green,
      blue: self.blue - other.blue,
    }
  }
}

impl Add for RGBA {
  type Output = RGBA;
  fn add(self, other: Self) -> Self::Output {
    RGBA {
      rgb: self.rgb.add(other.rgb),
      alpha: self.alpha + other.alpha,
    }
  }
}

impl Sub for RGBA {
  type Output = RGBA;
  fn sub(self, other: Self) -> Self::Output {
    RGBA {
      rgb: self.rgb.sub(other.rgb),
      alpha: self.alpha - other.alpha,
    }
  }
}

impl From<RGB> for RGBA {
  fn from(color: RGB) -> RGBA {
    RGBA {
      rgb: color,
      alpha: u8::MAX,
    }
  }
}

impl From<RGBA> for RGB {
  fn from(color: RGBA) -> RGB {
    RGB {
      red: color.rgb.red,
      blue: color.rgb.blue,
      green: color.rgb.green,
    }
  }
}

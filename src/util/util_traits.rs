use std::ops::*;

///
/// Trait's
///

/// To indicate that a type 'T' implements basic arithmetic functions aka 'add' 'sub' 'mul' 'div'.
pub trait BasicArithmetic<T>:
  Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy
{
  fn basic_add(&self, other: Self) -> T;
  fn basic_sub(&self, other: Self) -> T;
  fn basic_mul(&self, other: Self) -> T;
  fn basic_div(&self, other: Self) -> T;
}

/// To indicate that a type 'T' implements basic arithmetic functions and the remainder function.
pub trait BasicArithmeticWithRem<T>: BasicArithmetic<T> + Rem<Output = T> {}

/// To indicate that the type implement the "+=" "-=" "*=" and "/=" functions.
pub trait AdvanceArithmetic<T>:
  BasicArithmetic<T> + AddAssign<T> + SubAssign<T> + MulAssign<T> + DivAssign<T>
{
}

/// To indicate that the type implement the "+=" "-=" "*=" and "/=" functions plus remainder add function "%=".
pub trait AdvanceArithmeticWithRem<T>:
  BasicArithmeticWithRem<T> + AddAssign<T> + SubAssign<T> + MulAssign<T> + DivAssign<T> + RemAssign<T>
{
}

/// To indicate that a type implements the remainder operations.
pub trait RemainderArithmetic<T>: Rem<T> + RemAssign<T> + Copy {}

/// To indicate that a type implement all the arithmetic functions
pub trait FullArithmetic<T>: AdvanceArithmetic<T> + RemainderArithmetic<T> {}

/// Used to indicate that your type can use the square root function
pub trait SquareRoot<T, U = T> {
  fn do_sqrt(&self) -> U;
}

/// Used to indicate that you type can do this
/// ```rust
/// let x = 5;
/// let x_squared = x * x
/// ```
pub trait Squared<T, U = T> {
  fn do_squared(&self) -> U;
}
/// Returns the zero value of a type for i32 it 0 for f32 it 0.0_f32
pub trait ZeroValue<T> {
  fn get_zero_value() -> T;
}

pub trait BasicTrigonometryFunctions<T> {
  fn do_sinf32(self) -> f32;
  fn do_cosf32(self) -> f32;

  fn do_sinf64(self) -> f64;
  fn do_cosf64(self) -> f64;

  fn do_sin_self_type(self) -> T;
  fn do_cos_self_type(self) -> T;
}

///
/// Implementation macros
///

/// implementation for default types
macro_rules! IMPL_ZERO_VALUE_INTEGER {
  ($($tt:ty)*) => ($(
    impl ZeroValue<$tt> for $tt {
      fn get_zero_value() -> $tt {
        0
      }
    }
  )*)
}

IMPL_ZERO_VALUE_INTEGER! {i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

macro_rules! IMPL_BASIC_ARITHMETIC{
  ($($tt:ty)*) => ($(
    impl BasicArithmetic<$tt> for $tt {
      fn basic_add(&self, other: Self) -> $tt{
        self.add(other)
      }
      fn basic_sub(&self, other: Self) -> $tt{
        self.sub(other)
      }

      fn basic_mul(&self, other: Self) -> $tt{
        self.mul(other)
      }

      fn basic_div(&self, other: Self) -> $tt{
        self.div(other)
      }
    }
  )*)
}
IMPL_BASIC_ARITHMETIC! {i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 }

macro_rules! IMPL_BASIC_TRIGONOMETRY_FUNCTIONS{
  ($($tt:ty)*) => ($(
    impl BasicTrigonometryFunctions<$tt> for $tt {
      fn do_sinf32(self) -> f32 {
        let res = self;
        (res as f32).sin()
      }

      fn do_cosf32(self) -> f32 {
        let res = self;
        (res as f32).cos()
      }

      fn do_sinf64(self) -> f64 {
        let res = self;
        (res as f64).sin()
      }

      fn do_cosf64(self) -> f64 {
        let res = self;
        (res as f64).cos()
      }

      fn do_sin_self_type(self) -> $tt{
        (self.do_sinf64()) as $tt
      }

      fn do_cos_self_type(self) -> $tt{
        (self.do_cosf64()) as $tt
      }
    }
  )*);
}

IMPL_BASIC_TRIGONOMETRY_FUNCTIONS! {i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 }

///
/// Implementation of traits for primitive types
///
impl SquareRoot<f32, f32> for f32 {
  fn do_sqrt(&self) -> f32 {
    self.sqrt()
  }
}

impl SquareRoot<f64, f64> for f64 {
  fn do_sqrt(&self) -> f64 {
    self.sqrt()
  }
}

impl SquareRoot<i32, i32> for i32 {
  fn do_sqrt(&self) -> i32 {
    ((*self as f32).sqrt()) as i32
  }
}

impl SquareRoot<i64, f64> for i64 {
  fn do_sqrt(&self) -> f64 {
    (*self as f64).sqrt()
  }
}

impl SquareRoot<u32, f32> for u32 {
  fn do_sqrt(&self) -> f32 {
    (*self as f32).sqrt()
  }
}

impl ZeroValue<f32> for f32 {
  fn get_zero_value() -> f32 {
    0.0_f32
  }
}

impl ZeroValue<f64> for f64 {
  fn get_zero_value() -> f64 {
    0.0_f64
  }
}

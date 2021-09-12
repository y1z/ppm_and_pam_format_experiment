use crate::util::util_traits::{
  BasicArithmetic, BasicTrigonometryFunctions, SquareRoot, ZeroValue,
};
use std::ops::*;

/// Represents a 2d mathematical vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2<T> {
  pub x: T,
  pub y: T,
}

///
/// Implementations
///
impl<T> Vector2<T>
where
  T: BasicArithmetic<T> + SquareRoot<T, T> + ZeroValue<T> + BasicTrigonometryFunctions<T>,
{
  pub fn create(_x: T, _y: T) -> Vector2<T> {
    Vector2::<T> { x: _x, y: _y }
  }

  pub fn new() -> Vector2<T> {
    Vector2::<T>::create(T::get_zero_value(), T::get_zero_value())
  }

  pub fn mag_sqr(&self) -> T {
    (self.x * self.x) + (self.y * self.y)
  }

  pub fn mag(&self) -> T {
    self.mag_sqr().do_sqrt()
  }

  pub fn dot_product(&self, other: Self) -> T {
    (self.x * other.x) + (self.y * other.y)
  }

  // multiplies each component by the scalar
  pub fn mul_scalar(&self, scalar: T) -> Vector2<T> {
    Vector2::<T> {
      x: self.x * scalar,
      y: self.y * scalar,
    }
  }

  pub fn normalize(&self) -> Vector2<T> {
    let magnitude = self.mag();
    Vector2::<T> {
      x: self.x / magnitude,
      y: self.y / magnitude,
    }
  }

  pub fn rotate(&self, rotation_amount: T) -> Vector2<T> {
    let new_x =
      (self.x * rotation_amount.do_cos_self_type()) - (self.y * rotation_amount.do_sin_self_type());

    let new_y =
      (self.x * rotation_amount.do_sin_self_type()) + (self.y * rotation_amount.do_cos_self_type());

    Vector2::<T> { x: new_x, y: new_y }
  }
}

impl<T: Add<Output = T>> Add for Vector2<T> {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Vector2::<T> {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    Vector2::<T> {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl<T: Mul<Output = T>> Mul for Vector2<T> {
  type Output = Self;
  fn mul(self, other: Self) -> Self::Output {
    return Vector2::<T> {
      x: self.x * other.x,
      y: self.y * other.y,
    };
  }
}

impl<T: Div<Output = T>> Div for Vector2<T> {
  type Output = Self;
  fn div(self, other: Self) -> Self::Output {
    Vector2::<T> {
      x: self.x / other.x,
      y: self.y / other.y,
    }
  }
}

/**
 * Pre-made types.
 */
pub type Vector2u = Vector2<u32>;
pub type Vector2i = Vector2<i32>;
pub type Vector2f = Vector2<f32>;
pub type Vector2d = Vector2<f64>;

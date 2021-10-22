// Implementation of middle square weyl sequence
// Based on this video : https://youtu.be/VHJUlRiRDCY?t=2045
// Rules for this constants
const RANDOM_CONST: u64 = 0x78E364C1_2679FAB3;
// 1. Has to be on odd number
// 2. Every 4-bytes should have unique nibble with in those bytes, aka no repretied value in each halve
//    no value like 0xff 0xee 0xcc 0xbb 0xaa etc...
// 3. No nibble should equal zero

pub unsafe fn msws32_next() -> u32 {
  static mut X_WEYL32: u64 = 0u64 as u64;
  static mut W_WEYL32: u64 = 0u64 as u64;

  let square = X_WEYL32.wrapping_mul(X_WEYL32);
  W_WEYL32 = W_WEYL32.wrapping_add(RANDOM_CONST);
  X_WEYL32 = X_WEYL32.wrapping_add(W_WEYL32.wrapping_add(square));
  let shifted_x = X_WEYL32.wrapping_shl(32) as u32 | X_WEYL32.wrapping_shr(32) as u32;
  return shifted_x;
}

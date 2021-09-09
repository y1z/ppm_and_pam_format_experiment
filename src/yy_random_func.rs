// Implementation of middle square weyl sequence
// Based on this video : https://youtu.be/VHJUlRiRDCY?t=2045
// Rules for this constants
const RANDOM_CONST: u64 = 0x78E364C1_2679FAB3 as u64;
// 1. Has to be on odd number
// 2. Every 4-bytes should have unique nibble with in those bytes, aka no repretied value in each halve
//    no value like 0xff 0xee 0xcc 0xbb 0xaa etc...
// 3. No nibble should equal zero

pub unsafe fn msws32_next() -> u32 {
  static mut X_WEYL32: u64 = 0u64;
  static mut W_WEYL32: u64 = 0u64;

  let square = X_WEYL32 * X_WEYL32;
  let random_const = RANDOM_CONST;
  W_WEYL32 = W_WEYL32 + random_const;
  X_WEYL32 = square + W_WEYL32;
  return ((X_WEYL32 >> 32) | (X_WEYL32 << 32)) as u32;
}

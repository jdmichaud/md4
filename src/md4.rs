#![allow(dead_code)]
#![allow(non_snake_case)]

use std::num::Wrapping;

const A: u32 = 0x67452301;
const B: u32 = 0xefcdab89;
const C: u32 = 0x98badcfe;
const D: u32 = 0x10325476;

/*
 * size is the size of the message in bytes
 * returns padding words
 */
fn pad(size: u64) -> Vec<u8> {
  let last_word = size % 64u64; // 512 / 8 = 64
  let nb_of_empty_word = if last_word < 56u64 { // 448 / 8 = 56
    55u64 - last_word
  } else {
    63u64 - (last_word - 56u64)
  };
  println!("nb_of_empty_word:{}", nb_of_empty_word);
  let mut vec: Vec<u8> = Vec::new();
  vec.push(0x80u8);
  for _ in 0..nb_of_empty_word {
    vec.push(0x00u8);
  }
  return vec;
}

/*
 * Convert a double word into a vector of bytes
 */
fn append_size(dword: u64) -> Vec<u8> {
  let b1 = (dword       & 0x00000000000000FF) as u8;
  let b2 = (dword >> 8  & 0x00000000000000FF) as u8;
  let b3 = (dword >> 16 & 0x00000000000000FF) as u8;
  let b4 = (dword >> 24 & 0x00000000000000FF) as u8;
  let b5 = (dword >> 32 & 0x00000000000000FF) as u8;
  let b6 = (dword >> 40 & 0x00000000000000FF) as u8;
  let b7 = (dword >> 48 & 0x00000000000000FF) as u8;
  let b8 = (dword >> 56 & 0x00000000000000FF) as u8;
  let mut vec = vec![0;8];
  // Push low order first
  vec[0] = b4;
  vec[1] = b3;
  vec[2] = b2;
  vec[3] = b1;
  vec[4] = b8;
  vec[5] = b7;
  vec[6] = b6;
  vec[7] = b5;
  return vec;
}

// if X then Y else Z
fn F(X: u32, Y: u32, Z: u32) -> u32 { (X & Y) | (!X & Z) }
// Majority
fn G(X: u32, Y: u32, Z: u32) -> u32 { (X & Y) | (X & Z) | (Y & Z) }
// Only one
fn H(X: u32, Y: u32, Z: u32) -> u32 { X ^ Y ^ Z }

fn T1(a: u32, b: u32, c: u32, d: u32, k: usize, s: u8, word: u16) -> u32 {
  (Wrapping(a) + Wrapping(F(b, c, d)) + Wrapping((word & (1 << k)) as u32)).0 << s
}

fn T2(a: u32, b: u32, c: u32, d: u32, k: usize, s: u8, word: u16) -> u32 {
  (Wrapping(a) + Wrapping(G(b, c, d)) + Wrapping((word & (1 << k)) as u32) + Wrapping(0x5A827999)).0 << s
}

fn T3(a: u32, b: u32, c: u32, d: u32, k: usize, s: u8, word: u16) -> u32 {
  (Wrapping(a) + Wrapping(H(b, c, d)) + Wrapping((word & (1 << k)) as u32) + Wrapping(0x6ED9EBA1)).0 << s
}

fn round1(mut a: u32, mut b: u32, mut c: u32, mut d: u32, word: u16) -> (u32, u32, u32, u32) {
  a = T1(a, b, c, d,  0,  3, word);
  d = T1(d, a, b, c,  1,  7, word);
  c = T1(c, d, a, b,  2, 11, word);
  b = T1(b, c, d, a,  3, 19, word);
  a = T1(a, b, c, d,  4,  3, word);
  d = T1(d, a, b, c,  5,  7, word);
  c = T1(c, d, a, b,  6, 11, word);
  b = T1(b, c, d, a,  7, 19, word);
  a = T1(a, b, c, d,  8,  3, word);
  d = T1(d, a, b, c,  9,  7, word);
  c = T1(c, d, a, b, 10, 11, word);
  b = T1(b, c, d, a, 11, 19, word);
  a = T1(a, b, c, d, 12,  3, word);
  d = T1(d, a, b, c, 13,  7, word);
  c = T1(c, d, a, b, 14, 11, word);
  b = T1(b, c, d, a, 15, 19, word);
  return (a, b, c, d);
}

fn round2(mut a: u32, mut b: u32, mut c: u32, mut d: u32, word: u16) -> (u32, u32, u32, u32) {
  a = T2(a, b, c, d,  0,  3, word);
  d = T2(d, a, b, c,  4,  5, word);
  c = T2(c, d, a, b,  8,  9, word);
  b = T2(b, c, d, a, 12, 13, word);
  a = T2(a, b, c, d,  1,  3, word);
  d = T2(d, a, b, c,  5,  5, word);
  c = T2(c, d, a, b,  9,  9, word);
  b = T2(b, c, d, a, 13, 13, word);
  a = T2(a, b, c, d,  2,  3, word);
  d = T2(d, a, b, c,  6,  5, word);
  c = T2(c, d, a, b, 10,  9, word);
  b = T2(b, c, d, a, 14, 13, word);
  a = T2(a, b, c, d,  3,  3, word);
  d = T2(d, a, b, c,  7,  5, word);
  c = T2(c, d, a, b, 11,  9, word);
  b = T2(b, c, d, a, 15, 13, word);
  return (a, b, c, d);
}

fn round3(mut a: u32, mut b: u32, mut c: u32, mut d: u32, word: u16) -> (u32, u32, u32, u32) {
  a = T3(a, b, c, d,  0,  3, word);
  d = T3(d, a, b, c,  8,  9, word);
  c = T3(c, d, a, b,  4, 11, word);
  b = T3(b, c, d, a, 12, 15, word);
  a = T3(a, b, c, d,  2,  3, word);
  d = T3(d, a, b, c, 10,  9, word);
  c = T3(c, d, a, b,  6, 11, word);
  b = T3(b, c, d, a, 14, 15, word);
  a = T3(a, b, c, d,  1,  3, word);
  d = T3(d, a, b, c,  9,  9, word);
  c = T3(c, d, a, b,  5, 11, word);
  b = T3(b, c, d, a, 13, 15, word);
  a = T3(a, b, c, d,  3,  3, word);
  d = T3(d, a, b, c, 11,  9, word);
  c = T3(c, d, a, b,  7, 11, word);
  b = T3(b, c, d, a, 15, 15, word);
  return (a, b, c, d);
}

fn process_word(a: u32, b: u32, c: u32, d: u32, word: u16) -> (u32, u32, u32, u32) {
  let (a, b, c, d) = round1(a, b, c, d, word);
  let (a, b, c, d) = round2(a, b, c, d, word);
  let (a, b, c, d) = round3(a, b, c, d, word);
  return (a, b, c, d);
}

pub fn process_buffer(buffer: Vec<u8>) -> (u32, u32, u32, u32) {
  let mut a = A;
  let mut b = B;
  let mut c = C;
  let mut d = D;
  let half_length = buffer.len() / 2;
  for i in 0..half_length {
    let j = i * 2;
    let mut word: u16 = buffer[j] as u16;
    word <<= 8;
    word |= buffer[j + 1] as u16;
    let (ra, rb, rc, rd) = process_word(a, b, c, d, word);
    a = ra; b = rb; c = rc; d = rd;
  }
  return (a, b, c, d);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn pad_test() {
    let mut result = pad(0);
    assert_eq!(result.len(), 56);
    assert_eq!(result[0], 0x80);
    result = pad(1);
    assert_eq!(result.len(), 55);
    assert_eq!(result[0], 0x80);
    result = pad(55);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], 0x80);
    result = pad(56);
    assert_eq!(result.len(), 64);
    assert_eq!(result[0], 0x80);
    result = pad(57);
    assert_eq!(result.len(), 63);
    assert_eq!(result[0], 0x80);
    result = pad(63);
    assert_eq!(result.len(), 57);
    assert_eq!(result[0], 0x80);
    result = pad(64);
    assert_eq!(result.len(), 56);
    assert_eq!(result[0], 0x80);
    result = pad(65);
    assert_eq!(result.len(), 55);
    assert_eq!(result[0], 0x80);
  }

  #[test]
  fn append_size_test() {
    assert_eq!(append_size(0), vec![0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(append_size(1), vec![0, 0, 0, 1, 0, 0, 0, 0]);
    assert_eq!(append_size(256), vec![0, 0, 1, 0, 0, 0, 0, 0]);
    assert_eq!(append_size(0xDEADBEEFCAFEBABE), vec![0xCA, 0xFE, 0xBA, 0xBE, 0xDE, 0xAD, 0xBE, 0xEF]);
  }

  #[test]
  fn test_F() {
    assert_eq!(F(0b00001111000000000000000000000000,
                 0b00110011000000000000000000000000,
                 0b01010101000000000000000000000000),
                 0b01010011000000000000000000000000);
  }

  #[test]
  fn test_G() {
    assert_eq!(G(0b00001111000000000000000000000000,
                 0b00110011000000000000000000000000,
                 0b01010101000000000000000000000000),
                 0b00010111000000000000000000000000);
  }

  #[test]
  fn test_H() {
    assert_eq!(H(0b00001111000000000000000000000000,
                 0b00110011000000000000000000000000,
                 0b01010101000000000000000000000000),
                 0b01101001000000000000000000000000);
  }

  #[test]
  fn test_T1() {
    assert_eq!(T1(
      0b00001111000000000000000000000000,
      0b00001111000000000000000000000000,
      0b00110011000000000000000000000000,
      0b01010101000000000000000000000000,
      7,
      3, // Shift of three
      0b0000000010000000, // 7th bit
    ), 0b00010000000000000000010000000000);
  }
  // fn T1(a: u32, b: u32, c: u32, d: u32, k: usize, s: u8, word: u16) -> u32 {

  #[test]
  fn test_T2() {
  }

  #[test]
  fn test_T3() {
  }

  #[test]
  fn test_process_buffer() {
    assert_eq!(process_buffer("".to_string().into_bytes()), (0xcfe031d6, 0xe931d16a, 0x59d7b73c, 0x89c0e0c0));
    assert_eq!(process_buffer("a".to_string().into_bytes()), (0x2cb3bde5, 0x3e461de3, 0x05fb245e, 0xfb24dbd6));
    assert_eq!(process_buffer("abc".to_string().into_bytes()), (0x017aa448, 0xd852af21, 0x0ae85fc1, 0x729d7aa6));
    assert_eq!(process_buffer("message digest".to_string().into_bytes()), (0x0a81d913, 0x9fe86454, 0x48061887, 0x014be1c7));
    assert_eq!(process_buffer("abcdefghijklmnopqrstuvwxyz".to_string().into_bytes()), (0x1c30d79e, 0xbbcd8aa5, 0xed63eea8, 0x2da9df41));
    assert_eq!(process_buffer("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string().into_bytes()), (0x8582043f, 0xdb35f241, 0x27e11ce6, 0xf0e453e7));
    assert_eq!(process_buffer("12345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string().into_bytes()), (0x4ddce33b, 0xf2199c38, 0x7b169c3e, 0x05364fcc));
  }
}

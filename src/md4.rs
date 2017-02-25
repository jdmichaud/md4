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
fn pad(size: usize) -> Vec<u8> {
  let last_word = size % 64usize; // 512 / 8 = 64
  let nb_of_empty_word = if last_word < 56usize { // 448 / 8 = 56
    55usize - last_word
  } else {
    63usize - (last_word - 56usize)
  };
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
  vec[0] = b1;
  vec[1] = b2;
  vec[2] = b3;
  vec[3] = b4;
  vec[4] = b5;
  vec[5] = b6;
  vec[6] = b7;
  vec[7] = b8;
  return vec;
}

// if X then Y else Z
fn F(X: u32, Y: u32, Z: u32) -> u32 { (X & Y) | (!X & Z) }
// Majority
fn G(X: u32, Y: u32, Z: u32) -> u32 { (X & Y) | (X & Z) | (Y & Z) }
// Only one
fn H(X: u32, Y: u32, Z: u32) -> u32 { X ^ Y ^ Z }

fn T1(a: u32, b: u32, c: u32, d: u32, word: u32, s: u8) -> u32 {
  ((Wrapping(a) + Wrapping(F(b, c, d)) + Wrapping(word)).0).rotate_left(s as u32)
}

fn T2(a: u32, b: u32, c: u32, d: u32, word: u32, s: u8) -> u32 {
  ((Wrapping(a) + Wrapping(G(b, c, d)) + Wrapping(word) + Wrapping(0x5A827999)).0).rotate_left(s as u32)
}

fn T3(a: u32, b: u32, c: u32, d: u32, word: u32, s: u8) -> u32 {
  ((Wrapping(a) + Wrapping(H(b, c, d)) + Wrapping(word) + Wrapping(0x6ED9EBA1)).0).rotate_left(s as u32)
}

fn round1(mut a: u32, mut b: u32, mut c: u32, mut d: u32, block: &[u32]) -> (u32, u32, u32, u32) {
  a = T1(a, b, c, d,  block[0],  3);
  d = T1(d, a, b, c,  block[1],  7);
  c = T1(c, d, a, b,  block[2], 11);
  b = T1(b, c, d, a,  block[3], 19);
  a = T1(a, b, c, d,  block[4],  3);
  d = T1(d, a, b, c,  block[5],  7);
  c = T1(c, d, a, b,  block[6], 11);
  b = T1(b, c, d, a,  block[7], 19);
  a = T1(a, b, c, d,  block[8],  3);
  d = T1(d, a, b, c,  block[9],  7);
  c = T1(c, d, a, b, block[10], 11);
  b = T1(b, c, d, a, block[11], 19);
  a = T1(a, b, c, d, block[12],  3);
  d = T1(d, a, b, c, block[13],  7);
  c = T1(c, d, a, b, block[14], 11);
  b = T1(b, c, d, a, block[15], 19);
  return (a, b, c, d);
}

fn round2(mut a: u32, mut b: u32, mut c: u32, mut d: u32, block: &[u32]) -> (u32, u32, u32, u32) {
  a = T2(a, b, c, d,  block[0],  3);
  d = T2(d, a, b, c,  block[4],  5);
  c = T2(c, d, a, b,  block[8],  9);
  b = T2(b, c, d, a, block[12], 13);
  a = T2(a, b, c, d,  block[1],  3);
  d = T2(d, a, b, c,  block[5],  5);
  c = T2(c, d, a, b,  block[9],  9);
  b = T2(b, c, d, a, block[13], 13);
  a = T2(a, b, c, d,  block[2],  3);
  d = T2(d, a, b, c,  block[6],  5);
  c = T2(c, d, a, b, block[10],  9);
  b = T2(b, c, d, a, block[14], 13);
  a = T2(a, b, c, d,  block[3],  3);
  d = T2(d, a, b, c,  block[7],  5);
  c = T2(c, d, a, b, block[11],  9);
  b = T2(b, c, d, a, block[15], 13);
  return (a, b, c, d);
}

fn round3(mut a: u32, mut b: u32, mut c: u32, mut d: u32, block: &[u32]) -> (u32, u32, u32, u32) {
  a = T3(a, b, c, d,  block[0],  3);
  d = T3(d, a, b, c,  block[8],  9);
  c = T3(c, d, a, b,  block[4], 11);
  b = T3(b, c, d, a, block[12], 15);
  a = T3(a, b, c, d,  block[2],  3);
  d = T3(d, a, b, c, block[10],  9);
  c = T3(c, d, a, b,  block[6], 11);
  b = T3(b, c, d, a, block[14], 15);
  a = T3(a, b, c, d,  block[1],  3);
  d = T3(d, a, b, c,  block[9],  9);
  c = T3(c, d, a, b,  block[5], 11);
  b = T3(b, c, d, a, block[13], 15);
  a = T3(a, b, c, d,  block[3],  3);
  d = T3(d, a, b, c, block[11],  9);
  c = T3(c, d, a, b,  block[7], 11);
  b = T3(b, c, d, a, block[15], 15);
  return (a, b, c, d);
}

fn process_block(a: u32, b: u32, c: u32, d: u32, block: &[u32]) -> (u32, u32, u32, u32) {
  let (a, b, c, d) = round1(a, b, c, d, block);
  let (a, b, c, d) = round2(a, b, c, d, block);
  let (a, b, c, d) = round3(a, b, c, d, block);
  return (a, b, c, d);
}

pub fn process_buffer(mut buffer: Vec<u8>) -> (u32, u32, u32, u32) {
  let mut a = A;
  let mut b = B;
  let mut c = C;
  let mut d = D;
  let buffer_length = buffer.len();
  buffer.append(&mut pad(buffer_length));
  buffer.append(&mut append_size((buffer_length << 3) as u64));
  // Now buffer size is a multiple of 64 (number of bits multiple of 512)
  let number_of_blocks = buffer.len() >> 6;
  for k in 0..number_of_blocks {
    let mut block = [0u32; 16];
    for i in 0..16 {
      let j = k * 64 + i * 4;
      // MD4 expect the least significant byte first, so to create a u32 from 4 u8,
      // We need to swap them
      block[i] = (buffer[j] as u32) | (buffer[j + 1] as u32) << 8 |
        (buffer[j + 2] as u32) << 16 | (buffer[j + 3] as u32) << 24;
    }
    let (ra, rb, rc, rd) = process_block(a, b, c, d, &block);
    a = (Wrapping(a) + Wrapping(ra)).0;
    b = (Wrapping(b) + Wrapping(rb)).0;
    c = (Wrapping(c) + Wrapping(rc)).0;
    d = (Wrapping(d) + Wrapping(rd)).0;
  }
  return (u32::from_be(a), u32::from_be(b), u32::from_be(c), u32::from_be(d));
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_pad() {
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
    assert_eq!(append_size(1), vec![1, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(append_size(256), vec![0, 1, 0, 0, 0, 0, 0, 0]);
    assert_eq!(append_size(0xDEADBEEFCAFEBABE), vec![0xBE, 0xBA, 0xFE, 0xCA, 0xEF, 0xBE, 0xAD, 0xDE]);
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
    //                          b 0b00001111000000000000000000000000
    //                          c 0b00110011000000000000000000000000
    //                          d 0b01010101000000000000000000000000
    //                 F(b, c, d) 0b01010011000000000000000000000000
    //                          a 0b10011010110100001100010001001100
    //             a + F(b, c, d) 0b11101101110100001100010001001100
    //                       word 0b01101001110100001101010011100010
    //      a + F(b, c, d) + word 0b01010111101000011001100100101110
    // a + F(b, c, d) + word << s 0b10111101000011001100100101110010
    assert_eq!(T1(
      0b10011010110100001100010001001100,
      0b00001111000000000000000000000000,
      0b00110011000000000000000000000000,
      0b01010101000000000000000000000000,
      0b01101001110100001101010011100010,
      3, // Rotary shift of three
    ),0b10111101000011001100100101110010);
  }

  #[test]
  fn test_T2() {
    //                                       b 0b00001111000000000000000000000000
    //                                       c 0b00110011000000000000000000000000
    //                                       d 0b01010101000000000000000000000000
    //                              G(b, c, d) 0b00010111000000000000000000000000
    //                                       a 0b10011010110100001100010001001100
    //                          a + G(b, c, d) 0b10110001110100001100010001001100
    //                                    word 0b01101001110100001101010011100010
    //                   a + G(b, c, d) + word 0b00011011101000011001100100101110
    //                              0x5A827999 0b01011010100000100111100110011001
    //      a + G(b, c, d) + word + 0x5A827999 0b01110110001001000001001011000111
    // a + G(b, c, d) + word + 0x5A827999 << s 0b10110001001000001001011000111011
    assert_eq!(T2(
      0b10011010110100001100010001001100,
      0b00001111000000000000000000000000,
      0b00110011000000000000000000000000,
      0b01010101000000000000000000000000,
      0b01101001110100001101010011100010,
      3, // Rotary shift of three
    ),0b10110001001000001001011000111011);
  }

  #[test]
  fn test_T3() {
    //                                       b 0b00001111000000000000000000000000
    //                                       c 0b00110011000000000000000000000000
    //                                       d 0b01010101000000000000000000000000
    //                              H(b, c, d) 0b01101001000000000000000000000000
    //                                       a 0b10011010110100001100010001001100
    //                          a + H(b, c, d) 0b00000011110100001100010001001100
    //                                    word 0b01101001110100001101010011100010
    //                   a + H(b, c, d) + word 0b01101101101000011001100100101110
    //                              0x6ED9EBA1 0b01101110110110011110101110100001
    //      a + H(b, c, d) + word + 0x6ED9EBA1 0b11011100011110111000010011001111
    // a + H(b, c, d) + word + 0x6ED9EBA1 << s 0b11100011110111000010011001111110
    assert_eq!(T3(
      0b10011010110100001100010001001100,
      0b00001111000000000000000000000000,
      0b00110011000000000000000000000000,
      0b01010101000000000000000000000000,
      0b01101001110100001101010011100010,
      3, // Rotary shift of three
    ),0b11100011110111000010011001111110);
  }

  #[test]
  fn test_process_buffer() {
    assert_eq!((0x31d6cfe0, 0xd16ae931, 0xb73c59d7, 0xe0c089c0), process_buffer("".to_string().into_bytes()));
    assert_eq!((0xbde52cb3, 0x1de33e46, 0x245e05fb, 0xdbd6fb24), process_buffer("a".to_string().into_bytes()));
    assert_eq!((0xa448017a, 0xaf21d852, 0x5fc10ae8, 0x7aa6729d), process_buffer("abc".to_string().into_bytes()));
    assert_eq!((0xd9130a81, 0x64549fe8, 0x18874806, 0xe1c7014b), process_buffer("message digest".to_string().into_bytes()));
    assert_eq!((0xd79e1c30, 0x8aa5bbcd, 0xeea8ed63, 0xdf412da9), process_buffer("abcdefghijklmnopqrstuvwxyz".to_string().into_bytes()));
    assert_eq!((0x043f8582, 0xf241db35, 0x1ce627e1, 0x53e7f0e4), process_buffer("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".to_string().into_bytes()));
    assert_eq!((0xe33b4ddc, 0x9c38f219, 0x9c3e7b16, 0x4fcc0536), process_buffer("12345678901234567890123456789012345678901234567890123456789012345678901234567890".to_string().into_bytes()));
  }
}



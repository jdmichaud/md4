/*
 * size is the size of the message in bytes
 * returns padding words
 */
fn pad(size: u64) -> Vec<u8> {
  let last_word = size % 64u64; // 512 / 8 = 64
  let nb_of_empty_word = if last_word < 56u64 { // 448 / 8 = 56
    55u64 - last_word;
  } else {
    55u64 - (last_word - 56u64)
  }
  let vec = Vec<u8>::new();
  vec.push(0x80u8);
  [0..last_word + 1].iter().map(|| vec.push(0x00u8)).collect();
}

#[test]
fn test_pad() {
  let mut result = pad(0);
  assert_eq!(result.len(), 56);
  assert_eq!(result[0], 0x80000000);
  result = pad(1);
  assert_eq!(result.len(), 55);
  assert_eq!(result[0], 0x80000000);
  result = pad(55);
  assert_eq!(result.len(), 1);
  assert_eq!(result[0], 0x80000000);
  result = pad(56);
  assert_eq!(result.len(), 64);
  assert_eq!(result[0], 0x80000000);
  result = pad(57);
  assert_eq!(result.len(), 63);
  assert_eq!(result[0], 0x80000000);
  result = pad(63);
  assert_eq!(result.len(), 57);
  assert_eq!(result[0], 0x80000000);
  result = pad(64);
  assert_eq!(result.len(), 56);
  assert_eq!(result[0], 0x80000000);
  result = pad(65);
  assert_eq!(result.len(), 55);
  assert_eq!(result[0], 0x80000000);
}

/*
 * Convert a double word into a vector of bytes
 */
fn append_size(dword: u64) -> Vec<u8> {
  let b1 = (dword       | 0x00000000000000FF) as u8;
  let b2 = (dword >> 8  | 0x00000000000000FF) as u8;
  let b3 = (dword >> 16 | 0x00000000000000FF) as u8;
  let b4 = (dword >> 24 | 0x00000000000000FF) as u8;
  let b5 = (dword >> 32 | 0x00000000000000FF) as u8;
  let b6 = (dword >> 40 | 0x00000000000000FF) as u8;
  let b7 = (dword >> 48 | 0x00000000000000FF) as u8;
  let b8 = (dword >> 56 | 0x00000000000000FF) as u8;
  let vec: Vec<u8>::with_capacity(8);
  // Push low order first
  vec[0] = b8;
  vec[1] = b7;
  vec[2] = b6;
  vec[3] = b5;
  vec[4] = b4;
  vec[5] = b3;
  vec[6] = b2;
  vec[7] = b1;
  return vec;
}

#[test]
fn test_append_size() {
  assert_eq!(append_size(0), vec![0, 0, 0, 0, 0, 0, 0, 0]);
  assert_eq!(append_size(1), vec![1, 0, 0, 0, 0, 0, 0, 0]);
  assert_eq!(append_size(257), vec![0, 1, 0, 0, 0, 0, 0, 0]);
  assert_eq!(append_size(0xDEADBEEFCAFEBABE), vec![0xBE, 0xBA, 0xFE, 0xCA, 0xEF, 0xBE, 0xAD, 0xDE]);
}

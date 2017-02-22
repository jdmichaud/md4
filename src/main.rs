extern crate md4;

use md4::process_buffer;

fn main() {
  let buffer: Vec<u8> = vec![1, 2, 3, 4, 5, 7, 8, 9];
  let (a, b, c, d) = process_buffer(buffer);
}
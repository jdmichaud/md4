extern crate hash;

use std::io::{self, Read};
use hash::md4::process_buffer;

fn main() {
  let mut buffer = String::new();
  io::stdin().read_to_string(&mut buffer).unwrap();
  let (a, b, c, d) = process_buffer(buffer.into_bytes());
  println!("{:x}{:x}{:x}{:x}", a, b, c, d);
}

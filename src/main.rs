extern crate hash;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use hash::md4::process_buffer;

fn hashfile(filename: &str) -> (u32, u32, u32, u32) {
  let mut filecontent = Vec::new();
  return match File::open(filename).and_then(|mut file| file.read_to_end(&mut filecontent)) {
    Err(error) => panic!("Error: {}", error),
    Ok(_) => process_buffer(filecontent)
  }
}

fn hashstdin() -> (u32, u32, u32, u32) {
  let mut buffer = String::new();
  io::stdin().read_to_string(&mut buffer).unwrap();
  return process_buffer(buffer.into_bytes());
}

fn main() {
  let (a, b, c, d) = match env::args().nth(1) {
    Some(filename) => hashfile(&filename),
    None => hashstdin(),
  };
  println!("{:x}{:x}{:x}{:x}", a, b, c, d);
}

#![allow(unused)]

use phosphorus::*;

use std::{
  fs::File,
  io::{BufRead, Read},
  path::Path,
};

fn file_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, std::io::Error> {
  let mut v = vec![];
  let mut f = File::open(path)?;
  f.read_to_end(&mut v)?;
  Ok(v)
}

fn main() {
  let string = String::from_utf8(file_bytes("xml/gl.xml").unwrap()).unwrap();
  let body = drop_declaration(&string);
  for elem in XmlIterator::new(body) {
    println!("{:?}", elem);
  }
}

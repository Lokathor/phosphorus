#![allow(unused)]

use phosphorus::*;

use std::{
  fs::File,
  io::{BufRead, Read},
  path::Path,
};

const GL_XML: &str = include_str!("../../xml/gl.xml");

fn file_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, std::io::Error> {
  let mut v = vec![];
  let mut f = File::open(path)?;
  f.read_to_end(&mut v)?;
  Ok(v)
}

fn main() {
  let body = drop_declaration(GL_XML);
  for elem in XmlIterator::new(body) {
    println!("{:?}", elem);
  }
}

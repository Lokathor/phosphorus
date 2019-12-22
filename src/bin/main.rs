#![allow(unused)]

use phosphorus::*;
use XmlElement::*;

fn main() {
  let bytes_vec = std::fs::read("xml/gl.xml").unwrap();
  let string = String::from_utf8(bytes_vec).unwrap();
  let body = drop_declaration(&string);

  let mut in_types = false;
  let mut it = XmlIterator::new(body);
  while let Some(elem) = it.next() {
    match elem {
      StartTag { name: "types", attrs: "" } => in_types = true,
      StartTag { name: "type", attrs } if in_types => {
        'type_walk: loop {
          match it.next().unwrap() {
            EndTag{ name: "type" } => {
              println!();
              println!();
              break 'type_walk;
            }
            Text(text) => print!("{}", text),
            EmptyTag { .. } => (),
            StartTag{ name: "name", .. } => {
              match it.next().unwrap() {
                Text(text) => print!("{}", text),
                other => panic!("unexpected>{:?}", other),
              }
              match it.next().unwrap() {
                EndTag{ name: "name" } => (),
                other => panic!("unexpected>{:?}", other),
              }
            }
            other => panic!("unexpected>{:?}", other),
          }
        }
      }
      _ => (),
    }
  }
}

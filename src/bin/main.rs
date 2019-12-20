#![allow(unused)]

use phosphorus::*;

fn main() {
  let bytes_vec = std::fs::read("xml/gl.xml").unwrap();
  let string = String::from_utf8(bytes_vec).unwrap();
  let body = drop_declaration(&string);
  for elem in XmlIterator::new(body) {
    println!("{:?}", elem);
    match elem {
      XmlElement::StartTag { attrs, .. }
      | XmlElement::EmptyTag { attrs, .. } => {
        for (key, val) in AttributeIterator::new(attrs) {
          println!("==k:{}, v:{}", key, val);
        }
      }
      _ => (),
    }
  }
}

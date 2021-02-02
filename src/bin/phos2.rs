use magnesium::{XmlElement::*, *};
use tinyvec::TinyVec;

type AttrList<'s> = TinyVec<[TagAttribute<'s>; 8]>;

fn main() {
  let args: Vec<String> = std::env::args().skip(1).collect();
  let filename = &args[0];

  if cfg!(debug_assertions) {
    eprintln!("Reading `{}`", filename);
  }
  let gl_xml_string = std::fs::read_to_string(&filename).unwrap();
  let gl_xml_str = if gl_xml_string.chars().nth(0).unwrap() == '\u{feff}' { &gl_xml_string['\u{feff}'.len_utf8()..] } else { &gl_xml_string };

  let iter = ElementIterator::new(&gl_xml_str).filter_map(skip_comments).filter_map(skip_empty_text_elements);

  let mut indent = 0;
  for element in iter {
    match element {
      StartTag { name, attrs } => {
        for _ in 0..indent {
          print!(" ");
        }
        indent += 2;
        let mut attr_list: AttrList = TagAttributeIterator::new(attrs).collect();
        attr_list.sort();
        print!("<{name}", name = name);
        for tag_attr in attr_list.iter() {
          print!(" {key}=\"{val}\"", key = tag_attr.key, val = tag_attr.value);
        }
        println!(">");
      }
      EndTag { name } => {
        indent -= 2;
        for _ in 0..indent {
          print!(" ");
        }
        println!("</{name}>", name = name);
      }
      EmptyTag { name, attrs } => {
        for _ in 0..indent {
          print!(" ");
        }
        let mut attr_list: AttrList = TagAttributeIterator::new(attrs).collect();
        attr_list.sort();
        print!("<{name}", name = name);
        for tag_attr in attr_list.iter() {
          print!(" {key}=\"{val}\"", key = tag_attr.key, val = tag_attr.value);
        }
        println!("/>");
      }
      Text(t) => {
        for line in t.trim().lines() {
          for _ in 0..indent {
            print!(" ");
          }
          println!("{}", line);
        }
      }
      Comment(t) => {
        for _ in 0..indent {
          print!(" ");
        }
        println!("<!-- {} -->", t);
      }
    }
  }
}

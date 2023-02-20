use magnesium::*;
use phosphorus::*;

const XML: &str = include_str!("../gl.xml");

fn main() {
  let mut iter = ElementIterator::new(XML)
    .filter_map(skip_comments)
    .map(trim_text)
    .filter_map(skip_empty_text_elements);
  assert_eq!(iter.next().unwrap().unwrap_start_tag(), ("registry", ""));
  let registry = Registry::from_iter(&mut iter);
  println!("{registry:?}");
}

#![allow(unused_macros)]

use magnesium::{XmlElement::*, *};
use std::{collections::HashMap, fmt::Write};

macro_rules! submodule {
  ($v:vis $name:ident) => {
    mod $name;
    $v use $name::*;
  };
  ($v:vis $name:ident { $($content:tt)* }) => {
    mod $name { $($content)* }
    $v use $name::*;
  };
}

macro_rules! write_str {
  ($($tree:tt)*) => {
    write!($($tree)*).unwrap();
  }
}

macro_rules! writeln_str {
  ($($tree:tt)*) => {
    writeln!($($tree)*).unwrap();
  }
}

fn hashmap_from_attrs(attrs: &str) -> HashMap<String, String> {
  TagAttributeIterator::new(attrs)
    .map(|ta| (ta.key.to_string(), ta.value.to_string()))
    .collect()
}

submodule!(pub registry);
submodule!(pub types);
submodule!(pub groups);

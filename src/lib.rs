#![allow(unused_labels)]
#![allow(clippy::unused_label)]

#[derive(Debug, Clone, Default)]
pub struct Registry {
  pub types: Types,
}

#[derive(Debug, Clone, Default)]
pub struct Types {
  pub types: Vec<GlType>,
}

#[derive(Debug, Clone, Default)]
pub struct GlType {
  pub name: String,
  pub alias: String,
}

/// Drops any whitespace from the start of the string.
///
/// ```
/// use phosphorus::*;
/// assert_eq!(drop_whitespace("foo"), "foo");
/// assert_eq!(drop_whitespace("  bar"), "bar");
/// assert_eq!(drop_whitespace("   "), "");
/// ```
pub fn drop_whitespace(words: &str) -> &str {
  words.trim_start()
}

/// Drops the XML declaration and following whitespace off the front of the
/// string.
///
/// ```
/// use phosphorus::*;
/// assert_eq!(drop_declaration(r#"<?xml version="1.0" encoding="UTF-8"?>
/// <registry>"#), "<registry>");
pub fn drop_declaration(words: &str) -> &str {
  let words = drop_whitespace(words);
  let mut it = words.char_indices().peekable();
  if let Some((_, '<')) = it.next() {
    if let Some((_, '?')) = it.next() {
      'find_declaration_end: loop {
        while it.next().unwrap().1 != '?' {}
        if let Some((i, '>')) = it.peek() {
          return drop_whitespace(&words[(*i) + 1..]);
        } else {
          continue 'find_declaration_end;
        }
      }
    }
  } else {
    panic!(
      "Didn't start with a declaration: {}...",
      words.chars().take(10).collect::<String>()
    );
  }
  unimplemented!()
}

#[derive(Debug, Clone, Copy)]
pub enum XmlElement<'s> {
  StartTag { name: &'s str, attrs: &'s str },
  EndTag { name: &'s str },
  EmptyTag { name: &'s str, attrs: &'s str },
  Text(&'s str),
  CData(&'s str),
}

#[derive(Debug, Clone)]
pub struct XmlIterator<'s> {
  words: &'s str,
}
impl<'s> XmlIterator<'s> {
  pub fn new(words: &'s str) -> Self {
    Self { words: words.trim_start() }
  }
  fn move_to(&mut self, i: usize) {
    if i <= self.words.len() {
      self.words = &self.words[i..].trim_start();
    } else {
      self.words = "";
    }
  }

  /// `self.words` should point to immediately after the `<![CDATA[` opener.
  fn gather_cdata(&mut self) -> Option<XmlElement<'s>> {
    let mut it = self.words.char_indices();
    loop {
      match it.next()? {
        (end_i, ']') => match it.next()? {
          (_, ']') => match it.next()? {
            (gt, '>') => {
              let cdata = &self.words[..end_i];
              self.move_to(gt+1);
              return Some(XmlElement::CData(cdata));
            },
            _ => continue,
          },
          _ => continue,
        },
        _ => continue,
      }
    }
  }

  /// `self.words` should point to immediately after the `<!--` opener.
  fn skip_over_comment(&mut self) -> Option<()> {
    let mut it = self.words.char_indices();
    loop {
      match it.next()? {
        (_, '-') => match it.next()? {
          (_, '-') => match it.next()? {
            (gt, '>') => {
              self.move_to(gt + 1);
              return Some(());
            },
            _ => continue,
          },
          _ => continue,
        },
        _ => continue,
      }
    }
  }
}
impl<'s> Iterator for XmlIterator<'s> {
  type Item = XmlElement<'s>;

  #[allow(clippy::cognitive_complexity)]
  fn next(&mut self) -> Option<XmlElement<'s>> {
    let mut it = self.words.char_indices().peekable();
    match it.next()? {
      (_, '<') => {
        if it.peek()?.1 == '!' {
          it.next()?;
          match it.next()?.1 {
            '[' => {
              // cdata, like `<![CDATA[ words ]]>`, so skip `CDATA`, then move
              // to the position after `[`, and call the sub-function
              for _ in 0..5 {
                it.next()?;
              }
              self.move_to(it.peek()?.0);
              self.gather_cdata()
            }
            '-' => {
              // inline comment, like `<!-- words -->`, so skip the 2nd dash
              it.next()?;
              self.skip_over_comment()?;
              self.next()
            }
            _ => {
              self.words = "";
              None
            }
          }
        } else {
          let is_end_tag = if it.peek()?.1 == '/' {
            it.next();
            true
          } else {
            false
          };
          let name = 'gather_tag_name: loop {
            match it.next()? {
              (i, ' ') => {
                if is_end_tag {
                  return None;
                }
                break &self.words[1..i];
              }
              (i, '>') => {
                if is_end_tag {
                  let name = &self.words[2..i];
                  self.move_to(i + 1);
                  return Some(XmlElement::EndTag { name });
                } else {
                  let name = &self.words[1..i];
                  let attrs = "";
                  self.move_to(i + 1);
                  return Some(XmlElement::StartTag { name, attrs });
                }
              }
              (f_slash, '/') => loop {
                match it.next()? {
                  (i, '>') => {
                    let name = &self.words[1..f_slash];
                    let attrs = "";
                    self.move_to(i + 1);
                    return Some(XmlElement::EmptyTag { name, attrs });
                  }
                  (_, _) => {
                    continue;
                  }
                }
              },
              (_, _) => continue,
            }
          };
          'gather_tag_attrs: loop {
            match it.next()? {
              (i, '/') => match it.next()? {
                (_j, '>') => {
                  let attrs = &self.words[name.len() + 2..i];
                  match it.next() {
                    Some((k, _)) => {
                      self.move_to(k);
                    }
                    None => {
                      self.words = "";
                    }
                  }
                  return Some(XmlElement::EmptyTag { name, attrs });
                }
                (_, _) => {
                  continue;
                }
              },
              (i, '>') => {
                let attrs = &self.words[name.len() + 2..i];
                match it.next() {
                  Some((k, _)) => {
                    self.move_to(k);
                  }
                  None => {
                    self.words = "";
                  }
                }
                return Some(XmlElement::StartTag { name, attrs });
              }
              (_, _) => continue,
            }
          }
        }
      }
      (_, _) => 'gather_text: loop {
        match it.peek() {
          Some((i, '<')) => {
            let txt = XmlElement::Text(&self.words[..*i]);
            self.move_to(*i);
            return Some(txt);
          }
          Some(_) => {
            it.next();
          }
          None => {
            let txt = XmlElement::Text(self.words);
            self.words = "";
            return Some(txt);
          }
        }
      },
    }
  }
}

#[derive(Debug, Clone)]
pub struct AttributeIterator<'s> {
  words: &'s str,
}
impl<'s> AttributeIterator<'s> {
  pub fn new(words: &'s str) -> Self {
    Self { words: words.trim_start() }
  }
  fn move_to(&mut self, i: usize) {
    self.words = &self.words[i..].trim_start();
  }
}
impl<'s> Iterator for AttributeIterator<'s> {
  type Item = (&'s str, &'s str);

  fn next(&mut self) -> Option<(&'s str, &'s str)> {
    let mut it = self.words.char_indices().peekable();
    let key = loop {
      match it.next()? {
        (i, '=') => {
          break &self.words[..i];
        }
        (_, _) => {
          continue;
        }
      }
    };
    let (v_start, _) = it.next()?;
    let val = loop {
      match it.next()? {
        (i, '"') => {
          let v = &self.words[v_start + 1..i];
          if it.peek().is_none() {
            self.words = "";
          } else {
            self.move_to(i + 1);
          }
          break v;
        }
        (_, _) => {
          continue;
        }
      }
    };
    Some((key, val))
  }
}

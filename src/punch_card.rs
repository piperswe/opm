use std::collections::HashMap;

#[derive(Clone)]
pub struct PunchCard {
  pub characters: [u16; 80]
}

impl Default for PunchCard {
  fn default() -> Self {
    PunchCard {
      characters: [0; 80]
    }
  }
}

#[derive(Clone)]
pub struct Charset {
  to_ascii_map: HashMap<u16, char>,
  to_charset_map: HashMap<char, u16>
}

impl Charset {
  pub fn new(to_ascii_map: HashMap<u16, char>,
    to_charset_map: HashMap<char, u16>) -> Charset {
      Charset {
        to_ascii_map: to_ascii_map,
        to_charset_map: to_charset_map
      }
    }

  pub fn to_ascii(&self, ch: u16) -> Option<char> {
    self.to_ascii_map.get(&ch).cloned()
  }

  pub fn from_ascii(&self, ascii: char) -> Option<u16> {
    self.to_charset_map.get(&ascii).cloned()
  }

  pub fn to_string(&self, card: &PunchCard) -> String {
    card.characters.into_iter()
      .map(|x| self.to_ascii(*x))
      .map(|x| x.unwrap_or(' '))
      .collect()
  }
}
use crate::punch_card::{ Charset, PunchCard };

mod charset {
  use std::collections::HashMap;
  use std::char;
  use std::convert::TryInto;
  use crate::punch_card::Charset;
  use crate::utils::{ bit, bits };

  pub fn get_charset() -> Charset {
    let mut to_ascii = HashMap::new();
    let mut to_charset = HashMap::new();
    let mut add = |charset, ascii| {
      to_ascii.insert(charset, ascii);
      to_charset.insert(ascii, charset);
    };

    // Digits
    for n in 0..=9 {
      char::from_digit(n, 10)
        .map(|digit| { 
          add(bit(n.try_into().unwrap()), digit);
        });
    }

    // Letters
    for x in 1..=9 {
      add(bits(vec![12, x]), ('A' as u8 + x - 1) as char);
    }
    for x in 1..=9 {
      add(bits(vec![11, x]), ('J' as u8 + x - 1) as char);
    }
    for x in 1..=9 {
      add(bits(vec![0, x]), ('S' as u8 + x - 1) as char);
    }

    // Special characters
    add(bit(12), '&');
    add(bits(vec![12, 2, 8]), '¢');
    add(bits(vec![12, 3, 8]), '.');
    add(bits(vec![12, 4, 8]), '<');
    add(bits(vec![12, 5, 8]), '(');
    add(bits(vec![12, 6, 8]), '+');
    add(bits(vec![12, 7, 8]), '|');
    add(bit(11), '-');
    add(bits(vec![11, 2, 8]), '!');
    add(bits(vec![11, 3, 8]), '$');
    add(bits(vec![11, 4, 8]), '*');
    add(bits(vec![11, 5, 8]), ')');
    add(bits(vec![11, 6, 8]), ';');
    add(bits(vec![11, 7, 8]), '¬');
    add(bits(vec![0, 1, 8]), '/');
    // Ambiguous
    // add(bits(vec![0, 2, 8]), ' ');
    add(0, ' ');
    add(bits(vec![0, 3, 8]), ',');
    add(bits(vec![0, 4, 8]), '%');
    add(bits(vec![0, 5, 8]), '_');
    add(bits(vec![0, 6, 8]), '>');
    add(bits(vec![0, 7, 8]), '?');
    add(bits(vec![2, 8]), ':');
    add(bits(vec![3, 8]), '╬');
    add(bits(vec![4, 8]), '@');
    add(bits(vec![5, 8]), '\'');
    add(bits(vec![6, 8]), '=');
    add(bits(vec![7, 8]), '"');

    Charset::new(
      to_ascii,
      to_charset
    )
  }
}

pub struct OPM029 {
  charset: Charset
}

impl OPM029 {
  pub fn new() -> OPM029 {
    OPM029 {
      charset: charset::get_charset()
    }
  }

  pub fn punch(&self, value: char, card: &mut PunchCard, location: usize) {
    self.charset.from_ascii(value).map(|x| {
      card.0[location] = x;
    });
  }

  pub fn punch_str(&self, value: &str, card: &mut PunchCard, location: usize) {
    let mut i = location;
    for ch in value.chars() {
      self.punch(ch, card, i);
      i = i + 1;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::opm_029::OPM029;

  #[test]
  fn punches_correctly() {
    let mut card = PunchCard::default();
    let opm = OPM029::new();
    opm.punch_str("0123ABCD", &mut card, 0);
    assert_eq!(card.0[0], 1 << 0);
    assert_eq!(card.0[1], 1 << 1);
    assert_eq!(card.0[2], 1 << 2);
    assert_eq!(card.0[3], 1 << 3);
    assert_eq!(card.0[4], 1 << 12 | 1 << 1);
    assert_eq!(card.0[5], 1 << 12 | 1 << 2);
    assert_eq!(card.0[6], 1 << 12 | 1 << 3);
    assert_eq!(card.0[7], 1 << 12 | 1 << 4);
    assert!(opm.charset.to_string(card).starts_with("0123ABCD"));
    println!("{}", opm.charset.to_string(card));

    // Tested with a real IBM 029
    opm.punch_str("PIPER MCCORKLE", &mut card, 0);
    assert_eq!(card.0[0], 1 << 11 | 1 << 7);
    assert_eq!(card.0[1], 1 << 12 | 1 << 9);
    assert_eq!(card.0[2], 1 << 11 | 1 << 7);
    assert_eq!(card.0[3], 1 << 12 | 1 << 5);
    assert_eq!(card.0[4], 1 << 11 | 1 << 9);
    assert_eq!(card.0[5], 0);
    assert_eq!(card.0[6], 1 << 11 | 1 << 4);
    assert_eq!(card.0[7], 1 << 12 | 1 << 3);
    assert_eq!(card.0[8], 1 << 12 | 1 << 3);
    assert_eq!(card.0[9], 1 << 11 | 1 << 6);
    assert_eq!(card.0[10], 1 << 11 | 1 << 9);
    assert_eq!(card.0[11], 1 << 11 | 1 << 2);
    assert_eq!(card.0[12], 1 << 11 | 1 << 3);
    assert_eq!(card.0[13], 1 << 12 | 1 << 5);
    assert!(opm.charset.to_string(card).starts_with("PIPER MCCORKLE"));
    println!("{}", opm.charset.to_string(card));
  }
}
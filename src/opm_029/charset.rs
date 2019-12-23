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
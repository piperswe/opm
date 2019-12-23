mod charset;
#[cfg(test)]
mod tests;

use crate::punch_card::{ Charset, PunchCard };

pub struct OPM029 {
  charset: Charset,
  card: Option<PunchCard>,
  location: usize,
}

impl OPM029 {
  pub fn new() -> OPM029 {
    OPM029 {
      charset: charset::get_charset(),
      card: None,
      location: 0,
    }
  }

  pub fn insert_card(&mut self, card: PunchCard) {
    self.card = Some(card);
  }

  pub fn eject_card(&mut self) -> Option<PunchCard> {
    let card = self.card;
    self.card = None;
    card
  }

  pub fn reset(&mut self) {
    self.location = 0;
  }

  pub fn punch(&mut self, value: char) {
    self.card.map(|mut card| {
      self.charset.from_ascii(value).map(|x| {
        card.characters[self.location] = x;
        self.location = self.location + 1;
      });
    });
  }

  pub fn punch_str(&mut self, value: &str) {
    value.chars().for_each(|ch| {
      self.punch(ch);
    });
  }
}

mod charset;
#[cfg(test)]
mod tests;

use crate::punch_card::{ Charset, PunchCard };

pub struct OPM029 {
  charset: Charset,
  card: Option<PunchCard>,
  location: usize,
}

trait MapMut<T> {
  fn map_mut<U, F: FnOnce(T) -> U>(&self, f: F) -> Option<U>;
}

impl<T> MapMut<T> for Option<T>
  where T: Clone {
    fn map_mut<U, F: FnOnce(T) -> U>(&self, f: F) -> Option<U> {
      self.as_ref().cloned().map(f)
    }
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
    std::mem::replace(&mut self.card, None)
  }

  pub fn reset(&mut self) {
    self.location = 0;
  }

  pub fn punch(&mut self, value: char) {
    let result = self.charset.from_ascii(value).and_then(|x| {
      self.card.map_mut(|mut card| {
        card.characters[self.location] = x;
        card
      })
    });
    match result {
      Some(card) => {
        self.card = Some(card);
        self.location += 1;
      }
      None => {}
    }
  }

  pub fn punch_str(&mut self, value: &str) {
    value.chars().for_each(|ch| {
      self.punch(ch);
    });
  }
}

use super::*;
use crate::opm_029::OPM029;

#[test]
fn punches_correctly() {
  let mut opm = OPM029::new();
  opm.insert_card(PunchCard::default());
  opm.punch_str("0123ABCD");
  let card_option = opm.eject_card();
  assert!(card_option.is_some());
  let card = card_option.unwrap();
  assert_eq!(card.characters[0], 1 << 0);
  assert_eq!(card.characters[1], 1 << 1);
  assert_eq!(card.characters[2], 1 << 2);
  assert_eq!(card.characters[3], 1 << 3);
  assert_eq!(card.characters[4], 1 << 12 | 1 << 1);
  assert_eq!(card.characters[5], 1 << 12 | 1 << 2);
  assert_eq!(card.characters[6], 1 << 12 | 1 << 3);
  assert_eq!(card.characters[7], 1 << 12 | 1 << 4);
  assert!(opm.charset.to_string(&card).starts_with("0123ABCD"));
  println!("{}", opm.charset.to_string(&card));

  // Tested with a real IBM 029
  opm.insert_card(card);
  opm.reset();
  opm.punch_str("PIPER MCCORKLE");
  let card_option = opm.eject_card();
  assert!(card_option.is_some());
  let card = card_option.unwrap();
  assert_eq!(card.characters[0], 1 << 11 | 1 << 7);
  assert_eq!(card.characters[1], 1 << 12 | 1 << 9);
  assert_eq!(card.characters[2], 1 << 11 | 1 << 7);
  assert_eq!(card.characters[3], 1 << 12 | 1 << 5);
  assert_eq!(card.characters[4], 1 << 11 | 1 << 9);
  assert_eq!(card.characters[5], 0);
  assert_eq!(card.characters[6], 1 << 11 | 1 << 4);
  assert_eq!(card.characters[7], 1 << 12 | 1 << 3);
  assert_eq!(card.characters[8], 1 << 12 | 1 << 3);
  assert_eq!(card.characters[9], 1 << 11 | 1 << 6);
  assert_eq!(card.characters[10], 1 << 11 | 1 << 9);
  assert_eq!(card.characters[11], 1 << 11 | 1 << 2);
  assert_eq!(card.characters[12], 1 << 11 | 1 << 3);
  assert_eq!(card.characters[13], 1 << 12 | 1 << 5);
  assert!(opm.charset.to_string(&card).starts_with("PIPER MCCORKLE"));
  println!("{}", opm.charset.to_string(&card));
}
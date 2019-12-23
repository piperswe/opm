pub fn bit(idx: u8) -> u16 {
  1 << idx
}

pub fn bits(idxs: Vec<u8>) -> u16 {
  idxs.iter().map(|x| bit(*x)).fold(0, |a, b| a | b)
}
mod opm_029;
mod utils;
mod punch_card;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let mut card = punch_card::PunchCard::default();
    let opm = opm_029::OPM029::new();
    opm.punch_str("HELLO WORLD", &mut card, 0);
    alert(&format!("{}", card.0[0]));
}

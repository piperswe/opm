mod opm_029;
mod utils;
mod punch_card;
mod app;

#[macro_use]
extern crate stdweb;

use stdweb::js_export;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[js_export]
pub fn opm_gui() {
    yew::start_app::<app::App>()
}

pub fn greet() {
    let mut opm = opm_029::OPM029::new();
    opm.insert_card(punch_card::PunchCard::default());
    opm.punch_str("HELLO WORLD");
    let card = opm.eject_card().unwrap();
    let result = &format!("{}", card.characters[0]);
    js! {
        alert(@{result});
    }
}

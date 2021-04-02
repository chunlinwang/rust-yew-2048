// use std::io;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::component::game_panel::GamePanel;

mod component;
mod game;
mod state;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());

    let document = yew::utils::document();

    let div = document.query_selector("#app").unwrap().unwrap();

    App::<GamePanel>::new().mount(div);
}
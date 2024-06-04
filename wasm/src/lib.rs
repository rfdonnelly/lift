use wasm_bindgen::prelude::*;

use lift::{Set, Distribution};

#[wasm_bindgen]
pub fn get_sets(min: u32, max: u32, sets: u32) -> String {
    let sets = lift::get_sets(min, max, sets, Distribution::Sin);
    format_sets(min, &sets)
}

fn format_sets(base: u32, sets: &[Set]) -> String {
    sets
        .iter()
        .map(|set| format!("{:>7} {:?}", set, lift::get_plates(set.weight - base)))
        .collect::<Vec<String>>()
        .join("\n")
}

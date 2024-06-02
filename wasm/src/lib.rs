use wasm_bindgen::prelude::*;

use lift::{Distribution, SuperSet};

#[wasm_bindgen]
pub fn supersets(min: u32, max: u32, sets: u32) -> String {
    let sets = lift::supersets(min, max, sets, Distribution::Sin);
    format_sets(min, &sets)
}

fn format_sets(base: u32, sets: &[SuperSet]) -> String {
    sets.iter()
        .map(|set| format!("{:>7} {:?}", set, lift::plates(set.weight - base)))
        .collect::<Vec<String>>()
        .join("\n")
}

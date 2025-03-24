use std::{collections::HashMap, u16};

mod curve;
mod eval;
mod gray_code;
mod normal_forms;
mod operators;
mod sat;
mod sets;
mod truth_table;

// TODO Time and space complexity for each exercice

fn main() {
    let x = u16::MAX;
    let y = u16::MAX;
    println!("map({}, {}) = {}", x, y, curve::map(x, y));
}

use logic_kit::{BoolTree, Tree};
use std::collections::HashMap;

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
    let tree = BoolTree::try_from("101A|&!^");
    println!("{:?}", tree);

    let tree = tree.unwrap();

    let mut state = HashMap::<char, bool>::new();

    state.insert('A', true);
    // state.insert('B', true);

    println!("{}", tree);
}

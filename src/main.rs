use std::collections::HashMap;

use ready_set_boole::BooleanTree;

mod eval;
mod gray_code;
mod operators;

// TODO Time and space complexity for each exercice

fn main() {
    let tree = BooleanTree::try_from("AB|");
    println!("{:?}", tree);

    let tree = tree.unwrap();

    let mut state = HashMap::<char, bool>::new();

    state.insert('A', true);
    // state.insert('B', true);

    println!("{:?}", tree.evaluate(Some(&state)));
}

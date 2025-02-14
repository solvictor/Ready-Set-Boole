use std::collections::{HashMap, HashSet};

use crate::Tree;

pub fn eval_formula(formula: &str) -> bool {
    Tree::try_from(formula.to_uppercase().as_str())
        .expect(&format!("Invalid formula '{}'", formula))
        .evaluate(None)
        .expect(&format!("Invalid formula '{}'", formula))
}

fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    // TODO make hashset that satisfies Valid trait
    let tree: Tree<HashSet<i32>> = Tree::try_from(formula.to_uppercase().as_str())
        .expect(&format!("Invalid formula '{}'", formula));
    let state: HashMap<char, HashSet<i32>> = sets
        .iter()
        .enumerate()
        .map(|(i, set)| ((65 + i as u8) as char, set.clone().into_iter().collect()))
        .collect();
    println!("{:?}", state);
    // tree.evaluate(state)
    // TODO Make tree of any kind (bool or set)
    todo!()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_eval_subject() {
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("11>"), true);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("1011||="), true);
    }

    #[test]
    #[should_panic]
    fn test_eval_panic_operand() {
        eval_formula("10");
        eval_formula("10&0");
        eval_formula("1!0&0");
    }

    #[test]
    #[should_panic]
    fn test_eval_panic_operator() {
        eval_formula("&");
        eval_formula("11001&||>!");
    }

    #[test]
    fn test_eval_set_subject() {
        [
            ("AB&", vec![vec![0, 1, 2], vec![0, 3, 4]], vec![0]),
            (
                "AB|",
                vec![vec![0, 1, 2], vec![3, 4, 5]],
                vec![0, 1, 2, 3, 4, 5],
            ),
            ("A!", vec![vec![0, 1, 2]], vec![]),
        ]
        .iter()
        .for_each(|(formula, sets, res)| {
            assert_eq!(eval_set(formula, sets.clone()), res.clone());
        });
    }
}

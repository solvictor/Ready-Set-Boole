use crate::logic::{BoolTree, Set, Tree};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

/*
   With N = length of formula
   Time complexity: O(N)
   Space complexity: O(N)
*/
pub fn eval_formula(formula: &str) -> bool {
    BoolTree::try_from(formula.to_uppercase().as_str())
        .map_err(|e| format!("Invalid formula '{}': {}", formula, e))
        .unwrap()
        .evaluate(None)
        .map_err(|e| format!("Failed to evaluate '{}': {}", formula, e))
        .unwrap()
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    assert!(sets.len() < 27);
    let tree: Tree<Set<i32>> = Tree::try_from(formula.to_uppercase().as_str())
        .map_err(|e| format!("Invalid formula '{}': {}", formula, e))
        .unwrap();

    let universal: Arc<HashSet<i32>> = Arc::new(sets.iter().flatten().cloned().collect());
    let state: HashMap<char, Set<i32>> = sets
        .into_iter()
        .enumerate()
        .map(|(i, set)| {
            let var = (65 + i as u8) as char;
            let val = Set::new(HashSet::from_iter(set), Some(universal.clone()));
            (var, val)
        })
        .collect();

    tree.evaluate(Some(&state))
        .map_err(|e| format!("Failed to evaluate '{}': {}", formula, e))
        .unwrap()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_eval_subject() {
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("11>"));
        assert!(!eval_formula("10="));
        assert!(eval_formula("1011||="));
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
            let result = eval_set(formula, sets.clone());
            assert!(res.len() == result.len());
            assert!(res.iter().all(|e| result.contains(e)));
        });
    }

    #[test]
    fn test_eval_set() {
        [
            ("A", vec![vec![0, 1, 2]], vec![0, 1, 2]),
            ("AB&!", vec![vec![0, 1, 2], vec![0, 3, 4]], vec![1, 2, 3, 4]),
            ("AB|!", vec![vec![0, 1, 2], vec![0, 3, 4]], vec![]),
            ("AB^", vec![vec![0, 1, 2], vec![0, 3, 4]], vec![1, 2, 3, 4]),
            ("AB^!", vec![vec![0, 1, 2], vec![0, 3, 4]], vec![0]),
            ("AB>", vec![vec![0, 1, 2], vec![0, 3, 4]], vec![0, 3, 4]),
            ("AB>!", vec![vec![0, 1, 2], vec![0, 3, 4]], vec![1, 2]),
            ("AB=", vec![vec![0, 1, 2], vec![0, 3, 4]], vec![0]),
            ("AB=!", vec![vec![0, 1, 2], vec![0, 3, 4]], vec![1, 2, 3, 4]),
        ]
        .iter()
        .for_each(|(formula, sets, res)| {
            let result = eval_set(formula, sets.clone());
            assert!(res.len() == result.len());
            assert!(res.iter().all(|e| result.contains(e)));
        });
    }
}

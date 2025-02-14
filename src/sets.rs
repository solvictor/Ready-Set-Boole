use ready_set_boole::Tree;

fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    fn solve(i: usize, set: &Vec<i32>, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if i == set.len() {
            res.push(cur.clone());
            return;
        }
        solve(i + 1, set, cur, res);
        cur.push(set[i]);
        solve(i + 1, set, cur, res);
        cur.pop();
    }
    solve(0, &set, &mut vec![], &mut res);
    res
}

fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let tree = Tree::try_from(formula.to_uppercase().as_str())
        .expect(&format!("Invalid formula '{}'", formula));
    // tree.evaluate(state)
    // TODO Make tree of any kind (bool or set)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powerset() {
        [
            (
                vec![1, 2, 3],
                vec![
                    vec![],
                    vec![1],
                    vec![2],
                    vec![3],
                    vec![1, 2],
                    vec![1, 3],
                    vec![2, 3],
                    vec![1, 2, 3],
                ],
            ),
            (
                vec![1, 2, 3, 4],
                vec![
                    vec![],
                    vec![1],
                    vec![2],
                    vec![3],
                    vec![4],
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 3],
                    vec![2, 4],
                    vec![3, 4],
                    vec![1, 2, 3],
                    vec![1, 2, 4],
                    vec![1, 3, 4],
                    vec![2, 3, 4],
                    vec![1, 2, 3, 4],
                ],
            ),
        ]
        .iter()
        .for_each(|(set, power)| {
            assert!(powerset(set.clone())
                .iter()
                .all(|subset| power.contains(subset)));
        });
    }
}

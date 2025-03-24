/*
   With N = number of elements
   Time complexity: O(2^N)
   Space complexity: O(2^N)
*/
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
            let result = powerset(set.clone());
            assert!(result.len() == power.len());
            assert!(result.iter().all(|subset| power.contains(subset)));
        });
    }
}

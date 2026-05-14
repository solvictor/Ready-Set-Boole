/*
   Time complexity: O(1)
   Space complexity: O(1)

   (x * 65536 + y) / (65536 * 65535 + 65535)
*/
pub fn map(x: u16, y: u16) -> f64 {
    (x as f64 * 65536.0 + y as f64) / 4294967295.0
}

/*
   Time complexity: O(1)
   Space complexity: O(1)
*/
pub fn reverse_map(n: f64) -> (u16, u16) {
    let n = n * 4294967295.0;
    (n.div_euclid(65536.0) as u16, n.rem_euclid(65536.0) as u16)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_map_and_reverse() {
        const LIMIT: u16 = 1000;
        let mut seen = HashSet::with_capacity((LIMIT as usize).pow(2) * 2);

        for x in 0..=LIMIT {
            for y in 0..=LIMIT {
                let mapped = map(x, y);
                assert!(seen.insert(format!("{}", mapped)));
                assert_eq!((x, y), reverse_map(mapped));
            }
        }

        let max = u16::MAX;

        for x in max - LIMIT..=max {
            for y in max - LIMIT..=max {
                let mapped = map(x, y);
                assert!(seen.insert(format!("{}", mapped)));
                assert_eq!((x, y), reverse_map(mapped));
            }
        }
        assert!(!seen.insert(format!("{}", map(42, 42))));
    }
}

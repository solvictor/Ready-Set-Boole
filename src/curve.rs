// (x * 65536 + y) / (65536 * 65535 + 65535)
fn map(x: u16, y: u16) -> f64 {
    (x as f64 * 65536.0 + y as f64) / 4294967295.0
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_map() {
        const LIMIT: u16 = 1000;
        let mut seen = HashSet::<String>::with_capacity((LIMIT as usize).pow(2) * 2);

        for x in 0..=LIMIT {
            for y in 0..=LIMIT {
                let mapped = map(x, y);
                assert!(seen.insert(format!("{}", mapped)));
            }
        }

        let max = std::u16::MAX;

        for x in max - LIMIT..=max {
            for y in max - LIMIT..=max {
                let mapped = map(x, y);
                assert!(seen.insert(format!("{}", mapped)));
            }
        }
        assert!(!seen.insert(format!("{}", map(42, 42))));
    }
}

use crate::BooleanTree;
use std::collections::HashMap;

// TODO check output results
pub fn print_truth_table(formula: &str) {
    let tree = match BooleanTree::try_from(formula.to_uppercase().as_str()) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("Invalid formula '{}': {}", formula, e);
            return;
        }
    };

    let letters: u32 = formula
        .chars()
        .filter_map(|c| c.is_uppercase().then(|| 1 << (c as usize - 65)))
        .fold(0, |acc, i| acc | i);

    let n = letters.count_ones();

    // Header
    println!(
        "{}| = |",
        ('A'..='Z')
            .filter_map(|l| (letters & (1 << (l as u8 - 65)) != 0).then(|| format!("| {} ", l)))
            .collect::<String>()
    );

    println!("{}|---|", "|---".repeat(n as usize));

    // Values
    let mut letter_to_state = HashMap::<char, bool>::new();
    for state in 0u32..1 << n {
        ('A'..='Z')
            .filter(|&l| letters & (1 << (l as u8 - 65)) != 0)
            .enumerate()
            .for_each(|(i, l)| {
                letter_to_state.insert(l, state & (1 << (n - i as u32 - 1)) != 0);
            });
        println!(
            "{}| {} |",
            ('A'..='Z')
                .filter_map(|l| (letters & (1 << (l as u8 - 65)) != 0)
                    .then(|| format!("| {} ", letter_to_state[&l])))
                .collect::<String>(),
            if tree.evaluate(Some(&letter_to_state)).unwrap() {
                '1'
            } else {
                '0'
            }
        );
        letter_to_state.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_truth_table() {
        print_truth_table("AB&C|");
        print_truth_table("ABC&|");
        print_truth_table("ADJ&|");
        print_truth_table("ADJA&|^");
        print_truth_table("1001&|^");
        print_truth_table("1A&");
        print_truth_table("1A^");
    }

    #[test]
    #[serial]
    fn test_truth_table_invalid() {
        print_truth_table("1A");
        print_truth_table("10&B");
        print_truth_table("1!0&0");
    }
}

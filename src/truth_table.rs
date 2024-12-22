use crate::BooleanTree;
use std::collections::HashMap;

pub fn print_truth_table(formula: &str) {
    let tree = match BooleanTree::try_from(formula.to_uppercase().as_str()) {
        Ok(tree) => tree,
        Err(e) => {
            eprintln!("Invalid formula '{}': {}", formula, e);
            return;
        }
    };

    let variables: u32 = formula
        .chars()
        .filter_map(|c| c.is_uppercase().then(|| 1 << (c as usize - 65)))
        .fold(0, |acc, i| acc | i);

    let n = variables.count_ones();

    // Header
    println!(
        "{}| = |",
        ('A'..='Z')
            .filter_map(|l| (variables & (1 << (l as u8 - 65)) != 0).then(|| format!("| {} ", l)))
            .collect::<String>()
    );

    println!("{}|---|", "|---".repeat(n as usize));

    // Values
    for state in 0u32..1 << n {
        let variables_state = ('A'..='Z')
            .filter(|&name| variables & (1 << (name as u8 - 65)) != 0)
            .enumerate()
            .map(|(i, name)| (name, state & (1 << (n - i as u32 - 1)) != 0))
            .collect::<HashMap<char, bool>>();
        println!(
            "{}| {} |",
            ('A'..='Z')
                .filter_map(|name| (variables & (1 << (name as u8 - 65)) != 0)
                    .then(|| format!("| {} ", variables_state[&name] as u8)))
                .collect::<String>(),
            tree.evaluate(Some(&variables_state)).unwrap() as u8
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_truth_table() {
        ["AB&C|", "ADJ&|", "ADJA&|^", "1001&|^", "1A&", "1A^"]
            .iter()
            .for_each(|formula| {
                println!("Formula: '{}'", formula);
                print_truth_table(formula);
            });
    }

    #[test]
    #[serial]
    fn test_truth_table_invalid() {
        ["1A", "10&B", "1!0&0"].iter().for_each(|formula| {
            println!("Formula: '{}'", formula);
            print_truth_table(formula);
        });
    }
}

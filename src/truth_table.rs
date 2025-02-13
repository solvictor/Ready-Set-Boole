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

    let variables = tree.get_variables();

    let n = variables.len();

    // Header
    println!(
        "{}| = |",
        variables
            .iter()
            .map(|name| format!("| {} ", name))
            .collect::<String>(),
    );

    println!("{}|---|", "|---".repeat(n as usize));

    // Values
    for state in 0u32..1 << n {
        let variables_state = variables
            .iter()
            .enumerate()
            .map(|(i, name)| (*name, state & (1 << (n - i - 1)) != 0))
            .collect::<HashMap<char, bool>>();
        println!(
            "{}| {} |",
            variables
                .iter()
                .map(|name| format!("| {} ", variables_state[name] as u8))
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

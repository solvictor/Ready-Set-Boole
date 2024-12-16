use std::{collections::HashMap, io::Cursor};

use crate::eval;

fn is_valid(formula: &str) -> bool {
    let mut count = 0;
    for c in formula.chars() {
        match c {
            ('A'..='Z') | '0' | '1' => count += 1,
            '!' => {
                if count < 1 {
                    return false;
                }
            }
            '&' | '|' | '^' | '>' | '=' => {
                if count < 2 {
                    return false;
                }
                count -= 1;
            }
            _ => return false,
        }
    }
    count == 1
}

pub fn print_truth_table(formula: &str) {
    let formula = formula.to_uppercase();

    if !is_valid(&formula) {
        eprintln!("Invalid formula");
        return;
    }

    let mut letters = HashMap::<char, Vec<usize>>::new();

    for (i, c) in formula.char_indices() {
        if ('A'..='Z').contains(&c) {
            if !letters.contains_key(&c) {
                letters.insert(c, vec![]);
            }
            letters.get_mut(&c).unwrap().push(i);
        }
    }

    if letters.is_empty() {
        println!("| = |");
        println!("|---|");
        println!(
            "| {} |",
            if eval::eval_formula(&formula) {
                '1'
            } else {
                '0'
            }
        );
        return;
    }

    let n = letters.len();

    // Header
    println!(
        "| {} | = |",
        ('A'..='Z')
            .filter(|c| letters.contains_key(&c))
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" | ")
    );

    println!("{}|---|", "|---".repeat(n as usize));

    // Values
    for state in 0..1 << n {
        // let current: String = formula
        //     .chars()
        //     .map(|c| {
        //         println!("c {c}");
        //         match c {
        //             ('A'..='Z') => {
        //                 if state & (1 << (n - (c as u32 - 65) - 1)) != 0 {
        //                     '1'
        //                 } else {
        //                     '0'
        //                 }
        //             }
        //             _ => c,
        //         }
        //     })
        //     .collect();
        let current = "0";
        println!(
            "| {} | {} |",
            ('A'..='Z')
                .filter(|c| letters.contains_key(&c))
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" | "),
            // (0..=26u8)
            //     .filter(|l| { letters & (1 << l) != 0 })
            //     .enumerate()
            //     .map(|(i, _)| if state & (1 << (n - i as u32 - 1)) != 0 {
            //         "1"
            //     } else {
            //         "0"
            //     })
            //     .collect::<Vec<&str>>()
            //     .join(" | "),
            if eval::eval_formula(&current) {
                '1'
            } else {
                '0'
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truth_table() {
        // print_truth_table("ABC&|");
        print_truth_table("ADJ&|");
        // print_truth_table("ADJA&|^");
        // print_truth_table("1001&|^");
    }
}

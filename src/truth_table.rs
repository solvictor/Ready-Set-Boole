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

    let letters: u32 = formula
        .chars()
        .filter_map(|c| c.is_uppercase().then(|| 1 << (c as usize - 65)))
        .fold(0, |acc, i| acc | i);

    if letters == 0 {
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

    let n = letters.count_ones();

    // Header
    println!(
        "| {} | = |",
        (0..=26u8)
            .filter_map(|i| (letters & (1 << i) != 0).then(|| ((i + 65) as char).to_string()))
            .collect::<Vec<String>>()
            .join(" | ")
    );

    println!("{}|---|", "|---".repeat(n as usize));

    // Values
    let mut letter_to_state = [0; 26];
    for state in 0u32..1 << n {
        (0..=26u8)
            .filter(|l| letters & (1 << l) != 0)
            .enumerate()
            .for_each(|(i, l)| {
                letter_to_state[l as usize] = (state & (1 << (n - i as u32 - 1))).count_ones();
            });
        let current: String = formula
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    (letter_to_state[c as usize - 65] as u8 + 48) as char
                } else {
                    c
                }
            })
            .collect();
        println!(
            "| {} | {} |",
            (0..=26u8)
                .filter_map(|l| (letters & (1 << l) != 0).then(|| {
                    if letter_to_state[l as usize] == 1 {
                        "1"
                    } else {
                        "0"
                    }
                }))
                .collect::<Vec<&str>>()
                .join(" | "),
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
        print_truth_table("AB&C|");
        print_truth_table("ABC&|");
        print_truth_table("ADJ&|");
        print_truth_table("ADJA&|^");
        print_truth_table("1001&|^");
    }
}

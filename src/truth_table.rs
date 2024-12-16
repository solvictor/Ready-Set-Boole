use crate::eval;

fn is_valid(formula: &str) -> bool {
    let mut count = 0;
    for c in formula.chars() {
        match c {
            ('A'..='Z') => count += 1,
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
        .filter_map(|c| c.is_alphabetic().then(|| 1 << (c as usize - 65)))
        .sum();

    // Header
    println!(
        "| {} | = |",
        (0..=26u8)
            .filter_map(|i| (letters & (1 << i) != 0).then(|| ((i + 65) as char).to_string()))
            .collect::<Vec<String>>()
            .join(" | ")
    );

    println!("{}|---|", "|---".repeat(letters.count_ones() as usize));

    // Values
    for state in 0..1 << letters.count_ones() {
        let current = "0";
        println!(
            "| {} | {} |",
            (0..=26u8)
                .filter_map(
                    |i| (letters & (1 << i) != 0).then(|| if state & (1 << i) != 0 {
                        // TODO Cook
                        "1"
                    } else {
                        "0"
                    })
                )
                .collect::<Vec<&str>>()
                .join(" | "),
            if eval::eval_formula(current) {
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
    fn test_truth_table() {}
}

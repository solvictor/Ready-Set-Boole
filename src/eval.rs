use std::collections::VecDeque;

// TODO Visualizer

pub fn eval_formula(formula: &str) -> bool {
    let mut stack = VecDeque::<bool>::new();
    for c in formula.chars() {
        if c == '0' {
            stack.push_back(false);
            continue;
        }
        if c == '1' {
            stack.push_back(true);
            continue;
        }
        let q = stack
            .pop_back()
            .expect(&format!("Invalid formula '{}'", formula));
        if c == '!' {
            stack.push_back(!q);
            continue;
        }
        let p = stack
            .pop_back()
            .expect(&format!("Invalid formula '{}'", formula));
        match c {
            '&' => {
                stack.push_back(p & q);
            }
            '|' => {
                stack.push_back(p | q);
            }
            '^' => {
                stack.push_back(p ^ q);
            }
            '>' => {
                stack.push_back(!p | q);
            }
            '=' => {
                stack.push_back(p == q);
            }
            _ => panic!("Invalid formula '{}'", formula),
        }
    }
    if stack.len() != 1 {
        panic!("Invalid formula '{}'", formula);
    }
    stack.pop_front().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_subject() {
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("11>"), true);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("1011||="), true);
    }

    #[test]
    #[should_panic]
    fn test_eval_panic_operand() {
        eval_formula("10");
        eval_formula("10&0");
        eval_formula("1!0&0");
    }

    #[test]
    #[should_panic]
    fn test_eval_panic_operator() {
        eval_formula("&");
        eval_formula("11001&||>!");
    }
}

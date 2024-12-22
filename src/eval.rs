use crate::BooleanTree;

pub fn eval_formula(formula: &str) -> bool {
    BooleanTree::try_from(formula.to_uppercase().as_str())
        .expect(&format!("Invalid formula '{}'", formula))
        .evaluate(None)
        .expect(&format!("Invalid formula '{}'", formula))
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

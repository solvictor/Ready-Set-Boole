use crate::BooleanTree;

fn negation_normal_form(formula: &str) -> String {
    BooleanTree::try_from(formula.to_uppercase().as_str())
        .expect(&format!("Invalid formula '{}'", formula))
        .nnf()
        .rpn_formula()
}

fn conjunctive_normal_form(formula: &str) -> String {
    BooleanTree::try_from(formula.to_uppercase().as_str())
        .expect(&format!("Invalid formula '{}'", formula))
        .cnf()
        .rpn_formula()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nnf_subject() {
        [
            ("AB&!", "A!B!|"),
            ("AB|!", "A!B!&"),
            ("AB>", "A!B|"),
            ("AB=", "AB&A!B!&|"),
            ("AB|C&!", "A!B!&C!|"),
        ]
        .iter()
        .for_each(|&(formula, nnf)| {
            assert_eq!(negation_normal_form(formula), nnf);
        });
    }

    #[test]
    fn test_nnf() {
        [
            ("AB^", "A!B&AB!&|"),
            ("AB^!", "AB!|A!B|&"),
            ("AB>!", "AB!&"),
            ("AB=!", "AB|A!B!|&"),
            ("A!B!|", "A!B!|"),
            ("AB|", "AB|"),
            ("AB|!", "A!B!&"),
            ("AB!!&", "AB&"),
            ("AB!!!&", "AB!&"),
        ]
        .iter()
        .for_each(|&(formula, nnf)| {
            assert_eq!(negation_normal_form(formula), nnf);
        });
    }
}

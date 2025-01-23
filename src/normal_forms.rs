use ready_set_boole::BooleanTree;

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

    #[test] // TODO More checks
    fn test_nnf() {
        [
            ("AB&!", "A!B!|"),
            ("AB|!", "A!B!&"),
            ("AB^", "A!B&AB!&|"),
            ("AB^!", "AB!|A!B|&"),
            ("AB>!", "AB!&"),
            ("AB=!", "AB|A!B!|&"),
            ("AB=", "AB&A!B!&|"),
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

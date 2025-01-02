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

    #[test]
    fn test_nnf() {
        todo!()
    }
}

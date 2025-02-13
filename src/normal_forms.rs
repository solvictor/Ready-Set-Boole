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
        .nnf() // TODO can we do everything in cnf directly ?
        .cnf()
        .unwrap() // Should always be ok thanks to nnf
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

    #[test]
    fn test_cnf_subject() {
        [
            ("AB&!", "A!B!|"),
            ("AB|!", "A!B!&"),
            ("AB|C&", "AB|C&"),
            ("AB|C|D|", "ABCD|||"),
            ("AB&C&D&", "ABCD&&&"),
            ("AB&!C!|", "A!B!C!||"),
            ("AB|!C!&", "A!B!C!&&"),
            ("ABCD&|&", "ABC|BD|&&"),
        ]
        .iter()
        .for_each(|&(formula, cnf)| {
            assert_eq!(conjunctive_normal_form(formula), cnf);
        });
    }

    #[test]
    fn test_cnf() {
        [
            ("A", "A"),
            ("A!", "A!"),
            ("A!!", "A"),
            ("AB&", "AB&"),
            ("ABC|&", "ABC|&"),
            ("AB|CD|&", "AB|CD|&"),
            ("ABC||", "ABC||"),
            ("ABC&|", "AB|AC|&"),
            ("AB&CD&|", "CA|CB|DA|DB|&&&"),
            ("ABC|&!", "A!B!|A!C!|&"),
        ]
        .iter()
        .for_each(|&(formula, cnf)| {
            assert_eq!(conjunctive_normal_form(formula), cnf);
        });
    }
}

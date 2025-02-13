use crate::BooleanTree;

fn sat(formula: &str) -> bool {
    BooleanTree::try_from(formula.to_uppercase().as_str())
        .expect(&format!("Invalid formula '{}'", formula))
        .is_sat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sat_subject() {
        [
            ("AB|", true),
            ("AB&", true),
            ("AA!&", false),
            ("AA^", false),
        ]
        .iter()
        .for_each(|&(formula, is_sat)| {
            assert_eq!(sat(formula), is_sat);
        });
    }

    #[test]
    fn test_sat() {
        [
            ("A", true),
            ("A!", true),
            ("A!!", true),
            ("A!A=", false),
            ("A!A&", false),
            ("A!A|", true),
            ("AB=", true),
            ("AB&C|", true),
            ("AB|C!&", true),
            ("AB&AB&!&", false),
            ("AB&AB&=", true),
            ("ABC&&", true),
            ("A!A&!", true),
            ("A!B!&", true),
            ("A!B=", true),
            ("AA=", true),
            ("AB|!", true),
            ("ABC|&", true),
            ("ABC!|&", true),
            ("AA!&BB!&|", false),
            ("AB&CD&|", true),
        ]
        .iter()
        .for_each(|&(formula, is_sat)| {
            assert_eq!(sat(formula), is_sat);
        });
    }
}

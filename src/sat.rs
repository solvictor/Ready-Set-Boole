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
    /*
    (A | B) & (A | C | D)
    ()

         */

    // TODO
    // #[test]
    // fn test_sat() {
    //     [
    //         ("AB^", "A!B&AB!&|"),
    //         ("AB^!", "AB!|A!B|&"),
    //         ("AB>!", "AB!&"),
    //         ("AB=!", "AB|A!B!|&"),
    //         ("A!B!|", "A!B!|"),
    //         ("AB|", "AB|"),
    //         ("AB|!", "A!B!&"),
    //         ("AB!!&", "AB&"),
    //         ("AB!!!&", "AB!&"),
    //     ]
    //     .iter()
    //     .for_each(|&(formula, nnf)| {
    //         assert_eq!(negation_normal_form(formula), nnf);
    //     });
    // }
}

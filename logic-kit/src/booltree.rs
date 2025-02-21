use std::collections::VecDeque;

use crate::{boxed, Tree};

#[derive(Clone, Debug)]
pub struct BoolTree(Tree<bool>);

impl std::ops::Deref for BoolTree {
    type Target = Tree<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for BoolTree {
    type Error = String;

    fn try_from(formula: &str) -> Result<Self, Self::Error> {
        use Tree::*;

        let mut stack = VecDeque::<Tree<bool>>::new();

        for (i, c) in formula.char_indices() {
            match c {
                '0' | '1' => {
                    stack.push_back(Value(c == '1'));
                    continue;
                }
                'A'..='Z' => {
                    stack.push_back(Variable(c));
                    continue;
                }
                ' ' => continue,
                _ => {}
            }
            let q = stack
                .pop_back()
                .ok_or(format!("Missing operand at index {}", i))?;
            if c == '!' {
                stack.push_back(Not(boxed!(q)));
                continue;
            }
            let p = stack
                .pop_back()
                .ok_or(format!("Missing operand at index {}", i))?;
            stack.push_back(match c {
                '&' => And(boxed!(p), boxed!(q)),
                '|' => Or(boxed!(p), boxed!(q)),
                '^' => Xor(boxed!(p), boxed!(q)),
                '>' => Implication(boxed!(p), boxed!(q)),
                '=' => Equivalence(boxed!(p), boxed!(q)),
                _ => return Err(format!("Invalid character '{}'", c)),
            });
        }
        match stack.len() {
            1 => Ok(BoolTree(stack.pop_front().unwrap())),
            0 => Err("Empty formula".into()),
            2 => Err("Missing operator".into()),
            _ => Err("Missing operators".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_tree_rpn_formula() {
        [
            "AB&C|",
            "ADJ&|",
            "ADJA&|^",
            "1001&|^",
            "1A&",
            "10BA&|^10BC=>!&|",
        ]
        .iter()
        .for_each(|&formula| {
            let tree = BoolTree::try_from(formula).expect("Failed to parse formula");
            assert_eq!(formula, tree.rpn_formula().as_str());
        });
    }
}

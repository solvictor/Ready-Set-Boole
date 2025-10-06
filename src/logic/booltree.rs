use crate::logic::{Rc, Tree};
use crate::rc;
use std::collections::{HashMap, VecDeque};

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

        let mut stack = VecDeque::new();

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
                stack.push_back(Not(rc!(q)));
                continue;
            }
            let p = stack
                .pop_back()
                .ok_or(format!("Missing operand at index {}", i))?;
            stack.push_back(match c {
                '&' => And(rc!(p), rc!(q)),
                '|' => Or(rc!(p), rc!(q)),
                '^' => Xor(rc!(p), rc!(q)),
                '>' => Implication(rc!(p), rc!(q)),
                '=' => Equivalence(rc!(p), rc!(q)),
                _ => return Err(format!("Invalid character '{}'", c)),
            });
        }
        match stack.len() {
            1 => Ok(BoolTree(stack.pop_front().unwrap())),
            0 => Err("Empty formula".to_string()),
            2 => Err("Missing operator".to_string()),
            _ => Err("Missing operators".to_string()),
        }
    }
}

impl Tree<bool> {
    // LGTM
    pub fn rpn_formula(&self) -> String {
        use Tree::*;

        match self {
            Value(true) => '1'.to_string(),
            Value(false) => '0'.to_string(),
            Variable(var) => var.to_string(),
            Not(sub) => sub.rpn_formula() + "!",
            And(left, right) => left.rpn_formula() + &right.rpn_formula() + "&",
            Or(left, right) => left.rpn_formula() + &right.rpn_formula() + "|",
            Xor(left, right) => left.rpn_formula() + &right.rpn_formula() + "^",
            Implication(left, right) => left.rpn_formula() + &right.rpn_formula() + ">",
            Equivalence(left, right) => left.rpn_formula() + &right.rpn_formula() + "=",
        }
    }

    pub fn is_sat(&self) -> bool {
        fn backtrack(
            tree: &Tree<bool>,
            i: usize,
            variables: &Vec<char>,
            state: &mut HashMap<char, bool>,
        ) -> Result<bool, String> {
            if i == variables.len() {
                return tree.evaluate(Some(state));
            }
            if backtrack(tree, i + 1, variables, state)? {
                return Ok(true);
            }
            state.insert(variables[i], false);
            return backtrack(tree, i + 1, variables, state);
        }

        let variables = self.variables();

        let mut variables_state: HashMap<char, bool> =
            variables.iter().map(|x| (*x, true)).collect();

        backtrack(&self, 0, &variables, &mut variables_state)
            .map_err(|e| format!("Failed to evaluate '{}': {}", self.rpn_formula(), e))
            .unwrap()
    }

    pub fn as_char(&self) -> char {
        use Tree::*;

        match self {
            Value(true) => '⊤',
            Value(false) => '⊥',
            Variable(var) => *var,
            Not(_) => '¬',
            And(..) => '∧',
            Or(..) => '∨',
            Xor(..) => '⊕',
            Implication(..) => '⇒',
            Equivalence(..) => '⇔',
        }
    }
}

impl std::fmt::Display for Tree<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Tree::*;

        match self {
            Value(_) | Variable(_) => write!(f, "{}", self.as_char()),
            Not(sub) => write!(f, "{}({})", self.as_char(), sub),
            And(left, right)
            | Or(left, right)
            | Xor(left, right)
            | Implication(left, right)
            | Equivalence(left, right) => {
                write!(f, "({} {} {})", left, self.as_char(), right)
            }
        }
    }
}

impl std::fmt::Display for BoolTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
            let tree = BoolTree::try_from(formula)
                .map_err(|e| format!("Invalid formula '{}': {}", formula, e))
                .unwrap();
            assert_eq!(formula, tree.rpn_formula().as_str());
        });
    }
}

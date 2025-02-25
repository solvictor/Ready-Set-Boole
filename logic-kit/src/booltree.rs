use crate::{boxed, Tree};
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

    // TODO use DPLL or CDCL algorithm
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

        let tree = self.nnf().cnf().unwrap();

        let variables = self.variables();

        let mut variables_state: HashMap<char, bool> =
            variables.iter().map(|x| (*x, true)).collect();

        backtrack(&tree, 0, &variables, &mut variables_state).expect("Sat check failed")
    }

    pub fn as_char(&self) -> char {
        use Tree::*;

        match self {
            Value(true) => '⊤',
            Value(false) => '⊥',
            Variable(var) => *var,
            Not(_) => '¬',
            And(_, _) => '∧',
            Or(_, _) => '∨',
            Xor(_, _) => '⊕',
            Implication(_, _) => '⇒',
            Equivalence(_, _) => '⇔',
        }
    }
}

// TODO Add display for any type that can be displayed but keep bool with as_char
/*
101|& should give

  ^
 / \
1   v
   / \
  0   1

*/
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
            let tree = BoolTree::try_from(formula).expect("Failed to parse formula");
            assert_eq!(formula, tree.rpn_formula().as_str());
        });
    }
}

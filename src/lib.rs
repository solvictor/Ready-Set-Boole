use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

pub mod eval;
pub mod gray_code;
pub mod operators;
pub mod truth_table;

#[derive(Clone, Debug)]
pub enum BooleanTree {
    Value(bool),
    Variable(char),
    Not(Box<BooleanTree>),
    And(Box<BooleanTree>, Box<BooleanTree>),
    Or(Box<BooleanTree>, Box<BooleanTree>),
    Xor(Box<BooleanTree>, Box<BooleanTree>),
    Implication(Box<BooleanTree>, Box<BooleanTree>),
    Equivalence(Box<BooleanTree>, Box<BooleanTree>),
}

impl BooleanTree {
    pub fn evaluate(&self, state: Option<&HashMap<char, bool>>) -> Result<bool, String> {
        Ok(match self {
            BooleanTree::Value(val) => *val,
            BooleanTree::Variable(name) => *state
                .ok_or("Missing state")?
                .get(name)
                .ok_or(&format!("Missing value for {} in state", name))?,
            BooleanTree::Not(p) => !p.evaluate(state)?,
            BooleanTree::And(p, q) => p.evaluate(state)? & q.evaluate(state)?,
            BooleanTree::Or(p, q) => p.evaluate(state)? | q.evaluate(state)?,
            BooleanTree::Xor(p, q) => p.evaluate(state)? ^ q.evaluate(state)?,
            BooleanTree::Implication(p, q) => !p.evaluate(state)? | q.evaluate(state)?,
            BooleanTree::Equivalence(p, q) => p.evaluate(state)? == q.evaluate(state)?,
        })
    }

    // LGTM
    pub fn rpn_formula(&self) -> String {
        match self {
            BooleanTree::Value(val) => if *val { "1" } else { "0" }.to_string(),
            BooleanTree::Variable(var) => var.to_string(),
            BooleanTree::Not(sub) => sub.rpn_formula() + "!",
            BooleanTree::And(left, right) => left.rpn_formula() + &right.rpn_formula() + "&",
            BooleanTree::Or(left, right) => left.rpn_formula() + &right.rpn_formula() + "|",
            BooleanTree::Xor(left, right) => left.rpn_formula() + &right.rpn_formula() + "^",
            BooleanTree::Implication(left, right) => {
                left.rpn_formula() + &right.rpn_formula() + ">"
            }
            BooleanTree::Equivalence(left, right) => {
                left.rpn_formula() + &right.rpn_formula() + "="
            }
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            BooleanTree::Value(val) => {
                if *val {
                    '⊤'
                } else {
                    '⊥'
                }
            }
            BooleanTree::Variable(var) => *var,
            BooleanTree::Not(_) => '¬',
            BooleanTree::And(_, _) => '∧',
            BooleanTree::Or(_, _) => '∨',
            BooleanTree::Xor(_, _) => '⊕',
            BooleanTree::Implication(_, _) => '⇒',
            BooleanTree::Equivalence(_, _) => '⇔',
        }
    }

    // TODO less clone ?
    pub fn nnf(&self) -> BooleanTree {
        use BooleanTree::*;

        match self {
            Value(_) | Variable(_) => self.clone(),
            Not(sub) => match *sub.clone() {
                Value(_) | Variable(_) => self.clone(),
                Not(subb) => subb.nnf(),
                And(left, right) => Or(Box::new(Not(left)), Box::new(Not(right))).nnf(),
                Or(left, right) => And(Box::new(Not(left)), Box::new(Not(right))).nnf(),
                Xor(left, right) => And(
                    Box::new(Or(Box::new(Not(left.clone())), right.clone())),
                    Box::new(Or(left.clone(), Box::new(Not(right.clone())))),
                )
                .nnf(),
                Implication(left, right) => And(left, Box::new(Not(right))).nnf(),
                Equivalence(left, right) => Or(
                    Box::new(And(left.clone(), Box::new(Not(right.clone())))),
                    Box::new(And(right.clone(), Box::new(Not(left.clone())))),
                )
                .nnf(),
            },
            And(left, right) => And(Box::new(left.nnf()), Box::new(right.nnf())),
            Or(left, right) => Or(Box::new(left.nnf()), Box::new(right.nnf())),
            Xor(left, right) => Xor(Box::new(left.nnf()), Box::new(right.nnf())),
            Implication(left, right) => {
                Or(Box::new(Not(left.clone()).nnf()), Box::new(right.nnf()))
            }
            Equivalence(left, right) => And(
                Box::new(Implication(left.clone(), right.clone()).nnf()),
                Box::new(Implication(right.clone(), left.clone()).nnf()),
            ),
        }
    }

    // TODO
    pub fn cnf(&self) -> BooleanTree {
        todo!()
    }
}

// TODO

/*
101|& should give

  ^
 / \
1   v
   / \
  0   1


*/
impl Display for BooleanTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BooleanTree::*;

        match self {
            Value(_) | Variable(_) => write!(f, "{}", self.as_char()),
            Not(inner) => write!(f, "{}({})", self.as_char(), inner),
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

impl TryFrom<&str> for BooleanTree {
    type Error = String;

    // TODO Ignore whitespaces ?
    fn try_from(formula: &str) -> Result<Self, Self::Error> {
        let mut stack = VecDeque::<BooleanTree>::new();

        for (i, c) in formula.char_indices() {
            match c {
                '0' | '1' => {
                    stack.push_back(Self::Value(c == '1'));
                    continue;
                }
                ('A'..='Z') => {
                    stack.push_back(Self::Variable(c));
                    continue;
                }
                _ => {}
            }
            let q = stack
                .pop_back()
                .ok_or(format!("Missing operand at index {}", i))?;
            if c == '!' {
                stack.push_back(Self::Not(Box::new(q)));
                continue;
            }
            let p = stack
                .pop_back()
                .ok_or(format!("Missing operand at index {}", i))?;
            stack.push_back(match c {
                '&' => Self::And(Box::new(p), Box::new(q)),
                '|' => Self::Or(Box::new(p), Box::new(q)),
                '^' => Self::Xor(Box::new(p), Box::new(q)),
                '>' => Self::Implication(Box::new(p), Box::new(q)),
                '=' => Self::Equivalence(Box::new(p), Box::new(q)),
                _ => return Err(format!("Invalid character '{}'", c)),
            });
        }
        match stack.len() {
            1 => Ok(stack.pop_front().unwrap()),
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
            let tree = BooleanTree::try_from(formula).expect("Failed to parse formula");
            assert_eq!(formula, tree.rpn_formula().as_str());
        });
    }
}

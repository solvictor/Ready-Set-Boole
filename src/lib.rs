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
        writeln!(f, "Tree")?;
        fn leftmost(tree: &BooleanTree) -> i32 {
            match tree {
                BooleanTree::Value(_) => 0,
                BooleanTree::Variable(_) => 0,
                BooleanTree::Not(sub) => leftmost(sub),
                BooleanTree::And(left, right) => (2 + leftmost(left)).max(leftmost(right) - 2),
                BooleanTree::Or(left, right) => (2 + leftmost(left)).max(leftmost(right) - 2),
                BooleanTree::Xor(left, right) => (2 + leftmost(left)).max(leftmost(right) - 2),
                BooleanTree::Implication(left, right) => {
                    (2 + leftmost(left)).max(leftmost(right) - 2)
                }
                BooleanTree::Equivalence(left, right) => {
                    (2 + leftmost(left)).max(leftmost(right) - 2)
                }
            }
        }
        let mut pad = leftmost(&self).max(0) as usize;
        writeln!(f, "pad {}", pad)?;
        let mut level = 0;
        let mut deque = VecDeque::<(&BooleanTree, usize, usize)>::new();
        deque.push_back((self, 0, 0));
        while !deque.is_empty() {
            if level != 0 {}
            print!("{}", " ".repeat(pad));
            for _ in 0..deque.len() {
                let (cur, l, r) = deque.pop_front().unwrap();
                print!("{}", cur.as_char());
                match cur {
                    BooleanTree::Value(_) => {}
                    BooleanTree::Variable(_) => {}
                    BooleanTree::Not(sub) => deque.push_back((sub, l, r)),
                    BooleanTree::And(left, right) => {
                        deque.push_back((left, l + 1, r));
                        deque.push_back((right, l, r + 1));
                    }
                    BooleanTree::Or(left, right) => {
                        deque.push_back((left, l + 1, r));
                        deque.push_back((right, l, r + 1));
                    }
                    BooleanTree::Xor(left, right) => {
                        deque.push_back((left, l + 1, r));
                        deque.push_back((right, l, r + 1));
                    }
                    BooleanTree::Implication(left, right) => {
                        deque.push_back((left, l + 1, r));
                        deque.push_back((right, l, r + 1));
                    }
                    BooleanTree::Equivalence(left, right) => {
                        deque.push_back((left, l + 1, r));
                        deque.push_back((right, l, r + 1));
                    }
                }
            }
            println!();
            level += 1;
        }
        Ok(())
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

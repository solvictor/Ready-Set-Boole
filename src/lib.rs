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
    Or(Box<BooleanTree>, Box<BooleanTree>),
    And(Box<BooleanTree>, Box<BooleanTree>),
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
            BooleanTree::Or(p, q) => p.evaluate(state)? | q.evaluate(state)?,
            BooleanTree::And(p, q) => p.evaluate(state)? & q.evaluate(state)?,
            BooleanTree::Xor(p, q) => p.evaluate(state)? ^ q.evaluate(state)?,
            BooleanTree::Implication(p, q) => !p.evaluate(state)? | q.evaluate(state)?,
            BooleanTree::Equivalence(p, q) => p.evaluate(state)? == q.evaluate(state)?,
        })
    }

    // TODO
    pub fn rpn_formula(&self) -> String {
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
        writeln!(f, "Tree")?;
        fn leftmost(tree: &BooleanTree) -> u32 {
            match tree {
                BooleanTree::Value(_) => 0,
                BooleanTree::Variable(_) => 0,
                BooleanTree::Not(sub) => leftmost(sub),
                BooleanTree::Or(left, _) => 2 + leftmost(left),
                BooleanTree::And(left, _) => 2 + leftmost(left),
                BooleanTree::Xor(left, _) => 2 + leftmost(left),
                BooleanTree::Implication(left, _) => 2 + leftmost(left),
                BooleanTree::Equivalence(left, _) => 2 + leftmost(left),
            }
        }
        let mut pad = leftmost(&self);
        writeln!(f, "pad {}", pad)?;
        let mut deque = VecDeque::<&BooleanTree>::new();
        deque.push_back(self);
        while !deque.is_empty() {
            for _ in 0..deque.len() {
                let cur = deque.pop_front().unwrap();
                match cur {
                    BooleanTree::Value(_) => todo!(),
                    BooleanTree::Variable(_) => todo!(),
                    BooleanTree::Not(boolean_tree) => todo!(),
                    BooleanTree::Or(boolean_tree, boolean_tree1) => todo!(),
                    BooleanTree::And(boolean_tree, boolean_tree1) => todo!(),
                    BooleanTree::Xor(boolean_tree, boolean_tree1) => todo!(),
                    BooleanTree::Implication(boolean_tree, boolean_tree1) => todo!(),
                    BooleanTree::Equivalence(boolean_tree, boolean_tree1) => todo!(),
                }
            }
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

use std::collections::{HashMap, VecDeque};

pub mod eval;
pub mod gray_code;
pub mod operators;
pub mod truth_table;

// TODO Visualizer

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
}

impl TryFrom<&str> for BooleanTree {
    type Error = String;

    fn try_from(formula: &str) -> Result<Self, Self::Error> {
        let mut stack = VecDeque::<BooleanTree>::new();

        for c in formula.chars() {
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
                .ok_or(&format!("Invalid formula '{}'", formula))?;
            if c == '!' {
                stack.push_back(Self::Not(Box::new(q)));
                continue;
            }
            let p = stack
                .pop_back()
                .ok_or(&format!("Invalid formula '{}'", formula))?;
            stack.push_back(match c {
                '&' => Self::And(Box::new(p), Box::new(q)),
                '|' => Self::Or(Box::new(p), Box::new(q)),
                '^' => Self::Xor(Box::new(p), Box::new(q)),
                '>' => Self::Implication(Box::new(p), Box::new(q)),
                '=' => Self::Equivalence(Box::new(p), Box::new(q)),
                _ => return Err(format!("Invalid formula '{}'", formula)),
            });
        }
        if stack.len() == 1 {
            Ok(stack.pop_front().unwrap())
        } else {
            Err(format!("Invalid formula '{}'", formula))
        }
    }
}

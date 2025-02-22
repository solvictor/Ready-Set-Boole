use crate::boxed;
use std::collections::{HashMap, VecDeque};
use std::ops::*;

pub trait Evaluable:
    Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + std::cmp::PartialEq
    + Clone
{
}

// Implement it on any type that already can
impl<
        T: Not<Output = Self>
            + BitAnd<Output = Self>
            + BitOr<Output = Self>
            + BitXor<Output = Self>
            + std::cmp::PartialEq
            + Clone,
    > Evaluable for T
{
}

#[derive(Clone, Debug)]
pub enum Tree<T: Evaluable> {
    Value(T),
    Variable(char),
    Not(Box<Tree<T>>),
    And(Box<Tree<T>>, Box<Tree<T>>),
    Or(Box<Tree<T>>, Box<Tree<T>>),
    Xor(Box<Tree<T>>, Box<Tree<T>>),
    Implication(Box<Tree<T>>, Box<Tree<T>>),
    Equivalence(Box<Tree<T>>, Box<Tree<T>>),
}

// TODO Less abstraction ?
// TODO Compile time differentiation between NNF and Unchecked
impl<T: Evaluable> Tree<T> {
    // Get variables of the formula in alphabetical order
    pub fn variables(&self) -> Vec<char> {
        use Tree::*;

        let mut variables = 0u32;

        let mut queue = VecDeque::from([self]);

        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            match cur {
                Variable(var) => {
                    variables |= 1 << (*var as u8 - 65);
                }
                Not(sub) => {
                    queue.push_back(sub);
                }
                And(left, right)
                | Or(left, right)
                | Xor(left, right)
                | Implication(left, right)
                | Equivalence(left, right) => {
                    queue.push_back(left);
                    queue.push_back(right);
                }
                _ => {}
            }
        }

        ('A'..='Z')
            .filter(|&l| variables & (1 << (l as u8 - 65)) != 0)
            .collect()
    }

    // TODO May be invalid if we add a set without associated variable (TODO add a check everywhere and remove Option in evaluate? maybe even remove HashMap and use checked vec)
    pub fn evaluate(&self, state: Option<&HashMap<char, T>>) -> Result<T, String> {
        use Tree::*;

        Ok(match self {
            Value(val) => val.clone(),
            Variable(name) => state
                .ok_or("Missing state")?
                .get(name)
                .ok_or(&format!("Missing value for {} in state", name))?
                .clone(),
            Not(p) => !p.evaluate(state)?,
            And(p, q) => p.evaluate(state)? & q.evaluate(state)?,
            Or(p, q) => p.evaluate(state)? | q.evaluate(state)?,
            Xor(p, q) => p.evaluate(state)? ^ q.evaluate(state)?,
            Implication(p, q) => !p.evaluate(state)? | q.evaluate(state)?,
            Equivalence(p, q) => {
                let a = p.evaluate(state)?;
                let b = q.evaluate(state)?;
                (a.clone() & b.clone()) | (!a & !b)
            }
        })
    }

    // TODO less clone ?
    // Any expression using Exclusive disjunction | Implication | Equivalence have multiple valid representations
    pub fn nnf(&self) -> Self {
        use Tree::*;

        match self {
            Value(_) | Variable(_) => self.clone(),
            Not(sub) => match *sub.clone() {
                Value(_) | Variable(_) => self.clone(),
                Not(subb) => subb.nnf(),
                And(left, right) => Or(boxed!(Not(left)), boxed!(Not(right))).nnf(),
                Or(left, right) => And(boxed!(Not(left)), boxed!(Not(right))).nnf(),
                Xor(left, right) => And(
                    boxed!(Or(left.clone(), boxed!(Not(right.clone())))),
                    boxed!(Or(boxed!(Not(left.clone())), right.clone())),
                )
                .nnf(),
                Implication(left, right) => And(left, boxed!(Not(right))).nnf(),
                Equivalence(left, right) => And(
                    boxed!(Or(left.clone(), right.clone())),
                    boxed!(Or(boxed!(Not(left.clone())), boxed!(Not(right.clone())))),
                )
                .nnf(),
            },
            And(left, right) => And(boxed!(left.nnf()), boxed!(right.nnf())),
            Or(left, right) => Or(boxed!(left.nnf()), boxed!(right.nnf())),
            Xor(left, right) => Or(
                boxed!(And(boxed!(Not(left.clone())), right.clone())),
                boxed!(And(left.clone(), boxed!(Not(right.clone())))),
            )
            .nnf(),
            Implication(left, right) => Or(boxed!(Not(left.clone())), right.clone()).nnf(),
            Equivalence(left, right) => Or(
                boxed!(And(left.clone(), right.clone())),
                boxed!(And(boxed!(Not(left.clone())), boxed!(Not(right.clone())))),
            )
            .nnf(),
        }
    }

    // TODO Revoir
    pub fn cnf(&self) -> Result<Self, String> {
        use Tree::*;

        match self {
            Value(_) | Variable(_) | Not(_) => Ok(self.clone()),
            And(left, right) => Ok(And(boxed!(left.cnf()?), boxed!(right.cnf()?)).flatten_and()),
            Or(left, right) => {
                let left = left.cnf()?;
                let right = right.cnf()?;
                match (&left, &right) {
                    (a, And(b, c)) | (And(b, c), a) => Ok(And(
                        boxed!(Or(boxed!(a.clone()), boxed!((**b).clone())).flatten_or()),
                        boxed!(Or(boxed!(a.clone()), boxed!((**c).clone())).flatten_or()),
                    )
                    .cnf()?),
                    _ => Ok(Or(boxed!(left), boxed!(right)).flatten_or()),
                }
            }
            _ => Err("Unexpected operator in CNF".into()),
        }
    }
}

macro_rules! impl_flatten {
    ($fn_name:ident, $tree_variant:ident) => {
        impl<T: Evaluable> Tree<T> {
            fn $fn_name(&self) -> Self {
                fn collect_clauses<T: Evaluable>(node: &Tree<T>) -> Vec<Tree<T>> {
                    match node {
                        Tree::$tree_variant(left, right) => collect_clauses(left)
                            .into_iter()
                            .chain(collect_clauses(right).into_iter())
                            .collect(),
                        _ => vec![node.clone()],
                    }
                }

                let mut iter = collect_clauses(self).into_iter().rev();
                let last = iter.next().expect(concat!(
                    stringify!($tree_variant),
                    " must have at least one clause"
                ));

                iter.fold(last, |acc, clause| {
                    Tree::$tree_variant(boxed!(clause), boxed!(acc))
                })
            }
        }
    };
}

impl_flatten!(flatten_and, And);
impl_flatten!(flatten_or, Or);

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

impl<T: Evaluable> TryFrom<&str> for Tree<T> {
    type Error = String;

    fn try_from(formula: &str) -> Result<Self, Self::Error> {
        let mut stack = VecDeque::<Tree<T>>::new();

        for (i, c) in formula.char_indices() {
            match c {
                'A'..='Z' => {
                    stack.push_back(Self::Variable(c));
                    continue;
                }
                ' ' => continue,
                _ => {}
            }
            let q = stack
                .pop_back()
                .ok_or(format!("Missing operand at index {}", i))?;
            if c == '!' {
                stack.push_back(Self::Not(boxed!(q)));
                continue;
            }
            let p = stack
                .pop_back()
                .ok_or(format!("Missing operand at index {}", i))?;
            stack.push_back(match c {
                '&' => Self::And(boxed!(p), boxed!(q)),
                '|' => Self::Or(boxed!(p), boxed!(q)),
                '^' => Self::Xor(boxed!(p), boxed!(q)),
                '>' => Self::Implication(boxed!(p), boxed!(q)),
                '=' => Self::Equivalence(boxed!(p), boxed!(q)),
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

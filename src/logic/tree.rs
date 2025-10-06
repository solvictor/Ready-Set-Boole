use crate::logic::Rc;
use crate::rc;
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
    Not(Rc<Tree<T>>),
    And(Rc<Tree<T>>, Rc<Tree<T>>),
    Or(Rc<Tree<T>>, Rc<Tree<T>>),
    Xor(Rc<Tree<T>>, Rc<Tree<T>>),
    Implication(Rc<Tree<T>>, Rc<Tree<T>>),
    Equivalence(Rc<Tree<T>>, Rc<Tree<T>>),
}

impl<T: Evaluable> Tree<T> {
    // Get variables of the formula in alphabetical order
    pub fn variables(&self) -> Vec<char> {
        use Tree::*;

        let mut variables = 0;

        let mut queue = VecDeque::from([self]);

        while let Some(cur) = queue.pop_front() {
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

    pub fn evaluate(&self, state: Option<&HashMap<char, T>>) -> Result<T, String> {
        fn eval<T: Evaluable>(cur: &Tree<T>, state: Option<&HashMap<char, T>>) -> T {
            use Tree::*;

            match cur {
                Value(val) => val.clone(),
                Variable(name) => state.unwrap().get(name).unwrap().clone(),
                Not(p) => !eval(p, state),
                And(p, q) => eval(p, state) & eval(q, state),
                Or(p, q) => eval(p, state) | eval(q, state),
                Xor(p, q) => eval(p, state) ^ eval(q, state),
                Implication(p, q) => !eval(p, state) | eval(q, state),
                Equivalence(p, q) => {
                    let a = eval(p, state);
                    let b = eval(q, state);
                    (a.clone() & b.clone()) | (!a & !b)
                }
            }
        }

        let variables = self.variables();
        if !variables.is_empty() && state.is_none_or(|s| s.is_empty()) {
            Err("Missing state".to_string())
        } else if state.is_some_and(|s| {
            s.keys().len() != variables.len() || s.keys().any(|c| !variables.contains(c))
        }) {
            Err("State is not matching variables".to_string())
        } else {
            Ok(eval(self, state))
        }
    }

    // Any expression using Exclusive disjunction | Implication | Equivalence have multiple valid representations
    pub fn nnf(&self) -> Self {
        use Tree::*;

        match self {
            Value(_) | Variable(_) => self.clone(),
            Not(sub) => match sub.as_ref() {
                Value(_) | Variable(_) => self.clone(),
                Not(subb) => subb.nnf(),
                And(left, right) => Or(rc!(Not(left.clone()).nnf()), rc!(Not(right.clone()).nnf())),
                Or(left, right) => And(rc!(Not(left.clone()).nnf()), rc!(Not(right.clone()).nnf())),
                Xor(left, right) => And(
                    rc!(Or(rc!(left.nnf()), rc!(Not(right.clone()).nnf()))),
                    rc!(Or(rc!(Not(left.clone()).nnf()), rc!(right.nnf()))),
                ),
                Implication(left, right) => And(rc!(left.nnf()), rc!(Not(right.clone()).nnf())),
                Equivalence(left, right) => And(
                    rc!(Or(rc!(left.nnf()), rc!(right.nnf()))),
                    rc!(Or(
                        rc!(Not(left.clone()).nnf()),
                        rc!(Not(right.clone()).nnf())
                    )),
                ),
            },
            And(left, right) => And(rc!(left.nnf()), rc!(right.nnf())),
            Or(left, right) => Or(rc!(left.nnf()), rc!(right.nnf())),
            Xor(left, right) => Or(
                rc!(And(rc!(Not(left.clone()).nnf()), rc!(right.nnf()))),
                rc!(And(rc!(left.nnf()), rc!(Not(right.clone()).nnf()))),
            ),
            Implication(left, right) => Or(rc!(Not(left.clone()).nnf()), rc!(right.nnf())),
            Equivalence(left, right) => Or(
                rc!(And(rc!(left.nnf()), rc!(right.nnf()))),
                rc!(And(
                    rc!(Not(left.clone()).nnf()),
                    rc!(Not(right.clone()).nnf())
                )),
            ),
        }
    }

    // TODO less clone ? and real O(N)
    // Implicit conversion to nnf
    pub fn cnf(&self) -> Self {
        fn cnf<T: Evaluable>(cur: &Tree<T>) -> Tree<T> {
            use Tree::*;

            match cur {
                Value(_) | Variable(_) | Not(_) => cur.clone(),
                And(left, right) => And(rc!(cnf(left)), rc!(cnf(right))).flatten_and(),
                Or(left, right) => {
                    let left = cnf(left);
                    let right = cnf(right);
                    match (&left, &right) {
                        (a, And(b, c)) | (And(b, c), a) => cnf(&And(
                            rc!(Or(rc!(a.clone()), rc!((**b).clone())).flatten_or()),
                            rc!(Or(rc!(a.clone()), rc!((**c).clone())).flatten_or()),
                        )),
                        _ => Or(rc!(left), rc!(right)).flatten_or(),
                    }
                }
                _ => unreachable!(),
            }
        }
        cnf(&self.nnf())
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
                    Tree::$tree_variant(rc!(clause), rc!(acc))
                })
            }
        }
    };
}

impl_flatten!(flatten_and, And);
impl_flatten!(flatten_or, Or);

impl<T: Evaluable> TryFrom<&str> for Tree<T> {
    type Error = String;

    fn try_from(formula: &str) -> Result<Self, Self::Error> {
        use Tree::*;

        let mut stack = VecDeque::new();

        for (i, c) in formula.char_indices() {
            match c {
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
            1 => Ok(stack.pop_front().unwrap()),
            0 => Err("Empty formula".to_string()),
            2 => Err("Missing operator".to_string()),
            _ => Err("Missing operators".to_string()),
        }
    }
}

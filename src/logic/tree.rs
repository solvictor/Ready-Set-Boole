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

    fn flatten_cnf(self) -> Self {
        use Tree::*;

        fn collect_clause<T: Evaluable>(expr: &Tree<T>, clause: &mut Vec<Tree<T>>) {
            match expr {
                Or(left, right) => {
                    collect_clause(left, clause);
                    collect_clause(right, clause);
                }
                Value(_) | Variable(_) => clause.push(expr.clone()),
                Not(inner) => match inner.as_ref() {
                    Value(_) | Variable(_) => clause.push(expr.clone()),
                    _ => panic!("Formula isn't CNF"),
                },
                _ => panic!("Formula isn't CNF"),
            }
        }

        fn collect_clauses<T: Evaluable>(expr: &Tree<T>, clauses: &mut Vec<Vec<Tree<T>>>) {
            match expr {
                And(left, right) => {
                    collect_clauses(left, clauses);
                    collect_clauses(right, clauses);
                }
                other => {
                    let mut clause = Vec::new();
                    collect_clause(other, &mut clause);
                    clauses.push(clause);
                }
            }
        }

        let mut clauses = Vec::new();
        collect_clauses(&self, &mut clauses);
        let mut clauses = clauses
            .iter_mut()
            .map(|clause| {
                while clause.len() > 1 {
                    let right = clause.pop().unwrap();
                    let left = clause.pop().unwrap();
                    clause.push(Or(rc!(left), rc!(right)));
                }
                clause.pop().expect("Empty clause")
            })
            .collect::<Vec<Tree<T>>>();
        while clauses.len() > 1 {
            let right = clauses.pop().unwrap();
            let left = clauses.pop().unwrap();
            clauses.push(And(rc!(left), rc!(right)));
        }
        clauses.pop().expect("Empty clauses")
    }

    fn distribute_or(&self) -> Self {
        use Tree::*;

        match self {
            Or(left, right) => {
                let left = left.distribute_or();
                let right = right.distribute_or();

                match (left, right) {
                    (And(a1, a2), And(b1, b2)) => And(
                        rc!(And(
                            rc!(Or(a1.clone(), b1.clone()).distribute_or()),
                            rc!(Or(a1, b2.clone()).distribute_or()),
                        )),
                        rc!(And(
                            rc!(Or(a2.clone(), b1).distribute_or()),
                            rc!(Or(a2, b2).distribute_or()),
                        )),
                    ),
                    (And(a1, a2), c) => And(
                        rc!(Or(a1, rc!(c.clone())).distribute_or()),
                        rc!(Or(a2, rc!(c)).distribute_or()),
                    ),
                    (a, And(b1, b2)) => And(
                        rc!(Or(rc!(a.clone()), b1).distribute_or()),
                        rc!(Or(rc!(a), b2).distribute_or()),
                    ),
                    (a, b) => Or(rc!(a), rc!(b)),
                }
            }
            And(left, right) => And(rc!(left.distribute_or()), rc!(right.distribute_or())),
            other => other.clone(),
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

    pub fn cnf(&self) -> Self {
        self.nnf().distribute_or().flatten_cnf()
    }
}

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

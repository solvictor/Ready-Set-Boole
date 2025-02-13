use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

pub mod eval;
pub mod gray_code;
pub mod normal_forms;
pub mod operators;
pub mod truth_table;

#[macro_export]
macro_rules! boxed {
    ($a:expr) => {
        Box::new($a)
    };
}

#[derive(Clone, Debug)]
pub enum BooleanTree<T> {
    Value(T),
    Variable(char),
    Not(Box<BooleanTree<T>>),
    And(Box<BooleanTree<T>>, Box<BooleanTree<T>>),
    Or(Box<BooleanTree<T>>, Box<BooleanTree<T>>),
    Xor(Box<BooleanTree<T>>, Box<BooleanTree<T>>),
    Implication(Box<BooleanTree<T>>, Box<BooleanTree<T>>),
    Equivalence(Box<BooleanTree<T>>, Box<BooleanTree<T>>),
}

// TODO More abstraction
// TODO Compile time differentiation between NNF and Unchecked
impl<T> BooleanTree<T> {
    // Get variables of the formula in alphabetical order
    pub fn variables(&self) -> Vec<char> {
        use BooleanTree::*;

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
}

impl BooleanTree<bool> {
    pub fn evaluate(&self, state: Option<&HashMap<char, bool>>) -> Result<bool, String> {
        use BooleanTree::*;

        Ok(match self {
            Value(val) => *val,
            Variable(name) => *state
                .ok_or("Missing state")?
                .get(name)
                .ok_or(&format!("Missing value for {} in state", name))?,
            Not(p) => !p.evaluate(state)?,
            And(p, q) => p.evaluate(state)? & q.evaluate(state)?,
            Or(p, q) => p.evaluate(state)? | q.evaluate(state)?,
            Xor(p, q) => p.evaluate(state)? ^ q.evaluate(state)?,
            Implication(p, q) => !p.evaluate(state)? | q.evaluate(state)?,
            Equivalence(p, q) => p.evaluate(state)? == q.evaluate(state)?,
        })
    }

    // LGTM
    pub fn rpn_formula(&self) -> String {
        use BooleanTree::*;

        match self {
            Value(val) => if *val { "1" } else { "0" }.to_string(),
            Variable(var) => var.to_string(),
            Not(sub) => sub.rpn_formula() + "!",
            And(left, right) => left.rpn_formula() + &right.rpn_formula() + "&",
            Or(left, right) => left.rpn_formula() + &right.rpn_formula() + "|",
            Xor(left, right) => left.rpn_formula() + &right.rpn_formula() + "^",
            Implication(left, right) => left.rpn_formula() + &right.rpn_formula() + ">",
            Equivalence(left, right) => left.rpn_formula() + &right.rpn_formula() + "=",
        }
    }

    // TODO less clone ?
    // Any expression using Exclusive disjunction | Implication | Equivalence have multiple valid representations
    pub fn nnf(&self) -> Self {
        use BooleanTree::*;

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
        use BooleanTree::*;

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

    // TODO use DPLL or CDCL algorithm
    pub fn is_sat(&self) -> bool {
        fn backtrack(
            tree: &BooleanTree<bool>,
            i: usize,
            variables: &Vec<char>,
            state: &mut HashMap<char, bool>,
        ) -> Result<bool, String> {
            if i == variables.len() {
                return Ok(tree.evaluate(Some(state))?);
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

        backtrack(&tree, 0, &variables, &mut variables_state).unwrap()
    }

    pub fn as_char(&self) -> char {
        use BooleanTree::*;

        match self {
            Value(val) => {
                if *val {
                    '⊤'
                } else {
                    '⊥'
                }
            }
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

macro_rules! impl_flatten {
    ($fn_name:ident, $tree_variant:ident) => {
        impl BooleanTree<bool> {
            fn $fn_name(&self) -> Self {
                fn collect_clauses(node: &BooleanTree<bool>) -> Vec<BooleanTree<bool>> {
                    match node {
                        BooleanTree::$tree_variant(left, right) => collect_clauses(left)
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
                    BooleanTree::$tree_variant(boxed!(clause), boxed!(acc))
                })
            }
        }
    };
}

impl_flatten!(flatten_and, And);
impl_flatten!(flatten_or, Or);

// TODO

/*
101|& should give

  ^
 / \
1   v
   / \
  0   1


*/
impl Display for BooleanTree<bool> {
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

impl TryFrom<&str> for BooleanTree<bool> {
    type Error = String;

    fn try_from(formula: &str) -> Result<Self, Self::Error> {
        let mut stack = VecDeque::<BooleanTree<bool>>::new();

        for (i, c) in formula.char_indices() {
            match c {
                '0' | '1' => {
                    stack.push_back(Self::Value(c == '1'));
                    continue;
                }
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
            let tree = BooleanTree::<bool>::try_from(formula).expect("Failed to parse formula");
            assert_eq!(formula, tree.rpn_formula().as_str());
        });
    }
}

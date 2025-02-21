use std::{collections::HashSet, hash::Hash, ops::*, sync::Arc};

// TODO Maybe too much abstraction with this and Tree<T>
#[derive(Clone, PartialEq, Debug)]
pub struct Set<T: Eq + Hash + Clone> {
    data: HashSet<T>,
    universal: Option<Arc<HashSet<T>>>,
}

impl<T: Eq + Hash + Clone> BitXor for Set<T> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Set {
            data: &self.data ^ &rhs.data,
            universal: self.universal,
        }
    }
}

impl<T: Eq + Hash + Clone> BitOr for Set<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Set {
            data: &self.data | &rhs.data,
            universal: self.universal,
        }
    }
}

impl<T: Eq + Hash + Clone> BitAnd for Set<T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Set {
            data: &self.data & &rhs.data,
            universal: self.universal,
        }
    }
}

impl<T: Eq + Hash + Clone> Not for Set<T> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Set {
            data: &(*self.universal.clone().expect("Missing universal set")) - &self.data,
            universal: self.universal,
        }
    }
}

impl<T: Eq + Hash + Clone> Set<T> {
    pub fn new(data: HashSet<T>, universal: Option<Arc<HashSet<T>>>) -> Self {
        Set { data, universal }
    }
}

impl<T: Eq + Hash + Clone> IntoIterator for Set<T> {
    type Item = T;
    type IntoIter = std::collections::hash_set::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

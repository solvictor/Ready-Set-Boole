use std::{collections::HashSet, hash::Hash, ops::*, sync::Arc};

pub trait SetElement: Eq + Hash + Clone {}

// Implement it on any type that already can
impl<T: Eq + Hash + Clone> SetElement for T {}

#[derive(Clone, PartialEq, Debug)]
pub struct Set<T: SetElement> {
    data: HashSet<T>,
    universal: Option<Arc<HashSet<T>>>,
}

impl<T: SetElement> BitXor for Set<T> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Set {
            data: &self.data ^ &rhs.data,
            universal: self.universal,
        }
    }
}

impl<T: SetElement> BitOr for Set<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Set {
            data: &self.data | &rhs.data,
            universal: self.universal,
        }
    }
}

impl<T: SetElement> BitAnd for Set<T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Set {
            data: &self.data & &rhs.data,
            universal: self.universal,
        }
    }
}

impl<T: SetElement> Not for Set<T> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Set {
            data: &(*self.universal.clone().expect("Missing universal set")) - &self.data,
            universal: self.universal,
        }
    }
}

impl<T: SetElement> Set<T> {
    pub fn new(data: HashSet<T>, universal: Option<Arc<HashSet<T>>>) -> Self {
        Set { data, universal }
    }
}

impl<T: SetElement> IntoIterator for Set<T> {
    type Item = T;
    type IntoIter = std::collections::hash_set::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

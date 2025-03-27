mod booltree;
mod set;
mod tree;

pub use booltree::BoolTree;
pub use set::Set;
pub use tree::Tree;

use std::rc::Rc;

#[macro_export]
macro_rules! rc {
    ($a:expr) => {
        Rc::new($a)
    };
}

mod booltree;
mod set;
mod tree;

pub use booltree::BoolTree;
pub use set::Set;
pub use tree::Tree;

#[macro_export]
macro_rules! boxed {
    ($a:expr) => {
        Box::new($a)
    };
}

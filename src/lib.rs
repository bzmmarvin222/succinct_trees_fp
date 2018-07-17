#![feature(test)]

extern crate test;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bv;
extern crate bincode;

use bincode::serialize;
use bincode::deserialize;
use bv::BitVec;

mod bp;
mod louds;

trait SuccinctTree<T>: Sized {
    fn new(vec: BitVec<T>) -> Self;

    fn vec(&self) -> BitVec<T>;

    fn serialize(&self) -> Vec<u8>;
    fn deserialize(serialized: Vec<u8>) -> Self;

    // constant time for both
    fn index_represents_node(&self, x : u64) -> bool;
    fn is_leaf(&self, x : u64) -> Option<bool>;
    fn parent(&self, x : u64) -> Option<u64>;
    fn first_child(&self, x : u64) -> Option<u64>;
    fn next_sibling(&self, x : u64) -> Option<u64>;

    // constant time for BP
    fn ancestor(&self, x : u64, y : u64) -> Option<bool>;
    fn depth(&self, x : u64) -> Option<u64>;
    fn subtree_size(&self, x : u64) -> Option<u64>;

    // constant time for LOUDS
    fn child(&self, x : u64, i : u64) -> Option<u64>;
    fn degree(&self, x : u64) -> Option<u64>;
    fn child_rank(&self, x : u64) -> Option<u64>;
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;
    use louds::LOUDS;
    use bp::BalancedParentheses;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        let braces = String::from("((())(())()()(()())");
        let bp = BalancedParentheses::from_braces_representation(braces);
        b.iter(|| bp.subtree_size(0));
    }
}

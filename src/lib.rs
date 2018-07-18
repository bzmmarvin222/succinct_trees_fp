#![feature(test)]

extern crate test;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bv;
extern crate bincode;
extern crate rand;

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
    use rand::prelude::*;
    use bv::BitsMut;

    fn rand_big_tree_bp() -> BitVec {
        let upper_limit = 1000000000;
        let mut result = BitVec::new_fill(true, upper_limit);
        let mut current_depth = 1;
        for index in 1..upper_limit {
            let rand: bool = random();
            if !rand {
                result.set_bit(index, rand);
                current_depth -= 1;
            } else if upper_limit - index == current_depth {
                result.set_bit(index, false);
                current_depth -= 1;
            } else {
                current_depth += 1;
            }
        }
        result
    }
    fn rand_big_tree_louds() -> BitVec<u8> {
        let upper_limit = 1000000000;
        let mut result: BitVec<u8> = BitVec::new_fill(true, upper_limit);
        let mut current_depth = 1;
        for index in 1..upper_limit {
            let rand: bool = random();
            if !rand {
                result.set_bit(index, rand);
                current_depth -= 1;
            } else if upper_limit - index == current_depth {
                result.set_bit(index, false);
                current_depth -= 1;
            } else {
                current_depth += 1;
            }
        }
        result
    }
    #[bench]
    fn bench_bp_subtree_size(b: &mut Bencher) {
        let bp = BalancedParentheses::new(rand_big_tree_bp());
        b.iter(|| bp.subtree_size(0));
    }

    #[bench]
    fn bench_louds_subtree_size(b: &mut Bencher) {
        let louds = LOUDS::new(rand_big_tree_louds());
        b.iter(|| louds.subtree_size(0));
    }

    #[bench]
    fn bench_bp_index_represents_node(b: &mut Bencher) {
        let bp = BalancedParentheses::new(rand_big_tree_bp());
        let mut index = 133742;
        while !bp.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| bp.index_represents_node(index));
    }

    #[bench]
    fn bench_louds_index_represents_node(b: &mut Bencher) {
        let louds = LOUDS::new(rand_big_tree_louds());
        let mut index = 133742;
        while !louds.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| louds.index_represents_node(index));
    }

    #[bench]
    fn bench_bp_is_leaf(b: &mut Bencher) {
        let bp = BalancedParentheses::new(rand_big_tree_bp());
        let mut index = 133742;
        while !bp.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| bp.is_leaf(index));
    }

    #[bench]
    fn bench_louds_is_leaf(b: &mut Bencher) {
        let louds = LOUDS::new(rand_big_tree_louds());
        let mut index = 133742;
        while !louds.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| louds.is_leaf(index));
    }

    #[bench]
    fn bench_bp_parent(b: &mut Bencher) {
        let bp = BalancedParentheses::new(rand_big_tree_bp());
        let mut index = 133742;
        while !bp.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| bp.parent(index));
    }

    #[bench]
    fn bench_louds_parent(b: &mut Bencher) {
        let louds = LOUDS::new(rand_big_tree_louds());
        let mut index = 133742;
        while !louds.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| louds.parent(index));
    }

    #[bench]
    fn bench_bp_next_sibling(b: &mut Bencher) {
        let bp = BalancedParentheses::new(rand_big_tree_bp());
        let mut index = 133742;
        while !bp.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| bp.next_sibling(index));
    }

    #[bench]
    fn bench_louds_next_sibling(b: &mut Bencher) {
        let louds = LOUDS::new(rand_big_tree_louds());
        let mut index = 133742;
        while !louds.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| louds.next_sibling(index));
    }

    #[bench]
    fn bench_bp_first_child(b: &mut Bencher) {
        let bp = BalancedParentheses::new(rand_big_tree_bp());
        let mut index = 133742;
        while !bp.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| bp.first_child(index));
    }

    #[bench]
    fn bench_louds_first_child(b: &mut Bencher) {
        let louds = LOUDS::new(rand_big_tree_louds());
        let mut index = 133742;
        while !louds.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| louds.first_child(index));
    }

    #[bench]
    fn bench_bp_degree(b: &mut Bencher) {
        let bp = BalancedParentheses::new(rand_big_tree_bp());
        let mut index = 133742;
        while !bp.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| bp.degree(index));
    }

    #[bench]
    fn bench_louds_degree(b: &mut Bencher) {
        let louds = LOUDS::new(rand_big_tree_louds());
        let mut index = 133742;
        while !louds.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| louds.degree(index));
    }

    #[bench]
    fn bench_bp_child(b: &mut Bencher) {
        let bp = BalancedParentheses::new(rand_big_tree_bp());
        let mut index = 133742;
        while !bp.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| bp.child(index, 3));
    }

    #[bench]
    fn bench_louds_child(b: &mut Bencher) {
        let louds = LOUDS::new(rand_big_tree_louds());
        let mut index = 133742;
        while !louds.index_represents_node(index) {
            index += 1;
        }
        b.iter(|| louds.child(index, 3));
    }
}

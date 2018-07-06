extern crate serde;
extern crate bincode;
extern crate bv;

use self::bv::*;
use self::bincode::{serialize, deserialize};
use SuccinctTree;
use bp::range_min_max_tree::RangeMinMaxTree;
use std::cmp;


mod range_min_max_tree;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BalancedParentheses {
    vec: BitVec<usize>,
    rmm: RangeMinMaxTree
}

impl BalancedParentheses {

    fn has_index(&self, index: u64) -> bool {
        index < self.vec.len()
    }

    fn rank_closing_brace(&self, index: u64) -> usize {
        //TODO: real impl
        3
    }

    fn rank_opening_brace(&self, index: u64) -> usize {
        //TODO: real impl
        4
    }

    fn find_close(&self, index: u64) -> u64 {
        //TODO: real impl
        5
    }
}

impl SuccinctTree for BalancedParentheses {
    fn new(vector: BitVec<usize>) -> BalancedParentheses {
        let next_pow = vector.len().next_power_of_two() as f64;
        let blocksize = next_pow.log2() as usize;
        let sanitized_blocksize = cmp::max(blocksize, 8);
        BalancedParentheses {
            vec: (&vector).to_owned(),
            rmm: RangeMinMaxTree::new(vector, sanitized_blocksize)
        }
    }

    fn vec(&self) -> BitVec<usize> {
        (&self.vec).to_owned()
    }

    fn serialize(&self) -> Vec<u8> {
        serialize(self).unwrap()
    }

    fn deserialize(serialized: Vec<u8>) -> BalancedParentheses {
        deserialize(&serialized[..]).unwrap()
    }

    fn index_represents_node(&self, x : u64) -> bool {
        self.vec[x]
    }

    fn is_leaf(&self, x : u64) -> Option<bool>{
        if !self.has_index(x + 1) {
            return None;
        }
        Some(self.vec[x] && !self.vec[x+1])
    }

    fn parent(&self, x : u64) -> Option<u64> {
        //TODO: real impl
        Some(5)
    }

    fn first_child(&self, x : u64) -> Option<u64>{
        if !self.index_represents_node(x) || !self.vec[x + 1]{
            return None;
        }
        Some(x + 1)
    }

    fn next_sibling(&self, x : u64) -> Option<u64>{
        //TODO: real impl
        Some(5)
    }

    fn ancestor(&self, x : u64, y : u64) -> Option<bool>{
        if !self.has_index(x) || !self.has_index(y) {
            return None;
        }
        Some(x <= y && y <= self.find_close(x))
    }

    fn depth(&self, x : u64) -> Option<u64>{
        //TODO: real impl
        Some(5)
    }

    fn subtree_size(&self, x : u64) -> Option<u64>{
        //TODO: real impl
        Some(5)
    }

    //these functions need more than constant time
    //to be implemented
    fn child(&self, x : u64, i : u64) -> Option<u64>{
        //TODO: real impl
        Some(5)
    }

    fn degree(&self, x : u64) -> Option<u64>{
        //TODO: real impl
        Some(5)
    }

    fn child_rank(&self, x : u64) -> Option<u64>{
        //TODO: real impl
        Some(5)
    }

}

#[cfg(test)]
mod tests;

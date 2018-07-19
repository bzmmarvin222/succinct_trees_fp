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

    fn find_close(&self, index: u64) -> Option<u64> {
        //if !self.index_represents_node(index) {
        //    return None;
        //}
        let result = self.rmm.fwdsearch((index) as usize, -1);
        if let Some(res) = result {
            return Some(res as u64);
        }
        None
    }

    pub fn from_braces_representation(braces: String) -> Self {
        let mut vect = BitVec::new();
        for brace in braces.chars() {
            if brace == '(' {
                vect.push(true);
            } else if brace == ')' {
                vect.push(false);
            } else {
                panic!("only ( and ) allowed");
            }
        }
        Self::new(vect)
    }

    pub fn braces_representation(&self) -> String {
        let vec = &self.vec();
        let mut result = String::new();

        for i in 0..vec.len() {
            if vec.as_slice()[i] {
                result.push('(');
            } else {
                result.push(')');
            }
        }
        result
    }
}

impl SuccinctTree<usize> for BalancedParentheses {
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
        if !self.has_index(x + 1) || !self.index_represents_node(x) {
            return None;
        }
        Some(!self.vec[x+1])
    }

    fn parent(&self, x : u64) -> Option<u64> {
        if !self.has_index(x) || !self.index_represents_node(x) {
            return None;
        }
        let result = self.rmm.bwdsearch(x as usize, -2);
        if let Some(res) = result {
            return Some(res as u64);
        }
        None
    }

    fn first_child(&self, x : u64) -> Option<u64>{
        if !self.has_index(x) || !self.index_represents_node(x) || !self.vec[x + 1]{
            return None;
        }
        Some(x + 1)
    }

    fn next_sibling(&self, x : u64) -> Option<u64>{
        if !self.has_index(x) || !self.index_represents_node(x) {
            return None;
        }
        if let Some(index) = self.find_close(x) {
            if self.vec[index + 1] {
                return Some(index);
            }
        }
        None
    }

    fn ancestor(&self, x : u64, y : u64) -> Option<bool>{
        println!("is node {:?}", self.index_represents_node(x));
        if !self.has_index(x) || !self.has_index(y) || !self.index_represents_node(x) {
            return None;
        }
        println!("find_close: {:?}", self.find_close(x)?);
        println!("y {:?}", y);
        Option::from(x <= y && self.find_close(x)? >= y)
    }

    fn depth(&self, x : u64) -> Option<u64>{
        if !self.has_index(x) || !self.index_represents_node(x) {
            return None;
        } else if x == 0 {
            return Some(0);
        }
        Option::from((self.rmm.rank_1((x) as usize)? - self.rmm.rank_0((x) as usize)?) as u64)
    }

    fn subtree_size(&self, x : u64) -> Option<u64>{
        if !self.has_index(x) || !self.index_represents_node(x) {
            return None;
        }
        Option::from((self.find_close(x)? - x + 1) / 2)
    }

    //these functions need more than constant time
    //to be implemented
    fn child(&self, x : u64, i : u64) -> Option<u64>{
        if !self.has_index(x) || !self.index_represents_node(x) {
            return None;
        }

        let index_close = self.find_close(x)?;
        let mut current_depth = 0;
        let mut already_found_children = 0;

        for index in x..index_close {
            let is_node = self.vec[index];
            if is_node {
                if already_found_children == i {
                    return Some(index);
                } else if current_depth == 0 {
                    already_found_children += 1;
                }
                current_depth += 1;
            } else {
                current_depth -= 1;
            }
        }
        None
    }

    fn degree(&self, x : u64) -> Option<u64>{
        if !self.has_index(x) || !self.index_represents_node(x) {
            return None;
        }

        let index_close = self.find_close(x)?;
        let mut current_depth = 0;
        let mut current_degree = 0;

        for index in x..index_close {
            let is_node = self.vec[index];
            if is_node {
                if current_depth == 0 {
                    current_degree += 1;
                }
                current_depth += 1;
            } else {
                current_depth -= 1;
            }
        }
        Some(current_degree)
    }

    fn child_rank(&self, x : u64) -> Option<u64>{
        //TODO: real impl
        Some(5)
    }

}

#[cfg(test)]
mod tests;

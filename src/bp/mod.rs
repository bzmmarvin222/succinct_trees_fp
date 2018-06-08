extern crate serde;
extern crate bincode;
extern crate bv;

use self::bv::BitVec;
use self::bincode::{serialize, deserialize};

mod range_min_max_tree;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BalancedParentheses {
    vec: BitVec
}

impl BalancedParentheses {
    fn new(vector: BitVec) -> BalancedParentheses {
        BalancedParentheses {
            vec: vector
        }
    }

    fn from_binary_represantation(bin: String) -> BalancedParentheses {
        //TODO: real impl
        BalancedParentheses::new(BitVec::new_fill(true, 8))
    }

    fn from_braces_represantation(braces: String) -> BalancedParentheses {
        //TODO: real impl
        BalancedParentheses::new(BitVec::new_fill(true, 8))
    }

    fn binary_representation() -> String {
        //TODO: real impl
        String::from("01010101")
    }

    fn braces_represantation() -> String {
        //TODO: real impl
        String::from("((()())())")
    }

    fn rank_0(&self, index: u64) -> u64 {
        //TODO: real impl
        3
    }

    fn rank_1(&self, index: u64) -> u64 {
        //TODO: real impl
        4
    }

    fn find_close(&self, index: u64) -> u64 {
        //TODO: real impl
        5
    }

    fn is_leaf(&self, index: u64) -> bool {
        //TODO: real impl
        true
    }

    fn represents_node(&self, index: u64) -> bool {
        //TODO: real impl
        true
    }

    fn ancestor(&self, index_to_test: u64, index_ancestor: u64) -> bool {
        //TODO: real impl
        true
    }

    fn parent(&self, index: u64) -> u64 {
        //TODO: real impl
        5
    }

    fn first_child(&self, index: u64) -> u64 {
        //TODO: real impl
        5
    }

    fn subtree_size(&self, index: u64) -> u64 {
        //TODO: real impl
        5
    }

    fn serialize(&self) -> Vec<u8> {
        serialize(self).unwrap()
    }

    fn deserialize(serialized: Vec<u8>) -> BalancedParentheses {
        deserialize(&serialized[..]).unwrap()
    }
}

#[cfg(test)]
mod tests;

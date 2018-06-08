extern crate serde;
extern crate bincode;
extern crate bv;

use self::bv::*;
use self::bincode::{serialize, deserialize};

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

    fn from_binary_representation(bin: String) -> BalancedParentheses {
        //TODO: real impl
        BalancedParentheses::new(BitVec::new_fill(true, 8))
    }

    fn from_braces_representation(braces: String) -> BalancedParentheses {
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
        BalancedParentheses::new(vect)
    }

    fn binary_representation() -> String {
        //TODO: real impl
        String::from("01010101")
    }

    fn braces_representation(&self) -> String {
        let vec = &self.vec;
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

    fn rank_closing_brace(&self, index: u64) -> u64 {
        //TODO: real impl
        3
    }

    fn rank_opening_brace(&self, index: u64) -> u64 {
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
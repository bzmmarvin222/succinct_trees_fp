extern crate serde;
extern crate bincode;
extern crate bv;

use self::bv::*;
use self::bincode::{serialize, deserialize};
use SuccinctTree;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BalancedParentheses {
    vec: BitVec
}

impl BalancedParentheses {
    fn rank_closing_brace(&self, index: usize) -> usize {
        //TODO: real impl
        3
    }

    fn rank_opening_brace(&self, index: usize) -> usize {
        //TODO: real impl
        4
    }

    fn find_close(&self, index: usize) -> usize {
        //TODO: real impl
        5
    }

    fn is_leaf(&self, index: usize) -> bool {
        //TODO: real impl
        true
    }

    fn represents_node(&self, index: usize) -> bool {
        //TODO: real impl
        true
    }

    fn ancestor(&self, index_to_test: usize, index_ancestor: usize) -> bool {
        //TODO: real impl
        true
    }

    fn parent(&self, index: usize) -> usize {
        //TODO: real impl
        5
    }

    fn first_child(&self, index: usize) -> usize {
        //TODO: real impl
        5
    }

    fn subtree_size(&self, index: usize) -> usize {
        //TODO: real impl
        5
    }
}

impl SuccinctTree for BalancedParentheses {
    fn new(vector: BitVec) -> BalancedParentheses {
        BalancedParentheses {
            vec: vector
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
}

#[cfg(test)]
mod tests;
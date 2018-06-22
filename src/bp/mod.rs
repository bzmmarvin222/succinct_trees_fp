extern crate serde;
extern crate bincode;
extern crate bv;

use self::bv::*;
use self::bincode::{serialize, deserialize};
use SuccinctTree;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BalancedParentheses {
    vec: BitVec<u8>
}

impl BalancedParentheses {

    fn rank_closing_brace(&self, index: u64) -> usize {
        //TODO: real impl
        3
    }

    fn rank_opening_brace(&self, index: u64) -> usize {
        //TODO: real impl
        4
    }

    fn find_close(&self, index: u64) -> usize {
        //TODO: real impl
        5
    }

/*    fn is_leaf(&self, index: usize) -> bool {
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
    }*/
}

impl SuccinctTree for BalancedParentheses {
    fn new(vector: BitVec<u8>) -> BalancedParentheses {
        BalancedParentheses {
            vec: vector
        }
    }

    fn vec(&self) -> BitVec<u8> {
        (&self.vec).to_owned()
    }

    fn serialize(&self) -> Vec<u8> {
        serialize(self).unwrap()
    }

    fn deserialize(serialized: Vec<u8>) -> BalancedParentheses {
        deserialize(&serialized[..]).unwrap()
    }

    fn index_represents_node(&self, x : u64) -> bool{
        //TODO: real impl
        true
    }

    fn is_leaf(&self, x : u64) -> Option<bool>{
        //TODO: real impl
        Some(true)
    }

    fn parent(&self, x : u64) -> Option<u64> {
        //TODO: real impl
        Some(5)
    }

    fn first_child(&self, x : u64) -> Option<u64>{
        //TODO: real impl
        Some(5)
    }

    fn next_sibling(&self, x : u64) -> Option<u64>{
        //TODO: real impl
        Some(5)
    }


    fn ancestor(&self, x : u64, y : u64) -> Option<bool>{
        //TODO: real impl
        Some(true)
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

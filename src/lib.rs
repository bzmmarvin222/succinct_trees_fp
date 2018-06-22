extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bv;
extern crate bincode;

use bv::BitVec;

mod bp;
mod louds;

use bp::BalancedParentheses;
use bincode::serialize;
use bincode::deserialize;

trait SuccinctTree: Sized {
    fn new(vec: BitVec<u8>) -> Self;

    fn vec(&self) -> BitVec<u8>;

    fn from_binary_representation(bin: String) -> Self {
        let braces = bin.replace("1", "(").replace("0", ")");
        Self::from_braces_representation(bin)
    }
    fn from_braces_representation(braces: String) -> Self {
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
    fn binary_representation(&self) -> String{
        self.braces_representation().replace("(", "1").replace(")", "0")
    }
    fn braces_representation(&self) -> String {
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

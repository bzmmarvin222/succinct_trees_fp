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
    fn new(vec: BitVec) -> Self;
    fn vec(&self) -> BitVec;
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
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

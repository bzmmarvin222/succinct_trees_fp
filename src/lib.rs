extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bv;
extern crate bincode;

use bv::BitVec;

mod bp;

mod louds;

use bincode::serialize;
use bincode::deserialize;

trait SuccinctTree<T>: Sized {
    fn new(vec: BitVec<T>) -> Self;

    fn vec(&self) -> BitVec<T>;

    fn serialize(&self) -> Vec<u8>;
    fn deserialize(serialized: Vec<u8>) -> Self;

    // constant time for both
    fn index_represents_node(&self, x : usize) -> bool;
    fn is_leaf(&self, x : usize) -> Option<bool>;
    fn parent(&self, x : usize) -> Option<usize>;
    fn first_child(&self, x : usize) -> Option<usize>;
    fn next_sibling(&self, x : usize) -> Option<usize>;

    // constant time for BP
    fn ancestor(&self, x : usize, y : usize) -> Option<bool>;
    fn depth(&self, x : usize) -> Option<usize>;
    fn subtree_size(&self, x : usize) -> Option<usize>;

    // constant time for LOUDS
    fn child(&self, x : usize, i : usize) -> Option<usize>;
    fn degree(&self, x : usize) -> Option<usize>;
    fn child_rank(&self, x : usize) -> Option<usize>;
}

#[cfg(test)]
mod tests {

}

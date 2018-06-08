//extern crate serde;
//extern crate bincode;
extern crate bv;
extern crate bio;

use self::bio::data_structures::rank_select::RankSelect;
use self::bv::BitVec;
use self::bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LOUDS{
    vec: BitVec
}

impl LOUDS{
    fn new(vector: BitVec) -> LOUDS {
    //    k = (log(vector.bit_len()) hoch 2)/32;
        k=1;
        LOUDS {
            vec: vector,
            rank_select_structure: RankSelect::new(vec,k)
        }
    }

    fn from_binary_represantation(bin: String) -> LOUDS {
        //TODO: real impl
        LOUDS::new(BitVec::new_fill(true, 8))
    }

    fn from_braces_represantation(braces: String) -> LOUDS {
        //TODO: real impl
        LOUDS::new(BitVec::new_fill(true, 8))
    }

    fn binary_representation() -> String {
        //TODO: real impl
        String::from("01010101")
    }

    fn braces_represantation() -> String {
        //TODO: real impl
        String::from("((()())())")
    }


//never pass negative numbers as parameters in this class


// from here x,y are the elements of the sequence, which
// represent a node. So, in vec there is 0 at the Position x - 1,y - 1

    fn prev_0(x : u64){
        this.rank_select_structure.select_0(this.rank_select_structure.rank_0(x))
    }

    fn next0(x : u64){
        this.rank_select_structure.select_0(this.rank_select_structure.rank_0(x)+1)
    }

    // whether x is a leaf
    fn is_leaf(x : u64){

    }

    // number of siblings to the left of node x
    fn child_rank(x : u64){

    }

    // next sibling (to the right) of node x
    fn next_sibling(x : u64){

    }

    // number of children of node x
    fn degree(x : u64){

    }

    // parent of node x
    fn parent(x : u64){

    }

    // i-th child of node x
    fn child(x : u64, i : u64){

    }





}

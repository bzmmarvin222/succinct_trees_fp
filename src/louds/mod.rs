extern crate serde;
extern crate bincode;
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
        LOUDS {
            vec: vector,
            rank_select_structure: RankSelect::new(vec,1);
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

    fn prev0(i : u64){

    }

    fn next0(i : u64){

    }


}

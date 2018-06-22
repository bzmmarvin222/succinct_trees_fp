extern crate serde;
extern crate bincode;
extern crate bv;
extern crate bio;

use self::bio::data_structures::rank_select::RankSelect;
use self::bv::BitVec;
use self::bincode::{serialize, deserialize};
use SuccinctTree;

#[derive(Serialize, Deserialize)]
pub struct LOUDS{
    vec: BitVec<u8>,
    rank_select_structure: RankSelect
}

impl LOUDS {
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
    fn index_represents_node(&self, x : u64) -> bool {
        let result = (self.vec[x-1] == false);
        result
    }


    fn prev_0(&self, x : u64) -> Option<u64> {
        if (self.index_represents_node(x))
        {
            match self.rank_select_structure.rank_0(x) {
                Some(rank0_x) => {
                    self.rank_select_structure.select_0(rank0_x)
                },
                None => {None}
            }
        }
        else{None}
    }


    fn next_0(&self, x : u64) -> Option<u64> {
        if (self.index_represents_node(x))
        {
            match self.rank_select_structure.rank_0(x) {
                Some(rank0_x) => {
                    self.rank_select_structure.select_0(rank0_x + 1)
                },
                None => {None}
            }
        }
        else{None}
    }

}

impl SuccinctTree for LOUDS {
    fn new(vector: BitVec<u8>) -> LOUDS {
    //    k = (log(vector.bit_len()) hoch 2)/32;
        let k=1;
        // vector is a bv BitVec<Block = usize>
        // usize is u8 of a 64 bit mashine and u4 of a 32 bit mashine
        // u8 vertor shall be a bv BitVec<Block = u8>
        LOUDS {
            vec: (&vector).to_owned(),
            rank_select_structure: RankSelect::new(vector,k)
        }
    }

    fn vec(&self) -> BitVec<u8> {
        (&self.vec).to_owned()
    }

    fn serialize(&self) -> Vec<u8> {
        serialize(self).unwrap()
    }

    fn deserialize(serialized: Vec<u8>) -> LOUDS {
        deserialize(&serialized[..]).unwrap()
    }

    //never pass negative numbers as parameters in this class

    fn index_represents_node(&self, x : u64) -> bool {
        let result = (self.vec[x-1] == false);
        result
    }

        // from here x,y are the elements of the sequence, which
        // represent a node. So, in vec there is 0 at the Position x - 1,y - 1

    // whether x is a leaf
    fn is_leaf(&self, x : u64) -> Option<bool> {
        if (self.index_represents_node(x))
        {
            let result = (self.vec[x] == false);
            Some(result)
        }
        else
        {None}
    }


    // number of siblings to the left of node x
    fn child_rank(&self, x : u64) -> Option<u64> {
        if self.index_represents_node(x)
        {
            match self.rank_select_structure.rank_0(x - 1) {
                Some(rank0_x_1) => {
                    match self.rank_select_structure.select_1(rank0_x_1) {
                        Some(select1_rank0_x_1) => {
                            match self.prev_0(select1_rank0_x_1) {
                                Some(prev_zero_select1_rank0_x_1) => {
                                    let result = select1_rank0_x_1 - prev_zero_select1_rank0_x_1;
                                    Some(result)
                                },
                                None => {None}
                            }
                        },
                        None => {None}
                    }
                },
                None => {None}
            }
        }
        else {None}
    }


    // next sibling (to the right) of node x
    fn next_sibling(&self, x : u64) -> Option<u64> {
        if self.index_represents_node(x)
        {
            match self.rank_select_structure.rank_0(x - 1) {
                Some(rank0_x_1) => {
                    match self.rank_select_structure.select_1(rank0_x_1 + 1) {
                        Some(select1_rank0_x_1) => {
                            self.rank_select_structure.select_0(select1_rank0_x_1 + 1)
                        },
                        None => {None}
                    }
                },
                None => {None}
            }
        }
        else {None}
    }


    // number of children of node x
    fn degree(&self, x : u64) -> Option<u64> {
        if self.index_represents_node(x)
        {
            match self.next_0(x) {
                Some(next0_x) => {
                    let result = next0_x - x;
                    Some(result)
                },
                None => {None}
            }
        }
        else
        {None}
    }


    // parent of node x
    fn parent(&self, x : u64) -> Option<u64> {
        if self.index_represents_node(x)
        {
            match self.rank_select_structure.rank_0(x) {
                Some(rank0_x) => {
                    match self.rank_select_structure.select_1(rank0_x) {
                        Some(select1_rank0_x) => {
                            match self.prev_0(select1_rank0_x) {
                                Some(prev_zero_select1_rank0_x) => {
                                    let result = prev_zero_select1_rank0_x + 1;
                                    Some(result)
                                },
                                None => {None}
                            }
                        },
                        None => {None}
                    }
                },
                None => {None}
            }
        }
        else {None}
    }


    // i-th child of node x
    fn child(&self, x : u64, i : u64) -> Option<u64> {
        if self.index_represents_node(x)
        {
            match self.rank_select_structure.rank_1(x) {
                Some(rank1_x) => {
                    match self.rank_select_structure.select_0(rank1_x + i) {
                        Some(select0_rank1_x_i) => {
                            let result = select0_rank1_x_i + 1;
                            Some(result)
                        },
                        None => {None}
                    }
                },
                None => {None}
            }
        }
        else {None}
    }

    fn first_child(&self, x : u64) -> Option<u64> {
        self.child(x,1)
    }

    //these functions need more than constant time
    //to be implemented
    fn ancestor(&self, x : u64, y : u64) -> Option<bool>{
        Some(true)
    }


    fn depth(&self, x : u64) -> Option<u64>{
        Some(9)
    }


    fn subtree_size(&self, x : u64) -> Option<u64>{
        Some(9)
    }


}

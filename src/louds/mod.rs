extern crate serde;
extern crate bincode;
extern crate bv;
extern crate bio;

use self::bio::data_structures::rank_select::RankSelect;
use self::bv::BitVec;
use self::bincode::{serialize, deserialize};
use SuccinctTree;
use std::cmp;

#[derive(Serialize, Deserialize)]
pub struct LOUDS{
    vec: BitVec<u8>,
    rank_select_structure: RankSelect
}

impl LOUDS {

//never pass negative numbers as parameters in this class

    // from here x,y are the elements of the sequence, which
    // represent a node. So, in vec there is 0 at the Position x - 1


    //prev returns the 0 of the node before if at index x in rank_select_struckt there is 1
    // prev0 returns x if at index x in rank_select_struckt there is 0
    fn prev_0(&self, x : u64) -> Option<u64> {
        if self.index_represents_node(x)
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

    //next_0 returns always the 0 of the node after
    fn next_0(&self, x : u64) -> Option<u64> {
        if self.index_represents_node(x)
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

    fn from_binary_representation(bin: String) -> Self {
        let mut vect = BitVec::new();
        for b in bin.chars() {
            if b == '1' {
                vect.push(true);
            } else if b == '0' {
                vect.push(false);
            } else {
                panic!("only 1 and 0 allowed");
            }
        }
        Self::new(vect)
    }

    fn binary_representation(&self) -> String{
        let vec = &self.vec();
        let mut result = String::new();

        for i in 0..vec.len() {
            if vec.as_slice()[i] {
                result.push('1');
            } else {
                result.push('0');
            }
        }
        result
    }

}

impl SuccinctTree<u8> for LOUDS {
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
        x >= 1 && (!self.vec[x - 1] || x == 1)
    }

        // from here x,y are the elements of the sequence, which
        // represent a node. So, in vec there is 0 at the Position x - 1,y - 1

    // whether x is a leaf
    fn is_leaf(&self, x : u64) -> Option<bool> {
        Option::from(self.index_represents_node(x) && !self.vec[x])
    }


    // parent of node x
    fn parent(&self, x : u64) -> Option<u64> {
        if !self.index_represents_node(x) {
            return None;
        }

        let rank_0 = self.rank_select_structure.rank_0(x);
        let select_1 = self.rank_select_structure.select_1(rank_0?);
        let normed_select_1 = cmp::max(select_1?, 1);
        let result = self.prev_0(normed_select_1);
        Option::from(result? + 1)
    }


    fn first_child(&self, x : u64) -> Option<u64> {
        self.child(x,1)
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


    //returns whether x is an ansestor of y
    fn ancestor(&self, x : u64, c : u64) -> Option<bool>{
        if(self.index_represents_node(x) && self.index_represents_node(c)){
            let mut y = c;
            while(y>x){
                match self.parent(y){
                    Some(parent_y) => {
                        y = parent_y;
                    },
                    None => {return Some(false)}
                }
            }
            if(y==x){return Some(true)}
            else {return Some(false)}
        }
        None
    }


    //quantity of nodes in the path from root to c node
      fn depth(&self, c : u64) -> Option<u64>{
        if(self.index_represents_node(c)){
            let mut x = c;
            let mut levels_passed = 0;
            while(x>1){
                match self.parent(x){
                    Some(parent_x) => {
                        x = parent_x;
                        levels_passed = levels_passed +1;
                    },
                    None => {return None}
                }
            }
            if(x==1){return Some(levels_passed + 1)}
            else {return None}
        }
        None
    }

    // subtree_size(x) = 1 + add all (subtree_size(current_child) where current_child = child(x,i))
    fn subtree_size(&self, x : u64) -> Option<u64>{
        let mut result = 1;
        //whether x a valid node is will be controlled in degree(x)
        //no need to do it twice
        match self.degree(x){
            Some(quant_children_x) => {
                    for i in 1..quant_children_x
                    {
                        match self.child(x, i){
                            Some(current_child) => {
                                match self.subtree_size(current_child){
                                    Some(current_child_subtree_size) =>{
                                        result = result + current_child_subtree_size;
                                    },
                                    // if some child has a not valid subtree then there is an error
                                    None => {return None}
                                }

                            },
                            // if i-th child of x is not a valid node then there is an error
                            None => {return None}
                        }
                    }
                    return Some(result)
            },
            None => {None}
        }
    }

    //these functions need more than constant time
    //to be implemented

    // i-th child of node x
    fn child(&self, x : u64, i : u64) -> Option<u64> {
        if !self.index_represents_node(x) {
            return None;
        }

        let rank_1 = self.rank_select_structure.rank_1(x);
        let result = self.rank_select_structure.select_0(rank_1? + i - 1);
        Option::from(result? + 1)
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


}

#[cfg(test)]
mod tests;

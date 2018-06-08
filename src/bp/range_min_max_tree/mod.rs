extern crate bv;
use self::bv::BitVec;
use std::cmp;

#[derive(Default)]
pub struct rmm_Node {
    pub excess:  i64,
    pub min_excess: i64,
    pub max_excess: i64,
}

impl rmm_Node {
    pub fn new() -> rmm_Node{
        rmm_Node {
            excess : 0,
            min_excess : 0,
            max_excess : 0,
        }
    }

}


pub struct RangeMinMaxTree {
    blocksize: usize,
    tree: Vec<rmm_Node>,
    bitvector: BitVec,
}

impl RangeMinMaxTree {
    pub fn new(bitvector: BitVec, blocksize: usize) -> RangeMinMaxTree {
        RangeMinMaxTree {
            blocksize,
            tree: RangeMinMaxTree::build_from_bv(&bitvector, blocksize),
            bitvector
        }
    }

    fn build_from_bv(bitvector: &BitVec, blocksize: usize) -> Vec<rmm_Node> {

        let blocksize = blocksize as u64;

        let quantity_children = match bitvector.len()%blocksize {
             0 => bitvector.len()/blocksize,
             _ => bitvector.len()/blocksize+1,
         };

         let nodeCount =  match quantity_children.checked_next_power_of_two() {
             Some(n) => 2*n -1,
             None => panic!("Overflow Error"),
         };

         let mut heap: Vec<rmm_Node> = Vec::with_capacity(nodeCount as usize);

         let mut e: i64 = 0;
         let mut m: i64 = 0;
         let mut M: i64 = 0;
         let mut currentLeaf = heap.len()/2;

         for i in 0 .. bitvector.len() {
             if bitvector[i] {
                 e += 1;
             }
             else {
                 e -= 1;
             }
             m = cmp::min(e,m);
             M = cmp::max(e,M);
             if (i+1) % blocksize == 0 {
                 heap[currentLeaf].excess = e;
                 heap[currentLeaf].min_excess = m;
                 heap[currentLeaf].max_excess = M;
                 currentLeaf +=1;
                 e=0;
                 m=0;
                 M=0;
             }
         }

         for x in (heap.len()/2)-1 .. 0 {
             let left = 2*x+1;
             let right = 2*x+2;
             e = heap[left].excess + heap[right].excess;
             m = cmp::min(heap[left].min_excess, heap[left].excess + heap[right].min_excess);
             M = cmp::max(heap[left].max_excess, heap[left].excess + heap[right].max_excess);
             heap[x].excess = e;
             heap[x].min_excess = m;
             heap[x].max_excess = M;
         }

         heap
    }

    fn rank_1(&self, i: usize) -> Option<u64>{
        unimplemented!();
    }

    fn rank_0(&self, i: usize) -> Option<u64>{
        unimplemented!();
    }

    fn rank_10(&self, i: usize) -> Option<u64>{
        unimplemented!();
    }

    fn select_1(&self, k: usize) -> Option<usize>{
        unimplemented!();
    }

    fn select_0(&self, k: usize) -> Option<usize>{
        unimplemented!();
    }

    fn select_10(&self, k: usize) -> Option<usize>{
        unimplemented!();
    }

    fn fwdsearch(&self, i: usize, d: i64) -> Option<usize>{
        unimplemented!();
    }

    fn bwdsearch(&self, i: usize, d: i64) -> Option<usize>{
        unimplemented!();
    }
}

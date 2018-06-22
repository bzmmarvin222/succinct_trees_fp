extern crate bv;
use self::bv::BitVec;
use std::cmp;

#[derive(Default)]
pub struct rmm_Node {
    pub excess:  i64,
    pub min_excess: i64,
    pub max_excess: i64,
    pub count_bits: u64,
}

impl rmm_Node {
    pub fn new() -> rmm_Node{
        rmm_Node {
            excess : 0,
            min_excess : 0,
            max_excess : 0,
            count_bits : 0,
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
         let mut count: u64 = 0;
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
                 heap[currentLeaf].count_bits = blocksize;
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
             count = heap[left].count_bits + heap[right].count_bits;
             heap[x].excess = e;
             heap[x].min_excess = m;
             heap[x].max_excess = M;
             heap[x].count_bits = count;
         }

         heap
    }

    fn rank_1(&self, i: usize) -> Option<usize>{
        if i >= self.bitvector.len() as usize {
            None
        }
        else {
            let mut currentLeaf = (self.tree.len()/2) + (i/self.blocksize);
            let mut startbit = (currentLeaf - self.tree.len()/2) * self.blocksize;
            let mut rank: u64 = 0;
            for j in startbit .. i {
                if self.bitvector[j as u64] {
                    rank += 1;
                }
            }
            let  mut parent = (currentLeaf - 1) /2;
            while parent > 0 {
                let left = 2*parent +1;
                let mut temp = self.tree[left].count_bits as i64 + self.tree[left].excess;
                assert!(temp >= 0);
                rank += temp as u64 / 2;
                let parent = (currentLeaf - 1) /2;
            }
            Some(rank as usize)
        }

    }

    fn rank_0(&self, i: usize) -> Option<usize>{
        match self.rank_1(i) {
            None => None,
            Some(n) => Some (i-n),
        }
    }

    fn select_1(&self, k: usize) -> Option<usize>{
            Some (self.select_1_from_tree(k, 0))


    }

    fn select_1_from_tree(&self, k:usize, node: usize) -> usize{
        if node >= (self.tree.len()/2) {
            let startbit = (node - self.tree.len()/2) * self.blocksize;
            let mut select = 0;
            for j in startbit .. startbit + (self.blocksize-1) {
                if self.bitvector[j as u64] {
                    select += 1;
                }
            }
            select
        }
        else if (self.tree[2*node+1].count_bits as i64 + self.tree[2*node+1].excess)/2 >= k as i64 {
            self.select_1_from_tree(k, 2*node+1)
        }
        else {
            let temp = k as i64 - (self.tree[2 * node + 1 as usize].count_bits as i64 +
          self.tree[2 * node + 1 as usize].excess as i64) / 2 as i64;
            assert!( temp >= 0);
            self.tree[2*node+1 as usize].count_bits as usize + self.select_1_from_tree(temp as usize,
            2 * node + 2 as usize)
        }
    }

    fn select_0(&self, k: usize) -> Option<usize>{
            Some (self.select_0_from_tree(k, 0))
    }

    fn select_0_from_tree(&self, k:usize, node: usize) -> usize{
        if node >= (self.tree.len()/2) {
            let startbit = (node - self.tree.len()/2) * self.blocksize;
            let mut select = 0;
            for j in startbit .. startbit + (self.blocksize-1) {
                if !self.bitvector[j as u64] {
                    select += 1;
                }
            }
            select
        }
        else if (self.tree[2*node+1].count_bits as i64 - (self.tree[2*node+1].count_bits as i64 + self.tree[2*node+1].excess)/2) >= k as i64{
            self.select_1_from_tree(k, 2*node+1)
        }
        else {
            let temp = k as i64 -
                (self.tree[2*node+1 as usize].count_bits as i64 -
                    (self.tree[2*node+1 as usize].count_bits as i64 +
                        self.tree[2*node+1 as usize].excess)/2);
            assert!(temp >= 0);
            self.tree[2*node+1 as usize].count_bits as usize +
            self.select_1_from_tree(temp as usize, 2*node+2)
        }
    }

    fn fwdsearch(&self, i: usize, d: i64) -> Option<usize>{
        unimplemented!();

    }

    fn bwdsearch(&self, i: usize, d: i64) -> Option<usize>{
        unimplemented!();
    }
}

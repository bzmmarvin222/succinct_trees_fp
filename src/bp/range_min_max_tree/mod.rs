extern crate bv;
use self::bv::BitVec;
use std::cmp;

#[derive(Default, Debug, PartialEq)]
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
            min_excess : i64::max_value(),
            max_excess : i64::min_value(),
            count_bits : 0,
        }
    }

}


#[derive(Default)]
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
             _ => (bitvector.len()/blocksize)+1,
         };

         let nodeCount =  match quantity_children.checked_next_power_of_two() {
             Some(n) => 2*n -1,
             None => panic!("Overflow Error"),
         };


         let mut heap: Vec<rmm_Node> = Vec::with_capacity(nodeCount as usize);


         for i in 0 .. heap.capacity(){
             heap.push(rmm_Node::new());
         }


         let mut e: i64 = 0;
         let mut m: i64 = i64::max_value();
         let mut M: i64 = i64::min_value();
         let mut bitcount = 0;
         let mut count: u64 = 0;
         let mut currentLeaf = heap.len()/2;

         for i in 0 .. bitvector.len() {
             bitcount = bitcount +1;
             if bitvector[i] {
                 e += 1;
             }
             else {
                 e -= 1;
             }
             m = cmp::min(e,m);
             M = cmp::max(e,M);
             if (i+1) == bitvector.len(){
                 heap[currentLeaf].excess = e;
                 heap[currentLeaf].min_excess = m;
                 heap[currentLeaf].max_excess = M;
                 heap[currentLeaf].count_bits = bitcount;
             }
             else if ((i+1) % blocksize == 0) {
                 heap[currentLeaf].excess = e;
                 heap[currentLeaf].min_excess = m;
                 heap[currentLeaf].max_excess = M;
                 heap[currentLeaf].count_bits = bitcount;
                 currentLeaf +=1;
                 e=0;
                 if bitvector[i+1] {
                     m=1;
                     M=1;
                 }
                 else {
                     m=-1;
                     M=-1;
                 }
                 bitcount = 0;
             }
         }



         let mut x = heap.len()/2;

         while x > 0 {

             let mut node = x -1;
             let left = 2*node+1;
             let right = 2*node+2;



             e = heap[left].excess + heap[right].excess;
             if (heap[left].min_excess == i64::max_value()) {
                 m = i64::max_value();
                 M = i64::min_value();
             }
             else {
                 let right_min = i64::checked_add(heap[left].excess, heap[right].min_excess).unwrap_or(i64::max_value());
                 m = cmp::min(heap[left].min_excess, right_min);
                 let right_max = i64::checked_add(heap[left].excess, heap[right].max_excess).unwrap_or(i64::min_value());
                 M = cmp::max(heap[left].max_excess, right_max);
             }
             count = heap[left].count_bits + heap[right].count_bits;
             if left >= heap.len() || right >= heap.len() {
                 heap[node].excess = 0;
                 heap[node].min_excess = i64::max_value();
                 heap[node].max_excess = i64::min_value();
                 heap[node].count_bits = 0;
             } else {
                 heap[node].excess = e;
                 heap[node].min_excess = m;
                 heap[node].max_excess = M;
                 heap[node].count_bits = count;
             }
             x =x-1;
         }

         heap
    }

    fn rank_1(&self, i: usize) -> Option<usize>{
        if (i >= self.bitvector.len() as usize) || (i == 0) {
            None
        }
        else {
            let bit = i-1;
            let mut currentLeaf = (self.tree.len()/2) + (bit/self.blocksize);
            let mut j = (currentLeaf - self.tree.len()/2) * self.blocksize;
            let mut rank: u64 = 0;
            while j <= bit {
                if self.bitvector[j as u64] {
                    rank += 1;
                }
                j +=1;
            }
            println!("rank: {:?}", rank);
            while currentLeaf > 0 {
                if currentLeaf % 2 == 0 {
                    let parent = (currentLeaf -1) /2;
                    let left = 2*parent +1;
                    let mut temp = self.tree[left].count_bits as i64 + self.tree[left].excess;
                    assert!(temp >= 0);
                    rank += (temp as u64) / 2;
                }
                currentLeaf = (currentLeaf - 1) /2;
            }
            println!("after");
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
            if k < 1 || k >= (self.bitvector.len()/2) as usize {
                None
            }
            else {
                Some (self.select_1_from_tree(k, 0))
            }

    }

    fn select_1_from_tree(&self, k:usize, node: usize) -> usize{
        if node >= (self.tree.len()/2) {
            let startbit = (node - self.tree.len()/2) * self.blocksize;
            let mut select = 0;
            let mut tmp = 0;
            let mut j = startbit;
            while j <= (startbit + (self.blocksize-1)) {
                select +=1;
                println!("j {:?}", j);
                println!("select {:?}", select);
                if self.bitvector[j as u64] {
                    tmp += 1;
                    if tmp == k {
                        break;
                    }
                }
                j +=1;
            }
            println!("select {:?}", select);
            select
        }
        else if (self.tree[2*node+1].count_bits as i64 + self.tree[2*node+1].excess)/2 >= k as i64 {
            self.select_1_from_tree(k, 2*node+1)
        }
        else {
            let temp = k as i64 - (self.tree[2 * node + 1 as usize].count_bits as i64 +
          self.tree[2 * node + 1 as usize].excess as i64) / 2 as i64;
            assert!( temp >= 0);
            println!("bits left{:?}", self.tree[2*node+1 as usize].count_bits as usize);
            println!("to add{:?}", self.select_1_from_tree(temp as usize,
            2 * node + 2 as usize));
            self.tree[2*node+1 as usize].count_bits as usize + self.select_1_from_tree(temp as usize,
            2 * node + 2 as usize)
        }
    }

    fn select_0(&self, k: usize) -> Option<usize>{
        if k < 1 || k >= (self.bitvector.len()/2) as usize {
            None
        }
        else {
            Some (self.select_0_from_tree(k, 0))
        }
    }

    fn select_0_from_tree(&self, k:usize, node: usize) -> usize{
        if node >= (self.tree.len()/2) {
            let startbit = (node - self.tree.len()/2) * self.blocksize;
            let mut select = 0;
            let mut tmp = 0;
            let mut j = startbit;
            while j <= (startbit + (self.blocksize-1)) {
                select +=1;
                println!("j {:?}", j);
                println!("select {:?}", select);
                if !self.bitvector[j as u64] {
                    tmp += 1;
                    if tmp == k {
                        break;
                    }
                }
                j +=1;
            }
            println!("select {:?}", select);
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

#[cfg(test)]
mod tests;
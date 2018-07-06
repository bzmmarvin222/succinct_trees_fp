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

    pub fn rank_1(&self, i: usize) -> Option<usize>{
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

    pub fn rank_0(&self, i: usize) -> Option<usize>{
        match self.rank_1(i) {
            None => None,
            Some(n) => Some (i-n),
        }
    }

    pub fn select_1(&self, k: usize) -> Option<usize>{
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
                if self.bitvector[j as u64] {
                    tmp += 1;
                    if tmp == k {
                        break;
                    }
                }
                j +=1;
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
            //println!("bits left{:?}", self.tree[2*node+1 as usize].count_bits as usize);
            //println!("to add{:?}", self.select_1_from_tree(temp as usize,
//            2 * node + 2 as usize));
            self.tree[2*node+1 as usize].count_bits as usize + self.select_1_from_tree(temp as usize,
            2 * node + 2 as usize)
        }
    }

    pub fn select_0(&self, k: usize) -> Option<usize>{
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
                if !self.bitvector[j as u64] {
                    tmp += 1;
                    if tmp == k {
                        break;
                    }
                }
                j +=1;
            }
            select
        }
        else if (self.tree[2*node+1].count_bits as i64 - (self.tree[2*node+1].count_bits as i64 + self.tree[2*node+1].excess)/2) >= k as i64{

            self.select_0_from_tree(k, 2*node+1)
        }
        else {

            let temp = k as i64 -
                (self.tree[2*node+1 as usize].count_bits as i64 -
                    (self.tree[2*node+1 as usize].count_bits as i64 +
                        self.tree[2*node+1 as usize].excess)/2);
            assert!(temp >= 0);
            self.tree[2*node+1 as usize].count_bits as usize +
            self.select_0_from_tree(temp as usize, 2*node+2)
        }
    }

    pub fn fwdsearch(&self, mut i: usize, mut d: i64) -> Option<usize>{
        if i<=0 || i >= self.bitvector.len() as usize{
            None
        }
        else {
            i -=1;
            let mut pos = i;
            let k = match i%self.blocksize {
                    0 => i / self.blocksize,
                    _ => i / self.blocksize +1,
            };
            for j in (i+1) .. (k*self.blocksize) {
                if self.bitvector[j as u64] {
                    d -= 1;
                }
                else {
                    d +=1;
                }
                if d == 0 {
                    return Some(j)
                }
            }

            let mut node = (self.tree.len()/2 + k -1) as usize;
            return self.step2(node, i, d);
        }
    }

    fn step2(&self, mut node: usize, i: usize, mut d: i64) -> Option<usize> {
        if node <= 0 {
            None
        }
        else if node % 2 == 0{
            self.step2((node-1)/2, i, d)
        }
        else {
            let mut right = node +1;
            if (self.tree[right].min_excess <= d) && (d <= self.tree[right].max_excess) {
                self.step3(right, i, d)
            }
            else {
                d = d - self.tree[right].excess;
                self.step2((node-1)/2, i, d)
            }

        }
    }

    fn step3(&self, mut node: usize, i: usize, mut d: i64) -> Option<usize> {
        if node >= (self.tree.len()/2) as usize{
            let mut pos = (node - self.tree.len()/2)*self.blocksize;
            while d != 0 {
                if self.bitvector[pos as u64] {
                    d -= 1;
                }
                else {
                    d += 1;
                }
                pos+=1;
            }
            Some(pos)
        }
        else {
            let lc = 2*node +1;
            let rc = 2*node +2;
            if (self.tree[lc].min_excess <= d) && (d <= self.tree[lc].max_excess) {
                self.step3(lc, i, d)
            }
            else {
                d = d - self.tree[lc].excess;
                self.step3(rc, i, d)
            }
        }
    }

    pub fn bwdsearch(&self, i: usize, d: i64) -> Option<usize>{
        unimplemented!();
    pub fn bwdsearch(&self, mut i: usize, mut d: i64) -> Option<usize>{
        if i<=0 || i >= self.bitvector.len() as usize{
            None
        }
        else {
            i -=1;
            let mut pos = i;
            let k = match i%self.blocksize {
                    0 => i / self.blocksize,
                    _ => i / self.blocksize +1,
            };
            let j = i-1;
            while j >= k*self.blocksize -(self.blocksize-1) {
                if self.bitvector[j as u64] {
                    d -= 1;
                }
                else {
                    d +=1;
                }
                if d == 0 {
                    return Some(j)
                }
                j -= 1;
            }

            let mut node = (self.tree.len()/2 + k -1) as usize;
            return self.bstep2(node, i, d);
        }
    }

    fn bstep2(&self, mut node: usize, i: usize, mut d: i64) -> Option<usize> {
        if node <= 0 {
            None
        }
        else if node % 2 == 1{
            self.step2((node-1)/2, i, d)
        }
        else {
            let mut left = node -1;
            let mut tmp = 0;
            if self.bitvector[i as u64] {
                tmp = -1;
            }
            else {
                tmp = 1;
            }
            if (self.tree[left].min_excess- self.tree[left].excess + tmp <= d) && (d <= self.tree[left].max_excess - self.tree[left].excess +tmp) {
                self.bstep3(left, i, d)
            }
            else {
                d = d - self.tree[left].excess;
                self.bstep2((node-1)/2, i, d)
            }

        }
    }

    fn bstep3(&self, mut node: usize, i: usize, mut d: i64) -> Option<usize> {
        if node >= (self.tree.len()/2) as usize{
            let mut pos = (node - self.tree.len()/2)*self.blocksize;
            while d != 0 {
                if self.bitvector[pos as u64] {
                    d -= 1;
                }
                else {
                    d += 1;
                }
                pos+=1;
            }
            Some(pos)
        }
        else {
            let lc = 2*node +1;
            let rc = 2*node +2;
            let mut tmp = 0;
            if self.bitvector[i as u64] {
                tmp = -1;
            }
            else {
                tmp = 1;
            }
            if (self.tree[rc].min_excess - self.tree[rc].excess + tmp <= d) && (d <= self.tree[rc].max_excess - self.tree[rc].excess + tmp ) {
                self.bstep3(lc, i, d)
            }
            else {
                d = d - self.tree[rc].excess;
                self.bstep3(rc, i, d)
            }
        }
    }
}

#[cfg(test)]
mod tests;

use super::*;
use super::bv::*;
use bp::range_min_max_tree::*;
use std::cmp;

#[test]
fn test_build_from_bv() {
    let bitvec = bit_vec![true, true, false, true, false, false];
    let node_1 = rmm_Node {
        excess : 2,
        min_excess : 1,
        max_excess : 2,
        count_bits : 2,};

    let node_2 = rmm_Node {
        excess : 0,
        min_excess : -1,
        max_excess : 0,
        count_bits : 2,};

    let node_3 = rmm_Node {
        excess : -2,
        min_excess : -2,
        max_excess : -1,
        count_bits : 2,};

    let node_4 = rmm_Node {
        excess : 0,
        min_excess : 0,
        max_excess : 0,
        count_bits : 0,};

    let node_5 = rmm_Node {
        excess : 2,
        min_excess : 1,
        max_excess : 2,
        count_bits : 4,};

    let node_6 = rmm_Node {
        excess : -2,
        min_excess : 1,
        max_excess : -1,
        count_bits : 2,};


    let node_7 = rmm_Node {
        excess : 0,
        min_excess : 0,
        max_excess : 2,
        count_bits : 6,};

    let expected = vec![node_7, node_5, node_6, node_1, node_2, node_3, node_4];


    let tree = RangeMinMaxTree::new(bitvec, 3);
    assert_eq!(tree.tree, expected)


}

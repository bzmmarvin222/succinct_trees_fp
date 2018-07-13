use super::*;
use super::bv::*;
use bp::range_min_max_tree::*;
use std::cmp;


#[test]
fn test_build_tree() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let node_0 = rmm_Node {
        excess : 0,
        min_excess : 0,
        max_excess : 4,
        count_bits : 22,};

    let node_1 = rmm_Node {
        excess : 4,
        min_excess : 1,
        max_excess : 4,
        count_bits : 16,};

    let node_2 = rmm_Node {
        excess : -4,
        min_excess : -4,
        max_excess : 0,
        count_bits : 6,};

    let node_3 = rmm_Node {
        excess : 4,
        min_excess : 1,
        max_excess : 4,
        count_bits : 8,};

    let node_4 = rmm_Node {
        excess : 0,
        min_excess : -3,
        max_excess : 0,
        count_bits : 8,};

    let node_5 = rmm_Node {
        excess : -4,
        min_excess : -4,
        max_excess : 0,
        count_bits : 6,};

    let node_6 = rmm_Node {
        excess : 0,
        min_excess : i64::max_value(),
        max_excess : i64::min_value(),
        count_bits : 0,};

    let node_7 = rmm_Node {
        excess : 2,
        min_excess : 1,
        max_excess : 3,
        count_bits : 4,};

    let node_8 = rmm_Node {
        excess : 2,
        min_excess : 0,
        max_excess : 2,
        count_bits : 4,};

    let node_9 = rmm_Node {
        excess : -2,
        min_excess : -3,
        max_excess : -1,
        count_bits : 4,};

    let node_10 = rmm_Node {
        excess : 2,
        min_excess : -1,
        max_excess : 2,
        count_bits : 4,};

    let node_11 = rmm_Node {
        excess : -2,
        min_excess : -2,
        max_excess : 0,
        count_bits : 4,};

    let node_12 = rmm_Node {
        excess : -2,
        min_excess : -2,
        max_excess : -1,
        count_bits : 2,};

    let node_13 = rmm_Node {
        excess : 0,
        min_excess : i64::max_value(),
        max_excess : i64::min_value(),
        count_bits : 0,};

    let node_14 = rmm_Node {
        excess : 0,
        min_excess : i64::max_value(),
        max_excess : i64::min_value(),
        count_bits : 0,};


    let expected = vec![node_0, node_1, node_2, node_3, node_4, node_5, node_6, node_7, node_8, node_9, node_10, node_11, node_12, node_13, node_14];


    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.tree, expected)
}

#[test]
fn nonvalid_i_rank_1() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.rank_1(u64::max_value() as usize), None);
}

#[test]
fn nonvalid_i_is_zero_rank_1() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.rank_1(0), None);
}

#[test]
fn valid_rank_1() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.rank_1(11).unwrap(), 6);
}

#[test]
fn nonvalid_i_rank_0() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.rank_0(u64::max_value() as usize), None);
}

#[test]
fn nonvalid_i_is_zero_rank_0() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.rank_0(0), None);
}

#[test]
fn valid_rank_0() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.rank_0(11).unwrap(), 5);
}

#[test]
fn valid_select_1() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.select_1(7).unwrap(),12);
}

#[test]
fn nonvalid_i_is_zero_select_1() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.select_1(0), None);
}

#[test]
fn nonvalid_i_select_1() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.select_1(u64::max_value() as usize), None);
}

#[test]
fn valid_select_0() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.select_0(7).unwrap(),17);
}

#[test]
fn valid_fwdsearch() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.fwdsearch(6,-2).unwrap(), 22);
}

#[test]
fn invalid_fwdsearch_i() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.fwdsearch(90,-2), None);
}

#[test]
fn invalid_fwdsearch_d() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.fwdsearch(6,i64::min_value()/2), None);
}

#[test]
fn valid_bwdsearch() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.bwdsearch(17,-2).unwrap(), 14);
}

#[test]
fn valid_bwdsearch2() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.bwdsearch(4,1).unwrap(), 3);
}

#[test]
fn invalid_bwdsearch_i() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.bwdsearch(90,-2), None);
}

#[test]
fn invalid_bwdsearch_d() {
    let bitvec = bit_vec![true, true, true, false,
                        true, false, true, true,
                        false, false, false, true,
                        false, true, true, true,
                        false, true, false, false,
                        false, false];
    let tree = RangeMinMaxTree::new(bitvec, 4);
    assert_eq!(tree.bwdsearch(6,i64::min_value()/2), None);
}

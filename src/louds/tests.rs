use super::*;
use super::bv::*;

//serialize deserialize test


// parenthesis sequence form test

#[test]
#[should_panic]
fn test_from_binary_representation() {
    let louds = LOUDS::from_binary_representation(String::from("10"));
    let expected_vec = bit_vec![true, false];
    assert_eq!(louds.vec, expected_vec);

    let louds = LOUDS::from_binary_representation(String::from("1110100100"));
    let expected_vec = bit_vec![true, true, true, false, true, false, false, true, false, false];
    assert_eq!(louds.vec, expected_vec);

    let louds_panic = LOUDS::from_binary_representation(String::from("10abcd"));
}

#[test]
fn test_binary_representation() {
    let vec = bit_vec![true, true, true, false, true, false, false, true, false, false];
    let louds = LOUDS::new(vec);
    let rep = louds.binary_representation();
    assert_eq!(String::from("1110100100"), rep);

    let vec = bit_vec![true, false];
    let louds = LOUDS::new(vec);
    let rep = louds.binary_representation();
    assert_eq!(String::from("10"), rep);
}

#[test]
fn test_prev_0(){
    // prev0 works not correct if param is 0
//    111101010000
//    prev0(4)=select0(rank0(4))
//    rank0(4)=1
//    select0(1)=4
}

#[test]
fn test_next_0(){
}

#[test]
fn test_index_represents_node(){}

#[test]
fn test_is_leaf(){}

#[test]
fn test_child_rank(){}

#[test]
fn test_next_sibling(){}

#[test]
fn test_degree(){}

#[test]
fn test_parent(){}

#[test]
fn test_child(){}

#[test]
fn test_first_child(){}

#[test]
fn test_ancestor(){}

#[test]
fn test_depth(){}

#[test]
fn test_subtree_size(){}

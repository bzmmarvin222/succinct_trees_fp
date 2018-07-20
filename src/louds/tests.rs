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
    let louds1 = LOUDS::from_binary_representation(String::from("111101010000"));
    assert_eq!(louds1.prev_0(4),4);
    assert_eq!(louds1.prev_0(7),6);
    //    prev0(4)=select0(rank0(4))
    //    rank0(4)=1
    //    select0(1)=4
}

#[test]
fn test_next_0(){
    let louds1 = LOUDS::from_binary_representation(String::from("111101010000"));
    assert_eq!(louds1.next_0(8),9);
    assert_eq!(louds1.next_0(2),4);
}

#[test]
fn test_index_represents_node(){
    let louds1 = LOUDS::from_binary_representation(String::from("111101010000"));
    assert_eq!(louds1.index_represents_node(0),false);
    assert_eq!(louds1.index_represents_node(1),true);
    assert_eq!(louds1.index_represents_node(4),false);
    assert_eq!(louds1.index_represents_node(9),true);
    assert_eq!(louds1.index_represents_node(7),true);
    assert_eq!(louds1.index_represents_node(8),false);

}

#[test]
fn test_is_leaf(){
    let louds1 = LOUDS::from_binary_representation(String::from("111101010000"));
    assert_eq!(louds1.is_leaf(1),false);
    assert_eq!(louds1.is_leaf(10),true);
}

#[test]
fn test_child_rank(){
    let louds1 = LOUDS::from_binary_representation(String::from("111101010000"));
    assert_eq!(louds1.child_rank(1),0);
    assert_eq!(louds1.child_rank(9),2);
    assert_eq!(louds1.child_rank(10),0);
    assert_eq!(louds1.child_rank(7),1);
}

#[test]
fn test_next_sibling(){
    let louds1 = LOUDS::from_binary_representation(String::from("111101010000"));
    assert_eq!(louds1.next_sibling(1),None);
    assert_eq!(louds1.next_sibling(10),None);
    assert_eq!(louds1.next_sibling(5),Some(7));
    assert_eq!(louds1.next_sibling(7),Some(9));
}

#[test]
fn test_degree(){
    let louds1 = LOUDS::from_binary_representation(String::from("111101010000"));
    assert_eq!(louds1.degree(1),Some(3));
    assert_eq!(louds1.degree(7),Some(1));
}

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

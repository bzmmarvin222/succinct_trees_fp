use super::*;
use super::bv::*;

#[test]
fn test_serialize_deserialize() {
    let bp = BalancedParentheses::new(BitVec::new_fill(true, 8));
    let serialized = bp.serialize();
    let deserialized = BalancedParentheses::deserialize(serialized);
    assert_eq!(deserialized, bp);
}

#[test]
fn test_from_braces_representation() {
    let bp = BalancedParentheses::from_braces_representation(String::from("()"));
    let expected_vec = bit_vec![true, false];
    assert_eq!(bp.vec, expected_vec);

    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    let expected_vec = bit_vec![true, true, true, false, true, false, false, true, false, false];
    assert_eq!(bp.vec, expected_vec);
}

#[test]
#[should_panic]
fn test_from_braces_representation_panic() {
    let bp_panic = BalancedParentheses::from_braces_representation(String::from("()abcd"));
}

#[test]
fn test_braces_representation() {
    let vec = bit_vec![true, true, true, false, true, false, false, true, false, false];
    let bp = BalancedParentheses::new(vec);
    let rep = bp.braces_representation();
    assert_eq!(String::from("((()())())"), rep);

    let vec = bit_vec![true, false];
    let bp = BalancedParentheses::new(vec);
    let rep = bp.braces_representation();
    assert_eq!(String::from("()"), rep);
}

#[test]
fn test_is_leaf() {
    let bp = BalancedParentheses::from_braces_representation(String::from("()"));
    assert_eq!(Some(true), bp.is_leaf(0));
    assert_eq!(None, bp.is_leaf(2));
    assert_eq!(None, bp.is_leaf(1));

    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    assert_eq!(Some(true), bp.is_leaf(2));
    assert_eq!(Some(false), bp.is_leaf(1));
}

#[test]
fn test_has_index() {
    let bp = BalancedParentheses::from_braces_representation(String::from("()"));
    assert_eq!(true, bp.has_index(1));
    assert_eq!(false, bp.has_index(2));
}

#[test]
fn test_index_represents_node() {
    let bp = BalancedParentheses::from_braces_representation(String::from("()"));
    assert_eq!(true, bp.index_represents_node(0));
    assert_eq!(false, bp.index_represents_node(1));

    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    assert_eq!(true, bp.index_represents_node(2));
    assert_eq!(false, bp.index_represents_node(3));
}

#[test]
fn test_first_child() {
    let bp = BalancedParentheses::from_braces_representation(String::from("()"));
    assert_eq!(None, bp.first_child(0));
    assert_eq!(None, bp.first_child(1));

    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    assert_eq!(None, bp.first_child(2));
    assert_eq!(Some(2), bp.first_child(1));
}

#[test]
fn test_find_close() {
    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    assert_eq!(Some(9), bp.find_close(0));
    assert_eq!(Some(3), bp.find_close(2));
    assert_eq!(Some(5), bp.find_close(4));
    assert_eq!(Some(8), bp.find_close(7));
    assert_eq!(None, bp.find_close(30));
}

#[test]
fn test_ancestor() {
    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    assert_eq!(Some(true), bp.ancestor(0, 1));
    assert_eq!(Some(true), bp.ancestor(1, 2));
    assert_eq!(Some(false), bp.ancestor(1, 7));
    assert_eq!(None, bp.ancestor(1, 20));
}

#[test]
fn test_next_sibling() {
    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    assert_eq!(None, bp.next_sibling(20));
    assert_eq!(Some(4), bp.next_sibling(2));
    assert_eq!(None, bp.next_sibling(4));
}

#[test]
fn test_parent() {
    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    println!("test: {:?}", bp.rmm.bwdsearch(1, -2));
    println!("test: {:?}", bp.rmm.bwdsearch(2, -2));
    println!("test: {:?}", bp.rmm.bwdsearch(3, -2));
    println!("test: {:?}", bp.rmm.bwdsearch(4, -2));
    println!("test: {:?}", bp.rmm.bwdsearch(5, -2));
    assert_eq!(Some(1), bp.parent(4));
    assert_eq!(Some(0), bp.parent(1));
    assert_eq!(None, bp.parent(0));
}

#[test]
fn test_subtree_size() {
    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    assert_eq!(Some(5), bp.subtree_size(0));
    assert_eq!(Some(3), bp.subtree_size(1));
    assert_eq!(None, bp.subtree_size(20));
}

#[test]
fn test_depth() {
    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    assert_eq!(Some(0), bp.depth(0));
    assert_eq!(Some(1), bp.depth(1));
    assert_eq!(Some(2), bp.depth(2));
    assert_eq!(Some(2), bp.depth(4));
}

#[test]
fn test_child() {
    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    assert_eq!(Some(1), bp.child(0, 0));
    assert_eq!(Some(7), bp.child(0, 1));
    assert_eq!(None, bp.child(30, 0));
    assert_eq!(None, bp.child(0, 4));
}

#[test]
fn test_degree() {
    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    assert_eq!(Some(2), bp.degree(0));
    assert_eq!(Some(2), bp.degree(1));
    assert_eq!(Some(0), bp.degree(2));
    assert_eq!(None, bp.degree(30));
}

#[test]
fn test_child_rank() {
    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    assert_eq!(Some(0), bp.child_rank(2));
    assert_eq!(Some(1), bp.child_rank(4));
    assert_eq!(None, bp.child_rank(0));
}

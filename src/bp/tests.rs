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
#[should_panic]
fn test_from_braces_representation() {
    let bp = BalancedParentheses::from_braces_representation(String::from("()"));
    let expected_vec = bit_vec![true, false];
    assert_eq!(bp.vec, expected_vec);

    let bp = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    let expected_vec = bit_vec![true, true, true, false, true, false, false, true, false, false];
    assert_eq!(bp.vec, expected_vec);

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
fn test_rank_closing_brace(){
    let balanced_parenthesis1 = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    /*
                0
               / \
              1  7
             / \
            2  4

    */
    assert_eq!(2, balanced_parenthesis1.rank_closing_brace(5));
    // what if balanced_parenthesis1.rank_0(25)??

    let balanced_parenthesis2 = BalancedParentheses::from_braces_representation(String::from("()"));
    assert_eq!(0, balanced_parenthesis2.rank_closing_brace(0));
    // what if balanced_parenthesis2.rank_0(6)??

    let balanced_parenthesis3 = BalancedParentheses::from_braces_representation(String::from(""));
    // what if balanced_parenthesis3.rank_0(30)??
    // what if balanced_parenthesis3.rank_0(0)??

    let balanced_parenthesis4 = BalancedParentheses::from_braces_representation(String::from("(()()()()())"));
    /*
                0
            /  / \ \ \
           1  3  5 7 9
    */
    assert_eq!(2, balanced_parenthesis4.rank_closing_brace(4));
    assert_eq!(2, balanced_parenthesis4.rank_closing_brace(5));
    // what if balanced_parenthesis4.rank_0(13)??
    // what if balanced_parenthesis4.rank_0(42)??
}

#[test]
fn test_rank_opening_brace() {
    let balanced_parenthesis1 = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    /*
                0
               / \
              1  7
             / \
            2  4

    */
    assert_eq!(5, balanced_parenthesis1.rank_opening_brace(7));
    assert_eq!(4, balanced_parenthesis1.rank_opening_brace(6));
    // what if balanced_parenthesis1.rank_1(25)??

    let balanced_parenthesis2 = BalancedParentheses::from_braces_representation(String::from("()"));
    assert_eq!(0, balanced_parenthesis2.rank_opening_brace(0));
    // what if balanced_parenthesis2.rank_1(2)??

    let balanced_parenthesis3 = BalancedParentheses::from_braces_representation(String::from(""));
    // what if balanced_parenthesis3.rank_1(30)??
    // what if balanced_parenthesis3.rank_1(0)??
    let balanced_parenthesis4 = BalancedParentheses::from_braces_representation(String::from("(()()()()())"));
    /*
                0
            /  / \ \ \
           1  3  5 7 9
    */
    assert_eq!(1, balanced_parenthesis4.rank_opening_brace(1));
    // what if balanced_parenthesis4.rank_1(13)??
    // what if balanced_parenthesis4.rank_1(42)??
}

#[test]
fn test_find_close(){
    let balanced_parenthesis1 = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    /*
                0
               / \
              1  7
             / \
            2  4

    */
    assert_eq!(6, balanced_parenthesis1.find_close(0));
    assert_eq!(9, balanced_parenthesis1.find_close(1));
    assert_eq!(8, balanced_parenthesis1.find_close(7));
    // what if balanced_parenthesis1.rank0(25)??

    let balanced_parenthesis2 = BalancedParentheses::from_braces_representation(String::from("()"));
    assert_eq!(0, balanced_parenthesis2.find_close(0));
    // what if balanced_parenthesis2.find_close(2)??


    let balanced_parenthesis3 = BalancedParentheses::from_braces_representation(String::from(""));
    // what if balanced_parenthesis3.find_close(30)??
    // what if balanced_parenthesis3.find_close(0)??

    let balanced_parenthesis4 = BalancedParentheses::from_braces_representation(String::from("(()()()()())"));
    /*
                0
            /  / \ \ \
           1  3  5 7 9
    */
    assert_eq!(11, balanced_parenthesis4.find_close(0));
    assert_eq!(10, balanced_parenthesis4.find_close(9));
    // what if balanced_parenthesis4.find_close(13)??
    // what if balanced_parenthesis4.find_close(42)??

}

#[test]
fn test_index_represents_node(){
    let balanced_parenthesis1 = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    /*
                0
               / \
              1  7
             / \
            2  4

    */
    assert_eq!(true, balanced_parenthesis1.represents_node(0));
    assert_eq!(true, balanced_parenthesis1.represents_node(4));
    assert_eq!(true, balanced_parenthesis1.represents_node(1));
    assert_eq!(false, balanced_parenthesis1.represents_node(5));
    assert_eq!(false, balanced_parenthesis1.represents_node(8));
    // what if balanced_parenthesis1.represents_node(25)??
    // what if balanced_parenthesis1.represents_node(-3)??
    //will negative number interpreted as positive?

    let balanced_parenthesis2 = BalancedParentheses::from_braces_representation(String::from("()"));
    assert_eq!(true, balanced_parenthesis2.represents_node(0));
    assert_eq!(false, balanced_parenthesis2.represents_node(1));
    // what if balanced_parenthesis2.represents_node(2)??

    let balanced_parenthesis3 = BalancedParentheses::from_braces_representation(String::from(""));
    // what if balanced_parenthesis3.represents_node(30)??
    // what if balanced_parenthesis3.represents_node(0)??

    let balanced_parenthesis4 = BalancedParentheses::from_braces_representation(String::from("(()()()()())"));
    /*
                0
            /  / \ \ \
           1  3  5 7 9
    */
    assert_eq!(true, balanced_parenthesis4.represents_node(0));
    assert_eq!(true, balanced_parenthesis4.represents_node(5));
    assert_eq!(false, balanced_parenthesis4.represents_node(6));
    // what if balanced_parenthesis4.represents_node(13)??
    // what if balanced_parenthesis4.represents_node(42)??
}

#[test]
fn test_is_leaf(){
    let balanced_parenthesis1 = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    /*
                0
               / \
              1  7
             / \
            2  4

    */
    assert_eq!(true, balanced_parenthesis1.is_leaf(7));
    assert_eq!(false, balanced_parenthesis1.is_leaf(1));
    // what if balanced_parenthesis1.is_leaf(25)??
    // what if balanced_parenthesis1.is_leaf(5)??

    let balanced_parenthesis2 = BalancedParentheses::from_braces_representation(String::from("()"));
    assert_eq!(false, balanced_parenthesis2.is_leaf(0));
    // what if balanced_parenthesis2.is_leaf(2)??
    // what if balanced_parenthesis2.is_leaf(1)??

    let balanced_parenthesis3 = BalancedParentheses::from_braces_representation(String::from(""));
    // what if balanced_parenthesis3.is_leaf(30)??
    // what if balanced_parenthesis3.is_leaf(0)??

    let balanced_parenthesis4 = BalancedParentheses::from_braces_representation(String::from("(()()()()())"));
    /*
                0
            /  / \ \ \
           1  3  5 7 9
    */
    assert_eq!(true, balanced_parenthesis4.is_leaf(3));
    assert_eq!(false, balanced_parenthesis4.is_leaf(0));
    // what if balanced_parenthesis4.is_leaf(13)??
    // what if balanced_parenthesis4.is_leaf(42)??
    // what if balanced_parenthesis4.is_leaf(8)??
}

#[test]
fn test_ancestor(){
    let balanced_parenthesis1 = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    /*
                0
               / \
              1  7
             / \
            2  4

    */
    assert_eq!(true, balanced_parenthesis1.ancestor(1,2));
    assert_eq!(true, balanced_parenthesis1.ancestor(0,4));
    assert_eq!(false, balanced_parenthesis1.ancestor(2,1));
    assert_eq!(false, balanced_parenthesis1.ancestor(4,7));
    // what if balanced_parenthesis1.ancestor(7, 25)??
    // what if balanced_parenthesis1.ancestor(3, 5)??
    // what if balanced_parenthesis1.ancestor(1, 5)??

    let balanced_parenthesis2 = BalancedParentheses::from_braces_representation(String::from("()"));
    assert_eq!(false, balanced_parenthesis2.ancestor(0,0));
    // what if balanced_parenthesis2.ancestor(0,2)??
    // what if balanced_parenthesis2.ancestor(9,2)??

    let balanced_parenthesis3 = BalancedParentheses::from_braces_representation(String::from(""));
    // what if balanced_parenthesis3.ancestor(0,8)??
    // what if balanced_parenthesis3.ancestor(0,0)??

    let balanced_parenthesis4 = BalancedParentheses::from_braces_representation(String::from("(()()()()())"));
    /*
                0
            /  / \ \ \
           1  3  5 7 9
    */
    assert_eq!(true, balanced_parenthesis4.ancestor(0,5));
    assert_eq!(true, balanced_parenthesis4.ancestor(0,9));
    assert_eq!(false, balanced_parenthesis4.ancestor(9,3));
    assert_eq!(false, balanced_parenthesis4.ancestor(3,0));
    // what if balanced_parenthesis4.ancestor(9,42)??
    // what if balanced_parenthesis4.ancestor(9,2)??
    // what if balanced_parenthesis4.ancestor(6,0)??
    // what if balanced_parenthesis4.ancestor(13,1)??
}

#[test]
fn test_parent(){
    let balanced_parenthesis1 = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    /*
                0
               / \
              1  7
             / \
            2  4

    */
    assert_eq!(0, balanced_parenthesis1.parent(7));
    assert_eq!(1, balanced_parenthesis1.parent(4));
    assert_eq!(1, balanced_parenthesis1.parent(2));
    assert_eq!(0, balanced_parenthesis1.parent(1));
    // what if balanced_parenthesis1.parent(25)??
    // what if balanced_parenthesis1.parent(5)??


    let balanced_parenthesis2 = BalancedParentheses::from_braces_representation(String::from("()"));
    // what if balanced_parenthesis2.parent(0)??
    // what if balanced_parenthesis2.parent(2)??

    let balanced_parenthesis3 = BalancedParentheses::from_braces_representation(String::from(""));
    // what if balanced_parenthesis3.parent(8)??
    // what if balanced_parenthesis3..parent(0)??

    let balanced_parenthesis4 = BalancedParentheses::from_braces_representation(String::from("(()()()()())"));
    /*
                0
            /  / \ \ \
           1  3  5 7 9
    */
    assert_eq!(0, balanced_parenthesis4.parent(5));
    assert_eq!(0, balanced_parenthesis4.parent(9));
    // what if balanced_parenthesis4.parent(42)??
    // what if balanced_parenthesis4.parent(13)??
    // what if balanced_parenthesis4.parent(8)??
}

fn test_first_child(){
    let balanced_parenthesis1 = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    /*
                0
               / \
              1  7
             / \
            2  4

    */
    assert_eq!(1, balanced_parenthesis1.first_child(0));
    assert_eq!(2, balanced_parenthesis1.first_child(1));
    // what if balanced_parenthesis1.first_child(4)??
    // what if balanced_parenthesis1.first_child(25)??
    // what if balanced_parenthesis1.first_child(5)??

    let balanced_parenthesis2 = BalancedParentheses::from_braces_representation(String::from("()"));
    // what if balanced_parenthesis2.first_child(2)??
    // what if balanced_parenthesis2.first_child(0)??
    // what if balanced_parenthesis2.first_child(1)??

    let balanced_parenthesis3 = BalancedParentheses::from_braces_representation(String::from(""));
    // what if balanced_parenthesis3.first_child(8)??
    // what if balanced_parenthesis3.first_child(0)??

    let balanced_parenthesis4 = BalancedParentheses::from_braces_representation(String::from("(()()()()())"));
    /*
                0
            /  / \ \ \
           1  3  5 7 9
    */
    assert_eq!(1, balanced_parenthesis4.first_child(0));
    // what if balanced_parenthesis2.first_child(5)??
    // what if balanced_parenthesis4.first_child(42)??
    // what if balanced_parenthesis4.first_child(13)??
    // what if balanced_parenthesis4.first_child(8)??
}

#[test]
fn test_subtree_size(){
    let balanced_parenthesis1 = BalancedParentheses::from_braces_representation(String::from("((()())())"));
    /*
                0
               / \
              1  7
             / \
            2  4

    */
    assert_eq!(5, balanced_parenthesis1.subtree_size(0));
    assert_eq!(3, balanced_parenthesis1.subtree_size(1));
    // what if balanced_parenthesis1.subtree_size(25)??
    // what if balanced_parenthesis1.subtree_size(5)??

    let balanced_parenthesis2 = BalancedParentheses::from_braces_representation(String::from("()"));
    assert_eq!(false, balanced_parenthesis2.ancestor(0,0));
    // what if balanced_parenthesis2.subtree_size(0)??
    // what if balanced_parenthesis2.subtree_size(2)??
    // what if balanced_parenthesis2.subtree_size(1)??

    let balanced_parenthesis3 = BalancedParentheses::from_braces_representation(String::from(""));
    // what if balanced_parenthesis3.subtree_size(0)??
    // what if balanced_parenthesis3.subtree_size(8)??

    let balanced_parenthesis4 = BalancedParentheses::from_braces_representation(String::from("(()()()()())"));
    /*
                0
            /  / \ \ \
           1  3  5 7 9
    */
    assert_eq!(6, balanced_parenthesis4.subtree_size(0));
    assert_eq!(1, balanced_parenthesis4.subtree_size(9));
    // what if balanced_parenthesis4.subtree_size(42)??
    // what if balanced_parenthesis4.subtree_size(13)??
    // what if balanced_parenthesis4.subtree_size(8)??
}

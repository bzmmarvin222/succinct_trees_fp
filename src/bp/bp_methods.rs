#[cfg(test)]
mod tests {
    #[test]


//assume make_bp_from_0_1_string works correct


fn test_rank0(){
    let balanced_parenthesis1 = make_bp_from_0_1_string("1110100100");
    /*
                0
               / \
              1  7
             / \
            2  4

    */
    assert_eq(2, balanced_parenthesis1.rank0(5));
    // what if balanced_parenthesis1.rank0(25)??

    let balanced_parenthesis2 = make_bp_from_0_1_string("10");
    assert_eq(0, balanced_parenthesis2.rank0(0));
    // what if balanced_parenthesis2.rank0(6)??

    let balanced_parenthesis3 = make_bp_from_0_1_string("");
    // what if balanced_parenthesis3.rank0(30)??
    // what if balanced_parenthesis3.rank0(0)??

    let balanced_parenthesis4 = make_bp_from_0_1_string("110101010100");
    /*
                0
            /  / \ \ \
           1  3  5 7 9
    */
    assert_eq(2, balanced_parenthesis4.rank0(4));
    assert_eq(2, balanced_parenthesis4.rank0(5));
    // what if balanced_parenthesis4.rank0(13)??
    // what if balanced_parenthesis4.rank0(42)??
}








fn test_rank1(){
    let balanced_parenthesis1 = make_bp_from_0_1_string("1110100100");
    /*
                0
               / \
              1  7
             / \
            2  4

    */
    assert_eq(5, balanced_parenthesis1.rank1(7));
    assert_eq(4, balanced_parenthesis1.rank1(6));
    // what if balanced_parenthesis1.rank1(25)??

    let balanced_parenthesis2 = make_bp_from_0_1_string("10");
    assert_eq(0, balanced_parenthesis2.rank1(0));
    // what if balanced_parenthesis2.rank1(2)??

    let balanced_parenthesis3 = make_bp_from_0_1_string("");
    // what if balanced_parenthesis3.rank1(30)??
    // what if balanced_parenthesis3.rank1(0)??

    let balanced_parenthesis4 = make_bp_from_0_1_string("110101010100");
    /*
                0
            /  / \ \ \
           1  3  5 7 9
    */
    assert_eq(1, balanced_parenthesis4.rank1(1));
    // what if balanced_parenthesis4.rank1(13)??
    // what if balanced_parenthesis4.rank1(42)??
}




fn test_findclose(){
    let balanced_parenthesis1 = make_bp_from_0_1_string("1110100100");
    /*
                0
               / \
              1  7
             / \
            2  4

    */
    assert_eq(6, balanced_parenthesis1.findclose(0));
    assert_eq(9, balanced_parenthesis1.findclose(1));
    assert_eq(8, balanced_parenthesis1.findclose(7));
    // what if balanced_parenthesis1.rank0(25)??

    let balanced_parenthesis2 = make_bp_from_0_1_string("10");
    assert_eq(0, balanced_parenthesis2.findclose(0));
    // what if balanced_parenthesis2.findclose(2)??


    let balanced_parenthesis3 = make_bp_from_0_1_string("");
    // what if balanced_parenthesis3.findclose(30)??
    // what if balanced_parenthesis3.findclose(0)??

    let balanced_parenthesis4 = make_bp_from_0_1_string("110101010100");
    /*
                0
            /  / \ \ \
           1  3  5 7 9
    */
    assert_eq(11, balanced_parenthesis4.findclose(0));
    assert_eq(10, balanced_parenthesis4.findclose(9));
    // what if balanced_parenthesis4.findclose(13)??
    // what if balanced_parenthesis4.findclose(42)??

}





//from here x, y are not valid arguments if they do not represent a std_unicode
//so at index x should start a new node, so there should stay 1.



fn test_is_leaf(){
    let balanced_parenthesis1 = make_bp_from_0_1_string("1110100100");
    /*
                0
               / \
              1  7
             / \
            2  4

    */
    assert_eq(true, balanced_parenthesis1.is_leaf(7));
    assert_eq(false, balanced_parenthesis1.is_leaf(1));
    // what if balanced_parenthesis1.is_leaf(25)??
    // what if balanced_parenthesis1.is_leaf(5)??

    let balanced_parenthesis2 = make_bp_from_0_1_string("10");
    assert_eq(0, balanced_parenthesis2.is_leaf(0));
    // what if balanced_parenthesis2.is_leaf(2)??
    // what if balanced_parenthesis2.is_leaf(1)??

    let balanced_parenthesis3 = make_bp_from_0_1_string("");
    // what if balanced_parenthesis3.is_leaf(30)??
    // what if balanced_parenthesis3.is_leaf(0)??

    let balanced_parenthesis4 = make_bp_from_0_1_string("110101010100");
    /*
                0
            /  / \ \ \
           1  3  5 7 9
    */
    assert_eq(true, balanced_parenthesis4.is_leaf(3));
    assert_eq(false, balanced_parenthesis4.is_leaf(0));
    // what if balanced_parenthesis4.is_leaf(13)??
    // what if balanced_parenthesis4.is_leaf(42)??
    // what if balanced_parenthesis4.is_leaf(8)??
}





fn test_ansector(){

}


fn test_parent(){

}


fn test_first_child(){

}


fn test_subtree_size(){

}

}

extern crate serde;
#[macro_use]
extern crate serde_derive;

mod bp;
mod binary_heap;
mod range_min_max_node;
mod range_min_max_tree;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

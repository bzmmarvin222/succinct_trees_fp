extern crate bit_vec;
extern crate serde;
extern crate serde_json;

use bit_vec::BitVec;
use bp::wrapper::BpSerializingWrapper;

fn serialize(){

}

fn deserialize() {

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serializing_wrapper() {
        let bitvec = BitVec::from_bytes(&[1111]);
        let wrapper = BpSerializingWrapper {data: bitvec};

        let serialized = serde_json::to_string(&bitvec).unwrap();

        // Prints serialized = {"x":1,"y":2}
        println!("serialized = {}", serialized);
    }
}

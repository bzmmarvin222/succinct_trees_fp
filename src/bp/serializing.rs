extern crate serde;
extern crate serde_json;
extern crate bv;


fn serialize(){

}

fn deserialize() {

}


#[cfg(test)]
mod tests {
    use super::*;
    use super::bv::BitVec;

    #[test]
    fn test_serializer() {

        let vec: BitVec<u32> = BitVec::new_fill(true, 8);


        let serialized = serde_json::to_string(&vec).unwrap();
        let deserialized: BitVec<u32> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, vec);
    }
}

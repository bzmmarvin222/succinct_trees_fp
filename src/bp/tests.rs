use super::*;
use super::bv::BitVec;

#[test]
fn serialize_deserialize() {
    let bp = Bp::create(BitVec::new_fill(true, 8));
    let serialized = bp.serialize();
    let deserialized = Bp::deserialize(serialized);
    assert_eq!(deserialized, bp);
}
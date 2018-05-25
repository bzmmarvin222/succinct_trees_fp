extern crate bit_vec;
extern crate serde;

use bit_vec::BitVec;

#[derive(Serialize, Deserialize, Debug)]
pub struct BpSerializingWrapper {
    data: BitVec
}
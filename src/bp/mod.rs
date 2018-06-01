extern crate serde;
extern crate bincode;
extern crate bv;

use self::bv::BitVec;
use self::bincode::{serialize, deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Bp {
    vec: BitVec
}

impl Bp {
    fn create(vector: BitVec) -> Bp {
        Bp {
            vec: vector
        }
    }

    fn serialize(&self) -> Vec<u8> {
        serialize(self).unwrap()
    }

    fn deserialize(serialized: Vec<u8>) -> Bp {
        deserialize(&serialized[..]).unwrap()
    }
}

#[cfg(test)]
mod tests;
use crate::constant::NODE_SIZE;

const BYTES_PER_NODE: u64 = NODE_SIZE as u64;

pub const fn from_height(height: u32) -> u64 {
    2u64.pow(height) * BYTES_PER_NODE
}

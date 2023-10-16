use crate::constant::NODE_SIZE;
use sha2::{Digest, Sha256};

/**
 * Represents merkle tree node.
 */
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MerkleTreeNode(pub [u8; NODE_SIZE]);

impl MerkleTreeNode {
    pub fn new(payload: &[u8]) -> Self {
        MerkleTreeNode(truncated_hash(payload))
    }
    pub fn join(self: &Self, right: &MerkleTreeNode) -> Self {
        compute_node(self, right)
    }
    pub fn empty() -> Self {
        MerkleTreeNode([0u8; NODE_SIZE])
    }
}

impl From<[u8; NODE_SIZE]> for MerkleTreeNode {
    fn from(bytes: [u8; NODE_SIZE]) -> Self {
        MerkleTreeNode(bytes)
    }
}

// Function to compute the truncated hash of a payload
pub fn truncated_hash(payload: &[u8]) -> [u8; NODE_SIZE] {
    let mut sha256 = Sha256::new();
    sha256.update(payload);
    let mut digest = sha256.finalize().into();
    truncate(&mut digest);
    digest
}

// Function to compute a Merkle tree node from left and right nodes
pub fn compute_node(left: &MerkleTreeNode, right: &MerkleTreeNode) -> MerkleTreeNode {
    let mut payload = vec![0u8; left.0.len() + right.0.len()];
    payload[..left.0.len()].copy_from_slice(&left.0);
    payload[left.0.len()..].copy_from_slice(&right.0);

    MerkleTreeNode(truncated_hash(&payload))
}

pub fn empty_node() -> MerkleTreeNode {
    MerkleTreeNode([0u8; NODE_SIZE])
}

// Function to truncate a Merkle tree node
#[inline]
pub fn truncate(node: &mut [u8; NODE_SIZE]) {
    node[NODE_SIZE - 1] &= 0b00111111;
}

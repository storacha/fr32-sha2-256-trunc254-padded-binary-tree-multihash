use crate::constant::NODE_SIZE;
use crate::tree::MerkleTreeNode;
use lazy_static::lazy_static;
const MAX_LEVEL: usize = 64; // Define the maximum level

/// Lazy zero-comm buffer which fills up on demand.

struct ZeroComm {
    bytes: Vec<u8>,
    node: MerkleTreeNode,
    length: usize,
}

impl ZeroComm {
    fn new() -> Self {
        let mut zero_comm = ZeroComm {
            bytes: vec![0; MAX_LEVEL * NODE_SIZE],
            node: MerkleTreeNode::empty(),
            length: NODE_SIZE,
        };

        zero_comm.bytes[..NODE_SIZE].copy_from_slice(&zero_comm.node.0);

        zero_comm
    }

    fn slice(&mut self, start: usize, end: usize) -> &[u8] {
        while self.length < end {
            self.node = self.node.join(&self.node);
            self.bytes[self.length..self.length + NODE_SIZE].copy_from_slice(&self.node.0);
            self.length += NODE_SIZE;
        }

        &self.bytes[start..end]
    }
}

lazy_static! {
    static ref ZERO_COMM: std::sync::Mutex<ZeroComm> = std::sync::Mutex::new(ZeroComm::new());
}

/// Access nodes by level.
pub fn from_level(level: usize) -> Result<MerkleTreeNode, String> {
    if level >= MAX_LEVEL {
        let result: Result<MerkleTreeNode, String> = Err(format!(
            "Only levels between 0 and {} inclusive are available",
            MAX_LEVEL - 1,
        ));
        return result;
    }

    let start = NODE_SIZE * level;
    let end = NODE_SIZE * (level + 1);

    let mut zero_comm = ZERO_COMM.lock().unwrap();
    let mut node = [0u8; NODE_SIZE];
    node.copy_from_slice(zero_comm.slice(start, end));

    Ok(MerkleTreeNode(node))
}

#[cfg(test)]
mod tests {
    use crate::zero_comm::from_level;

    #[test]
    fn test_from_level_0() {
        let node = from_level(0);
        assert_eq!(node.unwrap().0, [0u8; 32]);
    }

    #[test]
    fn test_too_large() {
        let node = from_level(64);
        assert_eq!(node.is_err(), true);
    }
}

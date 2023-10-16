use crate::constant::{IN_BYTES_PER_QUAD, OUT_BYTES_PER_QUAD};
use crate::tree::MerkleTreeNode;
use crate::util::from_height;
use crate::zero_comm::from_level;

// Filecoin piece representation
pub struct Piece {
    root: MerkleTreeNode,
    height: usize,
    padding_size: u64,
}

impl Piece {
    /// Creates a piece for the given tree height filled with zeros.
    /// If you need to create a piece for a tree containing data other than
    /// zeros, use `with_root` instead.
    pub fn new(height: usize) -> Self {
        Piece {
            height,
            root: from_level(height).unwrap(),
            padding_size: 0,
        }
    }

    /// Sets the root of the given piece. This is generally used when creating
    /// a known piece e.g. `Piece::new(29).with_root(root)`.
    pub fn with_root(&mut self, root: MerkleTreeNode) -> &mut Self {
        self.root = root;
        self
    }

    /// Can be used to set the padding used when creating a piece.
    pub fn with_padding_size(&mut self, padding_size: u64) -> &mut Self {
        self.padding_size = padding_size;
        self
    }

    // Accessors
    // Note that struct fields aren't public because we don't want to commit
    // to a particular representation. Which is why we use set of accessors
    // which can be kept stable while the internal representation changes.

    pub fn root(&self) -> MerkleTreeNode {
        self.root
    }
    pub fn height(&self) -> usize {
        self.height
    }

    /// Piece size in bytes.
    pub fn size(&self) -> u64 {
        from_height(self.height as u32)
    }

    /// Number of 0-bytes payload was padded by the hasher.
    pub fn padding_size(&self) -> u64 {
        self.padding_size
    }

    /// Payload size in bytes.
    pub fn payload_size(&self) -> u64 {
        let padded_size = self.size() / OUT_BYTES_PER_QUAD as u64 * IN_BYTES_PER_QUAD as u64;
        padded_size - self.padding_size()
    }
}

use crate::PieceHasher;
use multihash_derive::MultihashDigest;

#[derive(Clone, Copy, Debug, Eq, MultihashDigest, PartialEq)]
#[mh(alloc_size = 64)]
pub enum Code {
    /// Example for using a custom hasher which returns truncated hashes
    #[mh(code = 0x1011, hasher = PieceHasher)]
    PieceHasher,
}

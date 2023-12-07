use crate::constant::{IN_BITS_FR, IN_BYTES_PER_QUAD, NODE_SIZE, OUT_BITS_FR, OUT_BYTES_PER_QUAD};
use crate::piece::Piece;
use crate::tree::{compute_node, truncated_hash, MerkleTreeNode};
use crate::util::{from_height, required_zero_padding};
use crate::{varint_estimate, zero_comm};
use cid;
use core::primitive::u64;
use js_sys::{Error, Uint8Array};
use multihash::Multihash;
use multihash_derive::Hasher;
use std::convert::{TryFrom, TryInto};
use unsigned_varint;
use wasm_bindgen::prelude::*;

// Fits for 32PiB of data
const MAX_HEIGHT: u8 = 50; //u8::MAX;

// Multihash code
pub const CODE: u64 = 0x1011;

const MAX_PADDING_SIZE: usize = 10; // Max amount of bytes allowed for varint
pub const HEIGHT_SIZE: usize = 1; // Height is stored in a single byte
pub const ROOT_SIZE: usize = NODE_SIZE; // Size of the merkle tree root
pub const CODE_SIZE: usize = varint_estimate(CODE); // Size of the multihash code

const RAW: usize = 0x55;

pub const MAX_MULTIHASH_SIZE: usize = HEIGHT_SIZE + MAX_PADDING_SIZE + NODE_SIZE;
type Layer = Vec<MerkleTreeNode>;
type Layers = Vec<Layer>;
type QuadBuffer = [u8; IN_BYTES_PER_QUAD];

/**
 * Max payload is determined by the maximum height of the tree, which is limited
 * by the int we could store in one byte. We calculate the max piece size
 * and derive max payload size that can would produce it after FR32 padding.
 */
pub const MAX_PAYLOAD_SIZE: u64 =
    from_height(MAX_HEIGHT as u32) * IN_BITS_FR as u64 / OUT_BITS_FR as u64;

#[wasm_bindgen]
pub struct PieceHasher {
    pub(crate) bytes_written: u64,
    buffer: QuadBuffer,
    offset: usize,
    layers: Layers,

    digest: [u8; MAX_MULTIHASH_SIZE],
}

impl PieceHasher {
    pub fn new() -> Self {
        PieceHasher {
            bytes_written: 0,
            buffer: [0; IN_BYTES_PER_QUAD],
            offset: 0,
            layers: vec![Vec::new()],
            digest: [0; MAX_MULTIHASH_SIZE],
        }
    }

    pub fn try_update(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let leaves = &mut self.layers[0];
        let length = bytes.len();
        // If we got no bytes there is nothing to do here
        if length == 0 {
            return Result::Ok(());
        } else if self.bytes_written + length as u64 > MAX_PAYLOAD_SIZE as u64 {
            return Result::Err(Error::new("Payload size exceeded"));
        }
        // If we do not have enough bytes to form a quad, just add append new bytes
        // to the buffer and return.
        else if self.offset + length < self.buffer.len() {
            self.buffer[self.offset..self.offset + length].copy_from_slice(bytes);
            self.offset += length;
            self.bytes_written += length as u64;
            return Result::Ok(());
        } else {
            let bytes_required = self.buffer.len() - self.offset;
            self.buffer[self.offset..].copy_from_slice(&bytes[..bytes_required]);
            read_quad(&self.buffer, leaves);
            // leaves.append(&mut split(pad(&self.buffer)));
            let mut read_offset = bytes_required;

            while read_offset + IN_BYTES_PER_QUAD < length {
                let quad = &bytes[read_offset..read_offset + IN_BYTES_PER_QUAD];
                read_quad(quad.try_into().unwrap(), leaves);
                read_offset += IN_BYTES_PER_QUAD;
            }

            self.buffer[..length - read_offset].copy_from_slice(&bytes[read_offset..]);
            self.offset = length - read_offset;
            self.bytes_written += length as u64;

            prune(&mut self.layers);
            return Result::Ok(());
        }
    }

    /// Returns number of bytes required to store the raw digest.
    // pub fn digestSize() {
    //     let paddingLength = required_zero_padding(self.bytes_written);
    //     unsigned_varint::codec::u64::
    // }

    /// Returns number of bytes required to store the multihash digest.
    pub fn multihashSize() {}

    pub fn multihash(&mut self) -> Multihash {
        let bytes = self.finalize();
        Multihash::wrap(CODE, bytes).unwrap()
    }

    pub fn link(&mut self) -> cid::Cid {
        cid::Cid::new_v1(RAW as u64, self.multihash())
    }
}

// Implement default constructor for the PieceHasher
impl Default for PieceHasher {
    fn default() -> Self {
        PieceHasher::new()
    }
}

// Implement conversion from Piece reference to PieceHasher that
// contains that piece. Next updates will end up in the sibling of
// the piece.
impl From<&Piece> for PieceHasher {
    fn from(piece: &Piece) -> Self {
        let mut hasher = PieceHasher::default();
        // All but the last layer will be empty as they will be
        // collapsed into the the root node.
        let top = piece.height() - 1;
        for _ in 0..top {
            hasher.layers.push(vec![]);
        }
        // Finally top layer will have only our piece root
        hasher.layers[top] = vec![piece.root()];

        // Bytes written will correspond to the sum of original payload size
        // and applied 0-padding. Note that we have to account for padding
        // given that we don't know internal piece subtrees without which we
        // are not able to continue hashing from with-in the piece boundaries.
        hasher.bytes_written = piece.payload_size() + piece.padding_size();

        hasher
    }
}

// Implement conversion from the byte array to the PieceHasher
impl<const N: usize> From<&[u8; N]> for PieceHasher {
    fn from(bytes: &[u8; N]) -> Self {
        let mut hasher = PieceHasher::new();
        hasher.update(bytes);
        hasher
    }
}

// Implements conversion from the byte array slice to the PieceHasher
// Note that above does not cover this nor this covers the above.
impl From<&[u8]> for PieceHasher {
    fn from(bytes: &[u8]) -> Self {
        let mut hasher = PieceHasher::new();
        hasher.update(bytes);
        hasher
    }
}

// Implement Hasher trait defined by the multihash crate for the PieceHasher
// so that it could be use by multihash codec table.
impl Hasher for PieceHasher {
    fn update(&mut self, bytes: &[u8]) {
        self.try_update(bytes).unwrap();
    }
    fn finalize(&mut self) -> &[u8] {
        let mut layers = self.layers.clone();
        let leaves = &mut layers[0];

        if self.offset > 0 || self.bytes_written == 0 {
            self.buffer[self.offset..].fill(0);
            read_quad(&self.buffer, leaves);
        }

        build(&mut layers);

        let height = layers.len();
        let root = layers[height - 1][0];

        // encode padding
        let mut padding_bytes = unsigned_varint::encode::u64_buffer();
        let padding = unsigned_varint::encode::u64(
            required_zero_padding(self.bytes_written),
            &mut padding_bytes,
        );
        self.digest[0..padding.len()].copy_from_slice(&padding);

        // set the tree height
        self.digest[padding.len()] = height as u8;

        // copy the root hash
        self.digest[padding.len() + 1..][..NODE_SIZE].copy_from_slice(&root.0);

        return &self.digest[..padding.len() + 1 + NODE_SIZE];
    }
    fn reset(&mut self) {
        self.offset = 0;
        self.bytes_written = 0;
        self.layers.clear();
        self.layers.push(Vec::new());
    }
}

fn read_quad(source: &QuadBuffer, output: &mut Layer) {
    let mut buffer = [0u8; OUT_BYTES_PER_QUAD];
    let mut offset = 0;

    // First 31 bytes + 6 bits are taken as-is (trimmed later)
    buffer[0..32].copy_from_slice(&source[0..32]);
    // first 2-bit "shim" forced into the otherwise identical output
    buffer[31] &= 0b00111111;

    offset += NODE_SIZE;

    // copy next Fr32 preceded with the last two bits of the previous Fr32
    for i in offset..offset + NODE_SIZE {
        buffer[i] = (source[i] << 2) | (source[i - 1] >> 6);
    }
    buffer[offset + 31] &= 0b00111111;
    offset += NODE_SIZE;

    for i in offset..offset + NODE_SIZE {
        buffer[i] = (source[i] << 4) | (source[i - 1] >> 4);
    }
    buffer[offset + 31] &= 0b00111111;
    offset += NODE_SIZE;

    for i in offset..OUT_BYTES_PER_QUAD - 1 {
        buffer[i] = (source[i] << 6) | (source[i - 1] >> 2);
    }
    // we shim last 2-bits by shifting the last byte by two bits
    buffer[offset + 31] = source[IN_BYTES_PER_QUAD - 1] >> 2;

    output.push(MerkleTreeNode::from(truncated_hash(
        &buffer[0..NODE_SIZE * 2],
    )));
    output.push(MerkleTreeNode::from(truncated_hash(
        &buffer[NODE_SIZE * 2..],
    )));
}

/**
 * Prunes layers by combining node pairs into nodes in the next layer and
 * removing them from the layer that they were in. After pruning each layer
 * will end up with at most one node. New layers may be created in the process
 * when nodes from the top layer are combined.
 */
fn prune(layers: &mut Layers) {
    flush(layers, false);
}

/**
 * Flushes all the nodes in layers by combining node pairs into nodes in the
 * next layer. Layers with only one node are combined with zero padded nodes
 * (corresponding to the level of the layer). Unlike {@link prune} combined
 * nodes are not removed and layers are copied instead of been mutated.
 */
fn build(layers: &mut Layers) {
    flush(layers, true);
}

// Function to flush layers by combining nodes and optionally adding zero padding nodes
fn flush(layers: &mut Layers, build: bool) {
    // Note it is important that we do not mutate any of the layers otherwise
    // writing more data into the hasher and computing the digest will produce
    // wrong results.
    let mut level = 0;
    // We will walk up the tree until we reach the top layer. However, we may end
    // up with creating new layers in the process, so we will keep track of the
    while level < layers.len() {
        let mut index = 0; // Initialize the index to 0

        {
            let height = layers.len();
            let layer = &mut layers[level];

            // If we have the odd number of nodes and we have not reached the top
            // layer, we push a zero padding node corresponding to the current level.
            if build && layer.len() % 2 > 0 && level + 1 < height {
                let zero_pad = zero_comm::from_level(level + 1).unwrap(); // Create a zero padding node
                layer.push(zero_pad); // Push the zero padding node to the current layer
            }
        }

        while index + 1 < layers[level].len() {
            let node = compute_node(&layers[level][index], &layers[level][index + 1]); // Compute a new node

            if level + 1 < layers.len() {
                layers[level + 1].push(node); // Otherwise, push the new node to the next layer
            } else {
                layers.push(vec![node]); // Push the new node to the current layer if it's the last layer
            }

            index += 2; // Increment the index
        }

        layers[level].splice(0..index, []);
        level += 1; // Increment the level
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    pub struct Varint([u8; 10]);

    impl From<u64> for Varint {
        fn from(value: u64) -> Self {
            let mut buffer = unsigned_varint::encode::u64_buffer();
            unsigned_varint::encode::u64(value, &mut buffer);
            Varint(buffer)
        }
    }

    impl TryInto<u64> for Varint {
        type Error = unsigned_varint::decode::Error;
        fn try_into(self) -> Result<u64, Self::Error> {
            let out = unsigned_varint::decode::u64(&self.0);
            match out {
                Ok((value, _)) => Ok(value),
                Err(e) => Err(e),
            }
        }
    }

    #[test]
    fn test_large_num() {
        let varint = Varint::from(2u64.pow(63));
        assert_eq!(
            varint.0[..],
            [128, 128, 128, 128, 128, 128, 128, 128, 128, 1]
        );

        assert_eq!(varint.try_into(), Ok(2u64.pow(63)));
    }
}

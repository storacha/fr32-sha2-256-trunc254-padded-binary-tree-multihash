use crate::constant::{IN_BITS_FR, IN_BYTES_PER_QUAD, NODE_SIZE, OUT_BITS_FR, OUT_BYTES_PER_QUAD};
use crate::tree::{compute_node, truncated_hash, MerkleTreeNode};
use crate::util::from_height;
use crate::zero_comm;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;

// Fits for 32PiB of data
const MAX_HEIGHT: u8 = 50; //u8::MAX;

pub const MULTIHASH_SIZE: usize = 33;
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

/**
 * The smallest amount of data for which FR32 padding has a defined result.
 * Silently upgrading 2 leaves to 4 would break the symmetry so we require
 * an extra byte and the rest can be 0 padded to expand to 4 leaves.
 */
pub const MIN_PAYLOAD_SIZE: u64 = (2 * NODE_SIZE + 1) as u64;

// @see https://github.com/multiformats/rust-multihash/blob/452a933396adcd5915c53563d5017df76ae3ec26/derive/src/hasher.rs
pub trait Hasher {
    /// Consume input and update internal state.
    fn update(&mut self, input: &[u8]);

    /// Returns the final digest.
    fn finalize(&mut self) -> &[u8];

    /// Reset the internal hasher state.
    fn reset(&mut self);
}

#[wasm_bindgen]
pub struct PieceHasher {
    pub(crate) bytes_written: u64,
    buffer: QuadBuffer,
    offset: usize,
    layers: Layers,

    digest: [u8; MULTIHASH_SIZE],
}

impl PieceHasher {
    pub fn new() -> Self {
        PieceHasher::default()
    }
}

impl Default for PieceHasher {
    fn default() -> Self {
        PieceHasher {
            bytes_written: 0,
            buffer: [0; IN_BYTES_PER_QUAD],
            offset: 0,
            layers: vec![Vec::new()],
            digest: [0; MULTIHASH_SIZE],
        }
    }
}

impl Hasher for PieceHasher {
    fn update(&mut self, bytes: &[u8]) {
        let leaves = &mut self.layers[0];
        let length = bytes.len();
        // If we got no bytes there is nothing to do here
        if length == 0 {
            return;
        } else if self.bytes_written + length as u64 > MAX_PAYLOAD_SIZE as u64 {
            panic!("Payload size exceeded")
        }
        // If we do not have enough bytes to form a quad, just add append new bytes
        // to the buffer and return.
        else if self.offset + length < self.buffer.len() {
            self.buffer[self.offset..self.offset + length].copy_from_slice(bytes);
            self.offset += length;
            self.bytes_written += length as u64;
            return;
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
            return;
        }
    }
    fn finalize(&mut self) -> &[u8] {
        let mut layers = self.layers.clone();
        let leaves = &mut layers[0];

        if self.bytes_written < MIN_PAYLOAD_SIZE {
            panic!(
                "Algorithm is not defined for payloads smaller than {} bytes",
                MIN_PAYLOAD_SIZE
            )
        }

        if self.offset > 0 {
            self.buffer[self.offset..].fill(0);
            read_quad(&self.buffer, leaves);
        }

        build(&mut layers);

        let height = layers.len();
        let root = layers[height - 1][0];

        self.digest[0] = height as u8;
        self.digest[1..].copy_from_slice(&root.0);

        return &self.digest;
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
    use hex::ToHex;

    use crate::hasher::{Hasher, PieceHasher};
    #[test]
    fn test_basic() {
        let mut hasher = PieceHasher::new();
        let data = vec![0u8; 65];
        hasher.update(&data);
        let digest = hasher.finalize();

        assert_eq!(
            digest.encode_hex::<String>(),
            "023731bb99ac689f66eef5973e4a94da188f4ddcae580724fc6f3fd60dfd488333"
        );
        assert_eq!(
            digest,
            [
                2, 55, 49, 187, 153, 172, 104, 159, 102, 238, 245, 151, 62, 74, 148, 218, 24, 143,
                77, 220, 174, 88, 7, 36, 252, 111, 63, 214, 13, 253, 72, 131, 51
            ]
        )
    }
}

use wasm_bindgen::prelude::*;
pub mod constant;
pub mod fr32;
mod hasher;
pub mod tree;
mod util;
mod zero_comm;

use hasher::{Hasher, PieceHasher, MULTIHASH_SIZE};

// When the `wee_olloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const PREFIX: [u8; 3] = [145, 32, 33]; // Replace with actual values

type PieceMultihasher = PieceHasher;

#[wasm_bindgen]
impl PieceHasher {
    /// Creates a new hasher
    #[wasm_bindgen(constructor)]
    pub fn create() -> PieceMultihasher {
        PieceHasher::default()
    }

    #[wasm_bindgen]
    pub fn count(&self) -> u64 {
        self.bytes_written
    }
    /// Resets the hasher state
    #[wasm_bindgen]
    pub fn reset(&self) {
        self.reset();
    }

    #[wasm_bindgen]
    pub fn write(&mut self, bytes: &[u8]) {
        &self.update(bytes);
    }

    #[wasm_bindgen(js_name = digestInto)]
    pub fn read(&mut self, target: &mut [u8], offset: Option<usize>, use_prefix: Option<bool>) {
        let mut byte_offset = offset.unwrap_or(0);
        if use_prefix.unwrap_or(true) {
            target[byte_offset..byte_offset + PREFIX.len()].copy_from_slice(&PREFIX);
            byte_offset += PREFIX.len();
        }
        target[byte_offset..byte_offset + MULTIHASH_SIZE].copy_from_slice(&self.finalize());
    }
}

#[wasm_bindgen]
pub fn create() -> PieceMultihasher {
    PieceHasher::default()
}

#[wasm_bindgen]
pub fn reset(hasher: &mut PieceHasher) {
    hasher.reset();
}

#[wasm_bindgen(js_name = readHashInto)]
pub fn read_hash_into(hasher: &mut PieceHasher, target: &mut [u8], offset: usize) {
    target[offset..].copy_from_slice(&hasher.finalize());
}

#[cfg(test)]
mod tests {
    use crate::create;

    #[test]
    fn test_lib() {
        let mut hasher = create();
        let data = vec![0u8; 65];
        hasher.write(&data);

        let mut out = [0u8; 36];
        hasher.read(&mut out, None, None);

        assert_eq!(
            out,
            [
                145, 32, 33, 2, 55, 49, 187, 153, 172, 104, 159, 102, 238, 245, 151, 62, 74, 148,
                218, 24, 143, 77, 220, 174, 88, 7, 36, 252, 111, 63, 214, 13, 253, 72, 131, 51
            ]
        );
    }
}

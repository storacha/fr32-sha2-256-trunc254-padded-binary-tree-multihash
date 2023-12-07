use js_sys::Error;
use wasm_bindgen::prelude::*;
pub mod constant;
mod hasher;
mod piece;
pub mod tree;
mod util;
mod zero_comm;
use hasher::{PieceHasher, CODE_SIZE, HEIGHT_SIZE, ROOT_SIZE};
use multihash_derive::Hasher;
use util::{required_zero_padding, varint_estimate};
pub mod multihash;

type PieceMultihasher = PieceHasher;

#[wasm_bindgen(inspectable)]
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
    pub fn reset(&mut self) {
        Hasher::reset(self);
    }

    #[wasm_bindgen]
    pub fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        PieceHasher::try_update(self, bytes)
    }

    #[wasm_bindgen(js_name = digestInto)]
    pub fn read(
        &mut self,
        target: &mut [u8],
        offset: Option<usize>,
        use_prefix: Option<bool>,
    ) -> usize {
        let hash = self.multihash();
        let byte_offset = offset.unwrap_or(0);
        if use_prefix.unwrap_or(true) {
            let bytes = hash.to_bytes();
            target[byte_offset..byte_offset + bytes.len()].copy_from_slice(&bytes);
            bytes.len()
        } else {
            let bytes = hash.digest();
            target[byte_offset..byte_offset + bytes.len()].copy_from_slice(&bytes);
            bytes.len()
        }
    }

    #[wasm_bindgen(js_name = digestByteLength)]
    pub fn digest_size(&self) -> usize {
        let padding = required_zero_padding(self.bytes_written);

        varint_estimate(padding) + HEIGHT_SIZE + ROOT_SIZE
    }

    #[wasm_bindgen(js_name = multihashByteLength)]
    pub fn multihash_size(&self) -> usize {
        let hash_size = self.digest_size();

        CODE_SIZE + varint_estimate(hash_size as u64) + hash_size
    }
}

#[wasm_bindgen]
pub fn create() -> PieceMultihasher {
    PieceHasher::default()
}

#[cfg(test)]
mod tests {
    use std::hash;

    use crate::create;

    #[test]
    fn test_lib() {
        let mut hasher = create();
        let data = [0u8; 65];
        hasher.write(&data);

        let mut out = [0u8; 32 + 1 + 9];
        let size = hasher.read(&mut out, None, None);
        assert_eq!(hasher.multihash_size(), size);

        assert_eq!(
            out[..size],
            [
                145, 32, 34, 62, 2, 55, 49, 187, 153, 172, 104, 159, 102, 238, 245, 151, 62, 74,
                148, 218, 24, 143, 77, 220, 174, 88, 7, 36, 252, 111, 63, 214, 13, 253, 72, 131,
                51
            ]
        );
    }

    use crate::piece::Piece;
    use crate::PieceHasher;
    use multihash_derive::MultihashDigest;
    #[test]
    fn test_multihash() {
        let hash = crate::multihash::Code::PieceHasher.digest(b"hello world");

        assert_eq!(hash.code(), 0x1011);
        assert_eq!(hash.size(), 34);
    }

    #[test]
    fn test_0_bytes() {
        let mut hasher = PieceHasher::from(&[]);
        let hash = hasher.multihash();

        assert_eq!(hash.code(), 0x1011);
        assert_eq!(hash.size(), 34);
        assert_eq!(
            hash.digest(),
            [
                127, // padding
                2,   // height
                55, 49, 187, 153, 172, 104, 159, 102, 238, 245, 151, 62, 74, 148, 218, 24, 143, 77,
                220, 174, 88, 7, 36, 252, 111, 63, 214, 13, 253, 72, 131, 51
            ]
        );

        assert_eq!(
            hasher.link().to_string(),
            "bafkzcibcp4bdomn3tgwgrh3g532zopskstnbrd2n3sxfqbze7rxt7vqn7veigmy"
        )
    }

    #[test]
    fn test_127_bytes() {
        let mut hasher = PieceHasher::from(&[0; 127]);
        assert_eq!(hasher.digest_size(), 34);
        let hash = hasher.multihash();

        assert_eq!(hash.code(), 0x1011);
        assert_eq!(hash.size(), 34);
        assert_eq!(
            hash.digest(),
            [
                0, // padding
                2, // height
                55, 49, 187, 153, 172, 104, 159, 102, 238, 245, 151, 62, 74, 148, 218, 24, 143, 77,
                220, 174, 88, 7, 36, 252, 111, 63, 214, 13, 253, 72, 131, 51
            ]
        );

        assert_eq!(
            hasher.link().to_string(),
            "bafkzcibcaabdomn3tgwgrh3g532zopskstnbrd2n3sxfqbze7rxt7vqn7veigmy"
        )
    }

    #[test]
    fn test_128_bytes() {
        let payload = [0u8; 128];
        let mut hasher = PieceHasher::from(&payload);
        let hash = hasher.multihash();

        assert_eq!(hasher.digest_size(), 34);
        assert_eq!(hash.code(), 0x1011);
        assert_eq!(hash.size(), 34);
        assert_eq!(
            hash.digest(),
            [
                126, // padding
                3,   // height
                // root
                100, 42, 96, 126, 248, 134, 176, 4, 191, 44, 25, 120, 70, 58, 225, 212, 105, 58,
                192, 244, 16, 235, 45, 27, 122, 71, 254, 32, 94, 94, 117, 15
            ]
        );

        assert_eq!(
            PieceHasher::from(&payload).link().to_string(),
            "bafkzcibcpybwiktap34inmaex4wbs6cghlq5i2j2yd2bb2zndn5ep7ralzphkdy"
        )
    }
    #[test]
    fn test_32() {
        let mut hasher = PieceHasher::from(&Piece::new(30));

        assert_eq!(hasher.digest_size(), 34);

        let multihash = hasher.multihash();

        assert_eq!(multihash.size(), 34);
        assert_eq!(
            multihash.digest(),
            [
                0,  // padding
                30, // height
                // root
                7, 126, 95, 222, 53, 197, 10, 147, 3, 165, 80, 9, 227, 73, 138, 78, 190, 223, 243,
                156, 66, 183, 16, 183, 48, 216, 236, 122, 199, 175, 166, 62
            ]
        );

        assert_eq!(
            hasher.link().to_string(),
            "bafkzcibcaapao7s73y24kcutaosvacpdjgfe5pw76ooefnyqw4ynr3d2y6x2mpq"
        )
    }

    #[test]
    fn test_64() {
        let mut hasher = PieceHasher::from(&Piece::new(31));

        assert_eq!(
            hasher.link().to_string(),
            "bafkzcibcaap6mqafu276g53zko4k23xzh4h4uecjwicbmvhsuqi7o4bhthhm4aq"
        )
    }

    #[test]
    fn test_first_case() {
        // Take data of size 127*4 bytes where
        let mut payload = [0u8; 127 * 4];
        // the first 127 bytes are 0
        payload[0..127].fill(0);
        // the next 127 are 1
        payload[127..127 * 2].fill(1);
        // the next 127 are 2
        payload[127 * 2..127 * 3].fill(2);
        // and the last 127 are 3
        payload[127 * 3..].fill(3);

        let hash = PieceHasher::from(&payload).multihash();

        assert_eq!(hash.code(), 0x1011);
        assert_eq!(hash.size(), 34);

        assert_eq!(
            hash.digest(),
            [
                0, // padding
                4, // height
                73, 109, 174, 12, 201, 226, 101, 239, 229, 160, 6, 232, 6, 38, 165, 220, 92, 64,
                158, 93, 49, 85, 193, 57, 132, 202, 246, 200, 213, 207, 214, 5
            ]
        );

        assert_eq!(
            hash.to_bytes(),
            [
                145, 32, // multihash code
                34, // multihash size
                0,  // padding
                4,  // height
                73, 109, 174, 12, 201, 226, 101, 239, 229, 160, 6, 232, 6, 38, 165, 220, 92, 64,
                158, 93, 49, 85, 193, 57, 132, 202, 246, 200, 213, 207, 214, 5
            ]
        );

        assert_eq!(
            PieceHasher::from(&payload).link().to_string(),
            "bafkzcibcaaces3nobte6ezpp4wqan2age2s5yxcatzotcvobhgcmv5wi2xh5mbi"
        );
    }

    #[test]
    fn test_second_case() {
        // Take data of size 128*4 bytes where
        let mut payload = [0u8; 128 * 4];
        // the first where the first 127 bytes are 0
        payload[0..127].fill(0);
        // the next 127 are 1
        payload[127..127 * 2].fill(1);
        // the next 127 are 2
        payload[127 * 2..127 * 3].fill(2);
        // the next 127 are 3
        payload[127 * 3..127 * 4].fill(3);
        // and the remaining 4 bytes are 0
        payload[127 * 4..].fill(0);

        let mut hasher = PieceHasher::from(&payload);
        assert_eq!(hasher.digest_size(), 35);
        let hash = hasher.multihash();

        assert_eq!(hash.size(), 35);
        assert_eq!(
            hash.digest(),
            [
                248, 3, // padding
                5, // height
                222, 104, 21, 220, 179, 72, 132, 50, 21, 169, 77, 229, 50, 149, 75, 96, 190, 85,
                10, 75, 236, 110, 116, 85, 86, 101, 233, 165, 236, 78, 15, 60
            ]
        );

        assert_eq!(
            PieceHasher::from(&payload).link().to_string(),
            "bafkzcibd7abqlxticxolgseegik2stpfgkkuwyf6kufex3doorkvmzpjuxwe4dz4"
        )
    }

    #[test]
    fn test_third_case() {
        // Take data of size 128*4 bytes where
        let mut data = [0u8; 128 * 4];
        // the first where the first 127 bytes are 0
        data[0..127].fill(0);
        // the next 127 are 1
        data[127..127 * 2].fill(1);
        // the next 127 are 2
        data[127 * 2..127 * 3].fill(2);
        // the next 127 are 3
        data[127 * 3..127 * 4].fill(3);
        // and the remaining 4 bytes are 0
        data[127 * 4..].fill(0);

        // append one more zero
        let mut payload = data.to_vec();
        payload.push(0);

        let mut hasher = PieceHasher::from(&payload[..]);
        assert_eq!(hasher.digest_size(), 35);
        let hash = hasher.multihash();

        assert_eq!(hash.size(), 35);
        assert_eq!(
            hash.digest(),
            [
                247, 3, // padding
                5, // height
                222, 104, 21, 220, 179, 72, 132, 50, 21, 169, 77, 229, 50, 149, 75, 96, 190, 85,
                10, 75, 236, 110, 116, 85, 86, 101, 233, 165, 236, 78, 15, 60
            ]
        );

        assert_eq!(
            PieceHasher::from(&payload[..]).link().to_string(),
            "bafkzcibd64bqlxticxolgseegik2stpfgkkuwyf6kufex3doorkvmzpjuxwe4dz4"
        )
    }

    #[test]
    fn test_js_case() {
        let mut hasher = PieceHasher::from(&[0u8; 65]);
        let hash = hasher.multihash();

        assert_eq!(hash.size(), 34);
        assert_eq!(
            hash.digest(),
            [
                62, // padding
                2,  // height
                55, 49, 187, 153, 172, 104, 159, 102, 238, 245, 151, 62, 74, 148, 218, 24, 143, 77,
                220, 174, 88, 7, 36, 252, 111, 63, 214, 13, 253, 72, 131, 51,
            ]
        );
    }
}

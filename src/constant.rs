pub const BITS_PER_BYTE: usize = 8;

pub const FRS_PER_QUAD: usize = 4;

pub const IN_BITS_FR: usize = 254;
pub const OUT_BITS_FR: usize = 256;

pub const IN_BYTES_PER_QUAD: usize = (FRS_PER_QUAD * IN_BITS_FR) / BITS_PER_BYTE;
pub const OUT_BYTES_PER_QUAD: usize = (FRS_PER_QUAD * OUT_BITS_FR) / BITS_PER_BYTE;

pub const BYTES_PER_FR: usize = OUT_BYTES_PER_QUAD / FRS_PER_QUAD;

// pub const FR_RATIO: usize = IN_BITS_FR / OUT_BITS_FR;

pub const NODE_SIZE: usize = OUT_BYTES_PER_QUAD / FRS_PER_QUAD;

pub const MIN_PAYLOAD_SIZE: u64 = (2 * NODE_SIZE + 1) as u64;

/**
 * Since first byte in the digest is the tree height, the maximum height is 255.
 */
pub const MAX_HEIGHT: u8 = u8::MAX;

use crate::constant::{FRS_PER_QUAD, IN_BYTES_PER_QUAD, NODE_SIZE};

const BYTES_PER_NODE: u64 = NODE_SIZE as u64;

pub const fn from_height(height: u32) -> u64 {
    2u64.pow(height) * BYTES_PER_NODE
}

/// Calculates zero padding required before the given payload can be fr32 padded.
pub fn required_zero_padding(payload_size: u64) -> u64 {
    let width = required_width(payload_size);
    let padded_size = width / FRS_PER_QUAD as u64 * IN_BYTES_PER_QUAD as u64;
    padded_size - payload_size
}

/// Counts number of leaves required to fit the given payload.
pub fn required_width(payload_size: u64) -> u64 {
    // Number of quads that would fit in the given payload size
    let quads = payload_size as f64 / IN_BYTES_PER_QUAD as f64;

    // Round up to the nearest power of 2 and multiply to number of leaves
    // per quad
    (2 as u64).pow(quads.log2().ceil() as u32) * FRS_PER_QUAD as u64
}

/// Counts number of bytes needed to encode the given value as a varint.
pub const fn varint_estimate(value: u64) -> usize {
    let n = (u64::BITS - value.leading_zeros()) as usize;
    if n < 7 {
        1
    } else {
        ((n - 1) / 7) + 1
    }
}

#[cfg(test)]
mod tests {

    use crate::util::{required_width, required_zero_padding, varint_estimate};

    #[test]
    fn test_varint_estimate() {
        assert_eq!(varint_estimate(0_u64), 1);
        assert_eq!(varint_estimate(1_u64), 1);
        assert_eq!(varint_estimate(2_u64.pow(7) - 1), 1);
        assert_eq!(varint_estimate(2_u64.pow(7)), 2);
        assert_eq!(varint_estimate(2_u64.pow(8)), 2);
        assert_eq!(varint_estimate(2_u64.pow(13)), 2);
        assert_eq!(varint_estimate(2_u64.pow(14)), 3);
        assert_eq!(varint_estimate(2_u64.pow(21) - 1), 3);
        assert_eq!(varint_estimate(2_u64.pow(21)), 4);
        assert_eq!(varint_estimate(2_u64.pow(28)) - 1, 4);
        assert_eq!(varint_estimate(2_u64.pow(28)), 5);
        assert_eq!(varint_estimate(2_u64.pow(35)) - 1, 5);
        assert_eq!(varint_estimate(2_u64.pow(35)), 6);
        assert_eq!(varint_estimate(2_u64.pow(42)) - 1, 6);
        assert_eq!(varint_estimate(2_u64.pow(42)), 7);
        assert_eq!(varint_estimate(2_u64.pow(49)) - 1, 7);
        assert_eq!(varint_estimate(2_u64.pow(49)), 8);

        assert_eq!(varint_estimate(2_u64.pow(56)) - 1, 8);
        assert_eq!(varint_estimate(2_u64.pow(56)), 9);

        assert_eq!(varint_estimate(2_u64.pow(63)) - 1, 9);
        assert_eq!(varint_estimate(2_u64.pow(63)), 10);
    }

    #[test]
    fn test_padding() {
        assert_eq!(required_width(128), 8);
        assert_eq!(required_width(127 * 4 + 1), 8 * 4);
        assert_eq!(required_zero_padding(0), 127);
        assert_eq!(required_zero_padding(1), 126);
        assert_eq!(required_zero_padding(5), 122);
        assert_eq!(required_zero_padding(11), 116);
        assert_eq!(required_zero_padding(127), 0);
        assert_eq!(required_zero_padding(128), 127 - 1);
        assert_eq!(required_zero_padding(127 * 2 - 1), 1);
        assert_eq!(required_zero_padding(127 * 2), 0);
        assert_eq!(required_zero_padding(127 * 2 + 1), 127 * 2 - 1);
        assert_eq!(required_zero_padding(127 * 3), 127);
        assert_eq!(required_zero_padding(127 * 4), 0);
        assert_eq!(required_zero_padding(127 * 4 + 10), 127 * 8 - 127 * 4 - 10);
        assert_eq!(required_zero_padding(128 * 4), 504);
    }
}

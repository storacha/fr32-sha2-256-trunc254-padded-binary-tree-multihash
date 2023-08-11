use crate::constant::{IN_BITS_FR, MIN_PAYLOAD_SIZE, OUT_BITS_FR};
use std::cmp::max;
use std::convert::TryInto;

const IN_BYTES_PER_QUAD: u64 = crate::constant::IN_BYTES_PER_QUAD as u64;
const OUT_BYTES_PER_QUAD: u64 = crate::constant::OUT_BYTES_PER_QUAD as u64;

pub fn to_zero_padded_size(payload_size: u64) -> u64 {
    let size = max(payload_size, MIN_PAYLOAD_SIZE);
    let highest_bit = u64::BITS - size.leading_zeros();

    let bound = from_piece_size(2u64.pow(highest_bit));

    if size <= bound {
        bound
    } else {
        from_piece_size(2u64.pow(highest_bit + 1))
    }
}

pub fn to_piece_size(size: u64) -> u64 {
    (to_zero_padded_size(size) * OUT_BITS_FR as u64) / IN_BITS_FR as u64
}

pub fn from_piece_size(size: u64) -> u64 {
    (size * IN_BITS_FR as u64) / OUT_BITS_FR as u64
}

pub fn pad(source: &[u8], output: &mut [u8]) -> Result<usize, ()> {
    let size = to_zero_padded_size(source.len() as u64);
    let quad_count = size / IN_BYTES_PER_QUAD;

    println!(
        "count={:?} size={:?} payload_size={:?} capacity={:?}",
        quad_count,
        size,
        source.len(),
        output.len()
    );

    for n in 0..quad_count {
        let read_offset = (n * IN_BYTES_PER_QUAD) as usize;
        let write_offset = (n * OUT_BYTES_PER_QUAD) as usize;

        // First 31 bytes + 6 bits are taken as-is (trimmed later)
        output[write_offset..write_offset + 32]
            .copy_from_slice(&source[read_offset..read_offset + 32]);

        // first 2-bit "shim" forced into the otherwise identical output
        output[write_offset + 31] &= 0b00111111;

        // copy next Fr32 preceded with the last two bits of the previous Fr32
        for i in 32..64 {
            output[write_offset + i] =
                (source[read_offset + i] << 2) | (source[read_offset + i - 1] >> 6);
        }

        // next 2-bit shim
        output[write_offset + 63] &= 0b00111111;

        for i in 64..96 {
            output[write_offset + i] =
                (source[read_offset + i] << 4) | (source[read_offset + i - 1] >> 4);
        }

        // next 2-bit shim
        output[write_offset + 95] &= 0b00111111;

        for i in 96..127 {
            output[write_offset + i] =
                (source[read_offset + i] << 6) | (source[read_offset + i - 1] >> 2);
        }

        // we shim last 2-bits by shifting the last byte by two bits
        output[write_offset + 127] = source[read_offset + 126] >> 2;
    }

    Result::Ok(size.try_into().unwrap())
}

pub fn unpad(source: &[u8], out: &mut [u8]) -> Result<usize, ()> {
    let chunks = source.len() / 128;
    for chunk in 0..chunks {
        let in_off_next = chunk * 128 + 1;
        let out_off = chunk * 127;

        let mut at = source[chunk * 128];

        for i in 0..32 {
            let next = source[i + in_off_next];

            out[out_off + i] = at;

            at = next;
        }

        out[out_off + 31] |= at << 6;

        for i in 32..64 {
            let next = source[i + in_off_next];

            out[out_off + i] = at >> 2;
            out[out_off + i] |= next << 6;

            at = next;
        }

        out[out_off + 63] ^= (at << 6) ^ (at << 4);

        for i in 64..96 {
            let next = source[i + in_off_next];

            out[out_off + i] = at >> 4;
            out[out_off + i] |= next << 4;

            at = next;
        }

        out[out_off + 95] ^= (at << 4) ^ (at << 2);

        for i in 96..127 {
            let next = source[i + in_off_next];

            out[out_off + i] = at >> 6;
            out[out_off + i] |= next << 2;

            at = next;
        }
    }

    Result::Ok(source.len() * IN_BITS_FR / OUT_BITS_FR)
}

#[cfg(test)]
mod tests {
    use crate::fr32;

    #[test]
    fn test_to_zero_padded_size() {
        assert_eq!(fr32::to_zero_padded_size(0), 127);
        assert_eq!(fr32::to_zero_padded_size(64), 127);
        assert_eq!(fr32::to_zero_padded_size(127), 127);
        assert_eq!(fr32::to_zero_padded_size(128), 254);
    }

    fn test_simple_short() {
        // Source is shorter than 1 padding cycle.
        let data = vec![3u8; 30];
        let mut padded = Vec::<u8>::new();
        padded.resize(256, 0);

        fr32::pad(&data, &mut padded).expect("in-memory read failed");
        // assert_eq!(padded.len(), 32);
        // assert_eq!(&data[..], &padded[..30]);
    }
}

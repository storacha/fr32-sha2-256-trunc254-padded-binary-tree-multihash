use crate::constant::NODE_SIZE;

const BYTES_PER_NODE: u64 = NODE_SIZE as u64;

pub const fn from_height(height: u32) -> u64 {
    2u64.pow(height) * BYTES_PER_NODE
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

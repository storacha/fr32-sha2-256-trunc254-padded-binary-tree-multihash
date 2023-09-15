use crate::constant::NODE_SIZE;
use crate::tree::MerkleTreeNode;
use js_sys::Uint8Array;
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Crypto, SubtleCrypto};

type Node = [u8; NODE_SIZE];

pub fn crypto() -> Crypto {
    use wasm_bindgen::JsCast;

    let object = js_sys::Reflect::get(&js_sys::global().into(), &"crypto".into()).expect("crypto");

    web_sys::Crypto::from(object)
}

#[wasm_bindgen(module = "/src/sha256.js")]
extern "C" {
    fn sha256_into(payload: &[u8], dst: &mut [u8]) -> js_sys::Promise;
}

#[wasm_bindgen]
pub async fn sha256(data: &[u8], dst: &mut [u8]) {
    let promise = sha256_into(data, dst);
    // let promise = crypto()
    //     .subtle()
    //     .digest_with_str_and_u8_array("SHA-256", data)
    //     .expect("Get digest");

    JsFuture::from(promise).await.expect("resolve promise");
}

#[cfg(test)]
mod tests {

    use crate::hasher_async::sha256;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_sha256() {
        let mut digest = [0u8; 32];
        let _digest = sha256(&mut [0u8; 64], &mut digest).await;

        assert_eq!(
            digest,
            [
                245, 165, 253, 66, 209, 106, 32, 48, 39, 152, 239, 110, 211, 9, 151, 155, 67, 0,
                61, 35, 32, 217, 240, 232, 234, 152, 49, 169, 39, 89, 251, 75
            ]
        )
    }
    // #[wasm_bindgen_test]
    // async fn test_crypto() {
    //     use wasm_bindgen::JsCast;
    //     print!("------------ start ------------");
    //     let global = js_sys::global();
    //     let g = js_sys::Reflect::get(&global, &"crypto".into()).expect("got crypto field");

    //     assert!(g.is_undefined());

    //     let c = js_sys::eval(&"globalThis").expect("access crypto field");
    //     assert!(c.is_truthy());

    //     let crypto = web_sys::Crypto::from(c);
    // }
}

// struct Channel {
//     buffer: Option<Node>,
//     out: Vec<Node>,
// }

// impl Channel {
//     fn put(&mut self, node: &Node) {
//         if self.buffer.is_none() {
//             self.buffer = Some(node);
//         } else {
//             let buffer = self.buffer.unwrap();
//             let parent = MerkleTreeNode::new(buffer, node);
//             self.out.push(parent);
//             self.buffer = None;
//         }
//     }
// }

// struct Job<'a> {
//     job: Future<()>,
//     node: Option<Node>,
//     in_offset: usize,
//     out_offset: usize,
//     pending: Vec<Future<usize>>,
//     output: Vec<Node>,
// }

// impl Job<'a> {
//     pub fn new() -> Self {
//         Self {
//             job: async {},
//             input: Vec::new(),
//             output: Vec::new(),
//         }
//     }

//     pub async fn resume(&mut self) {}

//     pub fn put(&mut self, node: &Node) {
//         if (self.node.is_none()) {
//             self.node = Some(node);
//         } else {
//             let mut payload = vec![0u8; left.0.len() + right.0.len()];
//             payload[..left.0.len()].copy_from_slice(&left.0);
//             payload[left.0.len()..].copy_from_slice(&right.0);

//             self.in_offset += 1;
//             let id = self.in_offset;
//             let task = async {
//                 return id;
//             };

//             self.pending.push(task);
//             let right = node;
//             self.output.resize(new_len, None());
//         }
//     }

//     pub async fn take(&mut self) -> Option<Node> {
//         self.output.pop()
//     }

//     pub fn finish(&mut self) {
//         self.job.await;
//     }
// }

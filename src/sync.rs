use crate::constant::NODE_SIZE;
use crate::tree::MerkleTreeNode;
use std::future::Future;
use wasm_streams::transform;
use web_sys::TransformStream;

type Node = [u8; NODE_SIZE];

pub fn setup() {
    let pipe = TransformStream::new();
}

pub fn open() -> transform::TransformStream {
    let raw = transform::sys::TransformStream::new();
    transform::TransformStream::from_raw(raw)
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use web_sys::{ReadableStreamDefaultReader, TransformStream, WritableStreamDefaultWriter};
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use crate::sync::open;
    use futures_util::io::AsyncWriteExt;
    use futures_util::stream::StreamExt;
    use wasm_bindgen::prelude::*;
    use wasm_streams::transform;

    // #[wasm_bindgen_test]
    // async fn test_transform_stream() {
    //     let pipe = TransformStream::new().expect("Get transform stream");

    //     let writable = pipe.writable();
    //     let readable = pipe.readable();

    //     let mut writer = WritableStreamDefaultWriter::new(&writable).expect("Get writer");
    //     let mut reader = ReadableStreamDefaultReader::new(&readable);

    //     writer.write_with_chunk(&vec![0, 1]);
    // }
    #[wasm_bindgen_test]
    async fn test_streams() {
        let channel = open();
        let writer = &mut channel.writable().into_async_write();

        let readable = channel.readable();
        let reader = readable.into_async_read();

        writer.write(&[0, 1, 2]);

        // let out = reader.take(1).next().await;
        // let r = out.unwrap().unwrap();

        // assert_eq!(r, JsValue::from(&[0, 1, 2].as_ref()));
    }
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

use bincode;
use serde::{Deserialize, Serialize};
use std::env;
use std::io;

const BYTES: &[u8; 55510032] = include_bytes!(concat!(env!("OUT_DIR"), "/mnist.bin"));

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MnistData {
    pub train_images: Vec<Vec<u8>>,
    pub test_images: Vec<Vec<u8>>,
    pub train_labels: Vec<u8>,
    pub test_labels: Vec<u8>,
}

pub fn load_mnist_data() -> Result<MnistData, io::Error> {
    let mnist = bincode::deserialize(BYTES).unwrap();
    Ok(mnist)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::*;
        let mnist = load_mnist_data().unwrap();
        assert_eq!(mnist.train_images.len(), 60000);
    }
}

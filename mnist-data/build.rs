use curl::easy::Easy;
use flate2::bufread::GzDecoder;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;

const N_TRING: usize = 60000;
const N_TEST: usize = 10000;

const FILES: [&str; 4] = [
    "train-images-idx3-ubyte.gz",
    "train-labels-idx1-ubyte.gz",
    "t10k-images-idx3-ubyte.gz",
    "t10k-labels-idx1-ubyte.gz",
];

const URL: &str = "http://yann.lecun.com/exdb/mnist";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct MNIST {
    pub train_images: Vec<Vec<u8>>,
    pub test_images: Vec<Vec<u8>>,
    pub train_labels: Vec<u8>,
    pub test_labels: Vec<u8>,
}

fn download_mnist(out_dir: &str) -> Result<(), io::Error> {
    let mut handle = Easy::new();
    for filename in FILES {
        let file_url = format!("{}/{}", URL, filename);
        let file_path = format!("{}/{}", out_dir, filename);
        let mut file = File::create(file_path)?;
        handle.url(&file_url).unwrap();
        handle.write_function(move |data| {
            file.write_all(data).unwrap();
            Ok(data.len())
        })?;
        handle.perform()?;
    }
    Ok(())
}

fn decode_mnist(out_dir: &str) -> Result<MNIST, io::Error> {
    let train_images_gz = fs::read(format!("{}/{}", out_dir, FILES[0]))?;
    assert_eq!(train_images_gz.len(), 9912422);
    let train_labels_gz = fs::read(format!("{}/{}", out_dir, FILES[1]))?;
    assert_eq!(train_labels_gz.len(), 28881);
    let test_images_gz = fs::read(format!("{}/{}", out_dir, FILES[2]))?;
    assert_eq!(test_images_gz.len(), 1648877);
    let test_labels_gz = fs::read(format!("{}/{}", out_dir, FILES[3]))?;
    assert_eq!(test_labels_gz.len(), 4542);
    let train_images = decode_images(&train_images_gz, N_TRING as u32)?;
    assert_eq!(train_images.len(), N_TRING);
    let test_images = decode_images(&test_images_gz, N_TEST as u32)?;
    assert_eq!(test_images.len(), N_TEST);
    let train_labels = decode_labels(&train_labels_gz, N_TRING as u32)?;
    assert_eq!(train_labels.len(), N_TRING);
    let test_labels = decode_labels(&test_labels_gz, N_TEST as u32)?;
    assert_eq!(test_labels.len(), N_TEST);
    Ok(MNIST {
        train_images,
        test_images,
        train_labels,
        test_labels,
    })
}

fn decode_labels(bytes: &[u8], n: u32) -> Result<Vec<u8>, io::Error> {
    let mut decoder = GzDecoder::new(bytes);
    let mut buf: [u8; 4] = [0; 4];

    decoder.read_exact(&mut buf)?;
    let magic = as_u32_be(&buf);
    assert_eq!(magic, 2049);

    decoder.read_exact(&mut buf)?;
    let n_labels = as_u32_be(&buf);
    assert_eq!(n_labels, n);

    let mut res = vec![0; n_labels as usize];
    decoder.read_exact(&mut res)?;
    Ok(res)
}

fn decode_images(bytes: &[u8], n: u32) -> Result<Vec<Vec<u8>>, io::Error> {
    let mut decoder = GzDecoder::new(bytes);
    let mut buf: [u8; 4] = [0; 4];

    decoder.read_exact(&mut buf)?;
    let magic = as_u32_be(&buf);
    assert_eq!(magic, 2051);

    decoder.read_exact(&mut buf)?;
    let n_images = as_u32_be(&buf);
    assert_eq!(n_images, n);

    decoder.read_exact(&mut buf)?;
    let n_rows = as_u32_be(&buf);
    assert_eq!(n_rows, 28);

    decoder.read_exact(&mut buf)?;
    let n_cols = as_u32_be(&buf);
    assert_eq!(n_cols, 28);

    let image_size = (n_rows * n_cols) as usize;
    let mut res = vec![];
    for _ in 0..n_images {
        let mut image = vec![0; image_size];
        decoder.read_exact(&mut image)?;
        res.push(image);
    }
    Ok(res)
}

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + ((array[3] as u32) << 0)
}

fn create_mnist(out_dir: &str, mnist: MNIST) -> Result<(), io::Error> {
    let bytes = bincode::serialize(&mnist).unwrap();
    let output_file = format!("{}/{}", out_dir, "mnist.bin");
    fs::write(output_file, bytes)?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let out_dir = env::var("OUT_DIR").unwrap();
    download_mnist(&out_dir)?;
    let mnist = decode_mnist(&out_dir)?;
    create_mnist(&out_dir, mnist)?;
    Ok(())
}

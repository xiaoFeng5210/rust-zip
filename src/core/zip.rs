use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::io::{Read, Write};

trait Zipper {
    fn compression() {
        let data = "hello, world";
        let mut encoder = GzEncoder::new(Vec::new(), flate2::Compression::default());
    }
}

#[derive(Debug)]
pub struct Zip {
    pub zip_path: String,    // 要压缩的文件的路径
    pub target_name: String, // 压缩后的文件名
}

impl Zip {}

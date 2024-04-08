use flate2::write::GzEncoder;
use flate2::Compression;
use std::io;
use std::io::prelude::*;

trait Zipper {
    fn compression() -> Result<(), std::io::Error> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(b"Hello, World!")?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Zip {
    pub zip_path: String,    // 要压缩的文件的路径
    pub target_name: String, // 压缩后的文件名
}

impl Zip {}

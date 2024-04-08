use flate2::write::GzEncoder;
use flate2::Compression;
use std::io;
use std::io::prelude::*;

pub trait Zipper {
    fn compression() -> Result<(), std::io::Error> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(b"Hello, World!")?;
        Ok(())
    }

    fn test(&self) -> String {
        String::from("Hello, World!")
    }
}

#[derive(Debug)]
pub struct Zip {
    pub zip_path: String,    // 要压缩的文件的路径
    pub target_name: String, // 压缩后的文件名
}

impl Zipper for Zip {
    fn test(&self) -> String {
        println!("{}", self.zip_path);
        String::from("Hello, World!")
    }
}

fn test() {
    let zip = Zip {
        zip_path: String::from("src/core/zip.rs"),
        target_name: String::from("zip.rs"),
    };
}

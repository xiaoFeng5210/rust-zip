use flate2::write::GzEncoder;
use flate2::Compression;
use std::io;
use std::io::prelude::*;

pub trait Zipper<RHS = Self> {
    fn compression(&self) -> Result<(), std::io::Error> {
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

#[derive(Debug)]
pub struct zip2 {
    pub zip_path: String,    // 要压缩的文件的路径
    pub target_name: String, // 压缩后的文件名
}
impl Zipper for zip2 {
    fn test(&self) -> String {
        println!("{}", self.zip_path);
        String::from("Hello, World2!")
    }
}

#[derive(Debug)]
pub struct zip3 {
    pub zip_path: String,    // 要压缩的文件的路径
    pub target_name: String, // 压缩后的文件名
}
impl zip3 {
    fn a() {
        println!("a");
    }
}

pub fn test_a() {
    let x = zip3 {
        zip_path: String::from("src/core/zip.rs"),
        target_name: String::from("zip.rs"),
    };
    zip3::a();
}

pub fn some_function<T>(item: &T)
where
    T: Zipper,
{
    item.compression().unwrap();
}

// 特征对象, 如果特征对象有一个方法属于静态方法，那么我们不能把它变为特征对象类型
pub fn some_object(item: &dyn Zipper) {
    println!("{}", item.test());
}

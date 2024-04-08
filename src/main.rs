use rust_zip::core::zip::Zip;
use rust_zip::core::zip::Zipper;
fn main() {
    let zip = Zip {
        zip_path: String::from("src/core/zip.rs"),
        target_name: String::from("zip.rs"),
    };
    Zip::compression().unwrap();
    zip.test();
}

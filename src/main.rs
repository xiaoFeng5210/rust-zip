use rust_zip::core::zip::Zip;
fn main() {
    let zip = Zip {
        zip_path: String::from("src/core/zip.rs"),
        target_name: String::from("zip.rs"),
    };
    zip.compression().unwrap();
}

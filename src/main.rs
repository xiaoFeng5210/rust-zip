use rust_zip::core::zip::Zipper;
use rust_zip::core::zip::{some_function, some_object, Zip};
fn main() {
    let zip = Zip {
        zip_path: String::from("src/core/zip.rs"),
        target_name: String::from("zip.rs"),
    };
    // some_function(&zip);
    some_object(&zip);
}

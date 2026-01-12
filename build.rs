use std::{env, fs, path::Path};

use types::TestStruct;

fn main() {
    let test = TestStruct::new("hello from build".into());

    let bytes = test.as_bytes();

    // once i have this shit figured out
    // let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // let path = Path::new(&manifest_dir).join("data/input.txt");
    // let data = fs::read_to_string(path).unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("routes.rkyv");
    fs::write(&dest_path, &bytes).unwrap();
}

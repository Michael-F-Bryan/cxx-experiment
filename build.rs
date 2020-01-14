use std::{env, path::PathBuf};

fn main() {
    let _project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
}

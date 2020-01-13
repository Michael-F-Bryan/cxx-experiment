use cc::Build;
use std::{env, path::PathBuf};

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let bzip2_dir = project_dir.join("vendor").join("bzip2");

    Build::new()
        .file(bzip2_dir.join("blocksort.c"))
        .file(bzip2_dir.join("huffman.c"))
        .file(bzip2_dir.join("crctable.c"))
        .file(bzip2_dir.join("randtable.c"))
        .file(bzip2_dir.join("compress.c"))
        .file(bzip2_dir.join("decompress.c"))
        .file(bzip2_dir.join("bzlib.c"))
        .define("_FILE_OFFSET_BITS", Some("64"))
        .warnings(false)
        .include(&bzip2_dir)
        .compile("bz2");
}

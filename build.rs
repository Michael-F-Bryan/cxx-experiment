use cmake::Config;
use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let vendor = project_dir.join("vendor");

    let libtorrent = vendor.join("libtorrent");
    let dst = Config::new(&libtorrent)
        .define("BUILD_SHARED_LIBS", "FALSE")
        .define("static_runtime", "TRUE")
        .uses_cxx11()
        .build();

    println!("cargo:warning=Output directory: {}", dst.display());
    add_to_search_path(&dst);
    println!("cargo:rustc-link-lib=static=torrent-rasterbar");
}

fn add_to_search_path(path: &Path) {
    println!("cargo:rustc-link-search=native={}", path.display());
    println!(
        "cargo:rustc-link-search=native={}",
        path.join("build").display()
    );
}

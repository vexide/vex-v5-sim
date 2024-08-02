fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    println!("cargo:rustc-link-search=native={manifest_dir}/link");

    // println!("cargo:rustc-link-arg=-Tkernel.ld");
    // println!("cargo:rerun-if-changed=-Tkernel.ld");

    println!("cargo:rustc-link-lib=static=xil");
    println!("cargo:rustc-link-lib=static=xiltimer");
    println!("cargo:rustc-link-lib=static=xilstandalone");
}

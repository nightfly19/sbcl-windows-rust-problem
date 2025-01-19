fn main() {
    // Link to the C standard library explicitly
    //println!("cargo:rustc-link-lib=c");
    //println!("cargo:rustc-flags=-C target-feature=+crt-static");
     println!("cargo:rustc-link-args=-rdynamic");
}

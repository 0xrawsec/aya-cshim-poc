use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let _ = Command::new("clang")
        .arg("-I")
        .arg("src/")
        .arg("-O2")
        .arg("-emit-llvm")
        .arg("-target")
        .arg("bpf")
        .arg("-c")
        .arg("-g")
        .arg("src/shim.c")
        .arg("-o")
        .arg(format!("{out_dir}/shim.o"))
        .status()
        .expect("Failed to compile the C-shim");

    println!("cargo:rustc-link-search=native={out_dir}");
    println!("cargo:rustc-link-lib=link-arg={out_dir}/shim.o");
    println!("cargo:rerun-if-changed=src/shim.c");
}

fn main() {
    cc::Build::new()
        .file("opm.c")
        .flag("-fwrapv")
        .compile("opm");

    println!("cargo:rerun-if-changed=opm.c");
    println!("cargo:rerun-if-changed=opm.h");
}

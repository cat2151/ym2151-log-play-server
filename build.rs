fn main() {
    cc::Build::new()
        .file("opm.c")
        .flag("-fwrapv")
        .opt_level(3)
        .compile("opm");

    println!("cargo:rerun-if-changed=opm.c");
    println!("cargo:rerun-if-changed=opm.h");
}

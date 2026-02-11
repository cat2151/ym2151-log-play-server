fn main() {
    cc::Build::new()
        .file("call_opm_clock_64times.c")
        .flag("-fwrapv")
        .compile("opm");

    println!("cargo:rerun-if-changed=call_opm_clock_64times.c");
    println!("cargo:rerun-if-changed=opm.c");
    println!("cargo:rerun-if-changed=opm.h");
}

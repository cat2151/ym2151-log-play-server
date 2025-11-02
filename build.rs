// Build script to compile Nuked-OPM C code

fn main() {
    // Compile opm.c using the cc crate
    cc::Build::new()
        .file("opm.c")
        .flag("-fwrapv")
        .compile("opm");

    // Tell Cargo to rerun this build script if these files change
    println!("cargo:rerun-if-changed=opm.c");
    println!("cargo:rerun-if-changed=opm.h");
}

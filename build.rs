// Build script to compile Nuked-OPM C code

fn main() {
    // Compile opm.c using the cc crate with maximum optimizations
    // Note: -fwrapv flag ensures defined behavior for signed integer overflow,
    // which is required for correct YM2151 emulation calculations
    cc::Build::new()
        .file("opm.c")
        .flag("-fwrapv")
        .opt_level(3) // Enable -O3 optimization for C code (CRITICAL for performance)
        .compile("opm");

    // Tell Cargo to rerun this build script if these files change
    println!("cargo:rerun-if-changed=opm.c");
    println!("cargo:rerun-if-changed=opm.h");
}

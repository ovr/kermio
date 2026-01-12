fn main() {
    // Compile the cxx bridge (bridge.h contains inline functions)
    cxx_build::bridge("src/sys.rs")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-stdlib=libc++")
        // Add JSI include paths from local include directory
        .include("include")
        .compile("jsi-rs");

    println!("cargo:rerun-if-changed=src/sys.rs");
    println!("cargo:rerun-if-changed=src/bridge.h");
    println!("cargo:rerun-if-changed=include");
}

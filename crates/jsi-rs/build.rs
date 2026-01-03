fn main() {
    // Only compile the cxx bridge - no custom C++ code needed
    // Just expose the JSI types from hermes-vendor/API/jsi
    cxx_build::bridge("src/sys.rs")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-stdlib=libc++")
        // Add JSI include paths from hermes-vendor
        .include("../hermes-engine/hermes-vendor/API")
        .include("../hermes-engine/hermes-vendor/API/jsi")
        .compile("jsi-rs");

    println!("cargo:rerun-if-changed=src/sys.rs");
}

use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = PathBuf::from(&manifest_dir);

    let hermes_src = manifest_path.join("hermes-vendor");
    if !hermes_src.exists() {
        eprintln!("ERROR: hermes-vendor directory not found!");
        eprintln!("The hermes-engine crate requires the Hermes source code.");
        eprintln!("");
        eprintln!("This directory should be included in the published crate.");
        eprintln!("If you're developing from source, initialize the git submodule:");
        eprintln!("  git submodule update --init --recursive");
        eprintln!("");
        eprintln!("After initializing the submodule, apply patches:");
        eprintln!("  ./apply-patches.sh");
        panic!("hermes-vendor not found");
    }

    let dst = cmake::Config::new(&hermes_src)
        .generator("Ninja")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("HERMES_ENABLE_DEBUGGER", "OFF")
        .define("HERMES_ENABLE_INTL", "OFF")
        .define("HERMES_BUILD_APPLE_FRAMEWORK", "OFF")
        .define("HERMES_BUILD_SHARED_JSI", "OFF")
        .define("HERMES_ENABLE_TEST_SUITE", "OFF")
        .build();

    // Setup includes for cxx bridge
    let hermes_api_include = hermes_src.join("API");
    let jsi_include = hermes_src.join("API/jsi");
    let installed_include = dst.join("include");
    let hermes_public_include = hermes_src.join("include");

    // Build the cxx bridge (header-only, no C++ files to compile)
    cxx_build::bridge("src/bridge.rs")
        .include(&hermes_api_include)
        .include(&jsi_include)
        .include(&installed_include)
        .include(&hermes_public_include)
        .include(&manifest_path)
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-stdlib=libc++")
        .compile("hermes_bridge");

    // Tell cargo to link the Hermes libraries
    // Note: Hermes build creates libraries in various subdirectories under build/
    let build_dir = dst.join("build");

    // Link search paths for all Hermes libraries
    println!(
        "cargo:rustc-link-search=native={}/API/hermes",
        build_dir.display()
    );
    println!("cargo:rustc-link-search=native={}/jsi", build_dir.display());
    println!("cargo:rustc-link-search=native={}/lib", build_dir.display());
    println!(
        "cargo:rustc-link-search=native={}/lib/VM",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/Inst",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/Platform",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/Platform/Unicode",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/Platform/Intl",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/Regex",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/Support",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/BCGen",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/BCGen/HBC",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/InternalJavaScript",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/ADT",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/Parser",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/AST",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/SourceMap",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/FrontEndDefs",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/Sema",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/CompilerDriver",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib/VM/Instrumentation",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/public/hermes/Public",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/external/llvh/lib/Support",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/external/llvh/lib/Demangle",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/external/dtoa",
        build_dir.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/external/boost/boost_1_86_0/libs/context",
        build_dir.display()
    );

    // Link only essential libraries for runtime creation
    // The order matters - dependencies come after dependents
    println!("cargo:rustc-link-lib=static=hermesapi");
    println!("cargo:rustc-link-lib=static=jsi");

    // CompileJS library for bytecode compilation
    let compilejs_lib = dst.join("build/API/hermes/libcompileJS.a");
    if compilejs_lib.exists() {
        println!(
            "cargo:rustc-link-search=native={}/build/API/hermes",
            dst.display()
        );
        println!("cargo:rustc-link-lib=static=compileJS");
    }

    // Try to link the full Hermes compiler if available
    if build_dir.join("lib/libhermescompiler.a").exists() {
        println!("cargo:rustc-link-lib=static=hermescompiler");
    }

    // VM Runtime and its dependencies
    println!("cargo:rustc-link-lib=static=hermesVMRuntime");
    println!("cargo:rustc-link-lib=static=hermesPublic");
    println!("cargo:rustc-link-lib=static=hermesPlatform");
    println!("cargo:rustc-link-lib=static=hermesPlatformUnicode");
    println!("cargo:rustc-link-lib=static=hermesSupport");
    println!("cargo:rustc-link-lib=static=hermesRegex");
    println!("cargo:rustc-link-lib=static=hermesADT");
    println!("cargo:rustc-link-lib=static=hermesInst");
    println!("cargo:rustc-link-lib=static=hermesFrontEndDefs");

    // Parser and AST for compiling JS source
    println!("cargo:rustc-link-lib=static=hermesParser");
    println!("cargo:rustc-link-lib=static=hermesAST");
    println!("cargo:rustc-link-lib=static=hermesSourceMap");

    // Backend for bytecode generation
    println!("cargo:rustc-link-lib=static=hermesHBCBackendLean");
    println!("cargo:rustc-link-lib=static=hermesBackend");
    println!("cargo:rustc-link-lib=static=hermesOptimizer");
    println!("cargo:rustc-link-lib=static=hermesFrontend");

    // Internal bytecode
    println!("cargo:rustc-link-lib=static=hermesInternalBytecode");

    // LLVH support
    println!("cargo:rustc-link-lib=static=LLVHSupport");
    println!("cargo:rustc-link-lib=static=LLVHDemangle");

    // External dependencies
    println!("cargo:rustc-link-lib=static=dtoa");
    println!("cargo:rustc-link-lib=static=boost_context");

    // Link C++ standard library
    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
        // Link macOS frameworks required by Hermes
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=stdc++");
    }

    // Rebuild if bridge files change
    println!("cargo:rerun-if-changed=src/bridge.rs");
    println!("cargo:rerun-if-changed=src/hermes_bridge.h");
    println!("cargo:rerun-if-changed=src/hermes_bridge.cpp");
}

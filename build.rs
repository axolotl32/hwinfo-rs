use std::env;
use std::path::PathBuf;

fn main() {
    let mut config = cmake::Config::new("hwinfo");

    config.define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreadedDLL");

    let dst = config.build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());

    println!("cargo:rustc-link-lib=static=hwinfo");
 
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=ole32");
        println!("cargo:rustc-link-lib=dylib=oleaut32");
        println!("cargo:rustc-link-lib=dylib=wbemuuid");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    }

    let header_path = dst.join("include").join("hwinfo").join("hwinfo_c.h");
    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().expect("Path to header is not valid UTF-8"))
        .allowlist_function("get_.*")
        .allowlist_function("free_.*")
        .allowlist_type("C_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
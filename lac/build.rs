use std::env;

fn main() {
    let sdk_stub = env::var("CARGO_FEATURE_SDK_STUB").is_ok();

    if sdk_stub {
        println!("cargo:rerun-if-changed=tests/c_stubs/");

        cc::Build::new()
            .cpp(true)
            .flag("-std=c++17")
            .flag("-fno-exceptions")
            .flag("-fno-rtti")
            .file("tests/c_stubs/device.cc")
            .file("tests/c_stubs/chip_sdk.cc")
            .file("tests/c_stubs/chip_cfg.cc")
            .include("tests/c_stubs")
            .compile("chip_sdk_stub");
    }

    #[cfg(feature = "bindgen")]
    {
        let bindings = bindgen::Builder::default()
            .header("tests/c_stubs/chip_sdk.h")
            .generate()
            .expect("Unable to generate bindings");
        bindings
            .write_to_file("src/ffi/bindings_gen.rs")
            .expect("Couldn't write bindings!");
    }
}

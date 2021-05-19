use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    build_open62541();
    generate_module("client");
    generate_module("server");
}

fn out_path() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn build_open62541() {
    let open62541_path = Config::new("open62541").generator("Ninja").build();

    println!(
        "cargo:rustc-link-search=native={}/lib",
        open62541_path.display()
    );
    println!("cargo:rustc-link-lib=static=open62541");
}

fn generate_module(module: &str) {
    let open62541_path = out_path();
    let bindings = bindgen::Builder::default()
        .header(format!("{}.h", module))
        .clang_arg(format!("-I{}/include", open62541_path.display()))
        .allowlist_function("UA_.*")
        .allowlist_type("UA_.*")
        .allowlist_var("UA_.*")
        .generate()
        .expect("Unable to generate open62541 bindings");

    bindings
        .write_to_file(out_path().join(format!("open62541_{}.rs", module)))
        .expect("Couldn't write open62541.rs");
}

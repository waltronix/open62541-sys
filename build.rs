use cmake::Config;
use std::path::PathBuf;
use std::{env, path::Path};

const OPEN6251_REPOSITORY_URL: &str = "https://github.com/open62541/open62541.git";
const OPEN6251_REPOSITORY_PATH: &str = "open62541_src";
const OPEN6251_TAG: &str = "v1.2.2";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    get_open62541();
    build_open62541();
    // generate_module("client");
    generate_module("server");
}

fn out_path() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn get_open62541() {
    println!("get_open62541");

    let out = std::process::Command::new("git")
        .args(&["clone", OPEN6251_REPOSITORY_URL, OPEN6251_REPOSITORY_PATH])
        .output()
        .expect("could not clone repsitory");
    println!("out: {:?}", out);

    std::process::Command::new("git")
        .args(&["checkout", OPEN6251_TAG])
        .current_dir(OPEN6251_REPOSITORY_PATH)
        .output()
        .expect("could not check out tag");

    let patch_dir = std::path::Path::new("patches");
    println!("patch_dir: {:?}", patch_dir);
    for entry in std::fs::read_dir(patch_dir).unwrap() {
        let patch = entry.unwrap().path();
        println!("patch: {:?}", patch);
        let patch_file = Path::new("..").join(patch);
        let patch_file = patch_file.as_os_str().to_str().unwrap();
        std::process::Command::new("git")
            .args(&["apply", patch_file])
            .current_dir(OPEN6251_REPOSITORY_PATH)
            .output()
            .expect("could not apply patches");
    }
}

fn build_open62541() {
    let open62541_path = Config::new(OPEN6251_REPOSITORY_PATH)
        .generator("Ninja")
        .very_verbose(true)
        .build();

    println!(
        "cargo:rustc-link-search=native={}/lib",
        open62541_path.display()
    );
    println!("cargo:rustc-link-lib=static=open62541");
    //sf
}

fn generate_module(module: &str) {
    let open62541_path = out_path();
    let bindings = bindgen::Builder::default()
        .header(format!("{}.h", module))
        .clang_arg(format!("-I{}/include", open62541_path.display()))
        .generate_inline_functions(true)
        .allowlist_function("UA_.*")
        .allowlist_type("UA_.*")
        .allowlist_var("UA_.*")
        .generate()
        .expect("Unable to generate open62541 bindings");

    bindings
        .write_to_file(out_path().join(format!("open62541_{}.rs", module)))
        .expect("Couldn't write open62541.rs");
}

use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let bindings = bindgen::builder()
        .header("wrapper.h")
        .clang_arg("-I./third_party/machine-guest-tools/sys-utils/libcmt/include")
        .clang_arg("-I./third_party/machine-guest-tools/sys-utils/libcmt/include/libcmt")
        .allowlist_function("cmt_.*")
        .allowlist_type("cmt_.*")
        .allowlist_var("CMT_.*")
        .allowlist_var("HTIF_.*")
        .layout_tests(false)
        .generate()
        .expect("Unable to generate libcmt bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    println!("cargo:rerun-if-changed=wrapper.h");
    println!(
        "cargo:out-dir={}",
        PathBuf::from(env::var("OUT_DIR")?).display()
    );

    Ok(())
}

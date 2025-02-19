use std::env;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let bindings = bindgen::Builder::default()
        .header("src/c_vec.h")
        .generate()?;
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    cc::Build::new().file("src/c_vec.c").compile("c_vec");

    println!("cargo:rerun-if-changed=src/c_vec.c");
    println!("cargo:rerun-if-changed=src/c_vec.h");
    Ok(())
}

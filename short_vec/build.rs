use std::env;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let bindings = bindgen::Builder::default()
        .header("src/short_vec.h")
        .generate()?;
    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    cc::Build::new()
        .file("src/short_vec.c")
        .compile("short_vec");

    println!("cargo:rerun-if-changed=src/short_vec.c");
    println!("cargo:rerun-if-changed=src/short_vec.h");
    Ok(())
}

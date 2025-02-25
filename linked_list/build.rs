use std::env;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let bindings = bindgen::Builder::default()
        .header("src/linked_list.h")
        .generate()?;
    let out_path = PathBuf::from(env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    cc::Build::new()
        .file("src/linked_list.c")
        .compile("linked_list");

    println!("cargo:rerun-if-changed=src/linked_list.c");
    println!("cargo:rerun-if-changed=src/linked_list.h");
    Ok(())
}

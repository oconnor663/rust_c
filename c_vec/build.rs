use std::env;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let bindings = bindgen::Builder::default().header("src/vec.h").generate()?;
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    cc::Build::new().file("src/vec.c").compile("vec");

    println!("cargo:rerun-if-changed=src/vec.c");
    println!("cargo:rerun-if-changed=src/vec.h");
    Ok(())
}

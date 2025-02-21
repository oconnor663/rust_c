fn main() {
    cc::Build::new().file("src/add_one.c").compile("add_one");

    println!("cargo:rerun-if-changed=src/add_one.c");
}

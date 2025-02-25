fn main() {
    cc::Build::new().file("src/lib.c").compile("add_two");

    println!("cargo:rerun-if-changed=src/lib.c");
}

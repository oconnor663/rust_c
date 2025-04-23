fn main() {
    cc::Build::new().file("src/add_two.c").compile("add_two");

    println!("cargo:rerun-if-changed=src/add_two.c");
}

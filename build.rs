fn main() {
    println!("cargo:rustc-link-arg=-Wl,-T,linker.ld");
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-arg=-Wl,--nmagic");
    println!("cargo:rustc-link-arg=-Wl,--build-id=none");
}

fn main() {
    #[cfg(target_arch = "x86")]
    println!("cargo:rustc-link-search=./lib/x86");
    #[cfg(target_arch = "x86_64")]
    println!("cargo:rustc-link-search=./lib/x64");
    #[cfg(target_arch = "arm")]
    println!("cargo:rustc-link-search=./lib/ARM");
    #[cfg(target_arch = "aarch64")]
    println!("cargo:rustc-link-search=./lib/ARM64");
}
fn main() {
    #[cfg(target_arch = "x86")]
    println!("cargo:rustc-link-search=./vendor/dstorage-1.2.1/lib/x86");
    #[cfg(target_arch = "x86_64")]
    println!("cargo:rustc-link-search=./vendor/dstorage-1.2.1/lib/x64");
    #[cfg(target_arch = "arm")]
    println!("cargo:rustc-link-search=./vendor/dstorage-1.2.1/lib/ARM");
    #[cfg(target_arch = "aarch64")]
    println!("cargo:rustc-link-search=./vendor/dstorage-1.2.1/lib/ARM64");

    let target_dstorage = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("../../..")
        .join("dstorage.dll");

    #[cfg(target_arch = "x86")]
    std::fs::copy("./vendor/dstorage-1.2.1/bin/x86/dstorage.dll", target_dstorage).unwrap();

    #[cfg(target_arch = "x86_64")]
    std::fs::copy("./vendor/dstorage-1.2.1/bin/x64/dstorage.dll", target_dstorage).unwrap();

    #[cfg(target_arch = "arm")]
    std::fs::copy("./vendor/dstorage-1.2.1/bin/ARM/dstorage.dll", target_dstorage).unwrap();

    #[cfg(target_arch = "aarch64")]
    std::fs::copy("./vendor/dstorage-1.2.1/bin/ARM64/dstorage.dll", target_dstorage).unwrap();
}

fn main() {
    let target_dstorage = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("../../..")
        .join("dstorage.dll");

    let target_dstoragecore = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("../../..")
        .join("dstoragecore.dll");

    #[cfg(target_arch = "x86")]
    {
        println!(concat!(
            "cargo:rustc-link-search=",
            env!("CARGO_MANIFEST_DIR"),
            "/vendor/dstorage-1.2.1/lib/x86"
        ));
        std::fs::copy(
            "./vendor/dstorage-1.2.1/bin/x86/dstorage.dll",
            target_dstorage,
        )
        .unwrap();
        std::fs::copy(
            "./vendor/dstorage-1.2.1/bin/x86/dstoragecore.dll",
            target_dstoragecore,
        )
        .unwrap();
    }

    #[cfg(target_arch = "x86_64")]
    {
        println!(concat!(
            "cargo:rustc-link-search=",
            env!("CARGO_MANIFEST_DIR"),
            "/vendor/dstorage-1.2.1/lib/x64"
        ));
        std::fs::copy(
            "./vendor/dstorage-1.2.1/bin/x64/dstorage.dll",
            target_dstorage,
        )
        .unwrap();
        std::fs::copy(
            "./vendor/dstorage-1.2.1/bin/x64/dstoragecore.dll",
            target_dstoragecore,
        )
        .unwrap();
    }

    #[cfg(target_arch = "arm")]
    {
        println!(concat!(
            "cargo:rustc-link-search=",
            env!("CARGO_MANIFEST_DIR"),
            "/vendor/dstorage-1.2.1/lib/ARM"
        ));
        std::fs::copy(
            "./vendor/dstorage-1.2.1/bin/ARM/dstorage.dll",
            target_dstorage,
        )
        .unwrap();
        std::fs::copy(
            "./vendor/dstorage-1.2.1/bin/ARM/dstoragecore.dll",
            target_dstoragecore,
        )
        .unwrap();
    }

    #[cfg(target_arch = "aarch64")]
    {
        println!(concat!(
            "cargo:rustc-link-search=",
            env!("CARGO_MANIFEST_DIR"),
            "/vendor/dstorage-1.2.1/lib/ARM64"
        ));
        std::fs::copy(
            "./vendor/dstorage-1.2.1/bin/ARM64/dstorage.dll",
            target_dstorage,
        )
        .unwrap();
        std::fs::copy(
            "./vendor/dstorage-1.2.1/bin/ARM64/dstoragecore.dll",
            target_dstoragecore,
        )
        .unwrap();
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    use std::ffi::OsString;
    use std::fs;

    cc::Build::new()
        .no_default_flags(true)
        // .files(&entries)
        .file(std::path::Path::new("src/x86/x86_64/helper.S"))
        .pic(true)
        .static_flag(true)
        .shared_flag(false)
        .compile("amadant");
}
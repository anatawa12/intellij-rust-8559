include!("macros.rs");

mod other;
include!("other_included.rs");

fn main() {
    println!("Hello, world!");
}

// in this file, a file includes macros.rs, str_enum! is resolved
// so I can go to definition of str_enum!
str_enum! {
    pub(crate) enum Platform {
        pub(crate) type Err = UnknownPlatformErr;
        MacOS("macos"),
        Windows("windows"),
        Linux("linux"),
    }
}

#[test]
fn test_main() {
    // also, because str_enum! is expanded IDE internally, the definition of CpuArch can be found.
    // I can go to definition of Platform or CpuArch::MacOS.
    assert_eq!(Platform::MacOS, "macos".parse().unwrap());
}

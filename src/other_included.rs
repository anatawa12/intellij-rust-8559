// in this file, a file included after macros.rs, str_enum! is resolved
// str_enum! is not resolved so I can't go to definition of str_enum!
str_enum! {
    pub(crate) enum BinaryFormat {
        pub(crate) type Err = UnknownBinaryFormatErr;
        MachO("mach-o"),
        ELF("x86_64"),
        PE("pe"),
    }
}

#[test]
fn test_other_included() {
    // also, because str_enum! is expanded IDE internally,
    //  the definition of BinaryFormat can be found.
    // I can go to definition of BinaryFormat or BinaryFormat::MachO.
    assert_eq!(BinaryFormat::MachO, "mach-o".parse().unwrap());
}

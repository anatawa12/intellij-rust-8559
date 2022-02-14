
// str_enum! is not resolved so I can't go to definition of str_enum!
str_enum! {
    pub(crate) enum CpuArch {
        pub(crate) type Err = UnknownCpuArchErr;
        I386("i386"),
        X86_64("x86_64"),
        ArmV7("armv7"),
        Arm64("arm64"),
    }
}

#[test]
fn test_other() {
    // also, because str_enum! is not expanded IDE internally, 
    //  the definition of CpuArch is not found.
    // I can't go to CpuArch or CpuArch::I386.
    assert_eq!(CpuArch::I386, "i386".parse().unwrap());
}

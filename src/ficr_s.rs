#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 512usize],
    #[doc = "0x200 - Device info"]
    pub info: INFO,
    _reserved1: [u8; 212usize],
    #[doc = "0x300 - Unspecified"]
    pub trimcnf: [TRIMCNF; 256],
    _reserved2: [u8; 256usize],
    #[doc = "0xc00 - NIST800-90B RNG calibration data"]
    pub trng90b: TRNG90B,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct INFO {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Description collection: Device identifier"]
    pub deviceid: [self::info::DEVICEID; 2],
    #[doc = "0x0c - Part code"]
    pub part: self::info::PART,
    #[doc = "0x10 - Part Variant, Hardware version and Production configuration"]
    pub variant: self::info::VARIANT,
    #[doc = "0x14 - Package option"]
    pub package: self::info::PACKAGE,
    #[doc = "0x18 - RAM variant"]
    pub ram: self::info::RAM,
    #[doc = "0x1c - Flash variant"]
    pub flash: self::info::FLASH,
    #[doc = "0x20 - Code memory page size"]
    pub codepagesize: self::info::CODEPAGESIZE,
    #[doc = "0x24 - Code memory size"]
    pub codesize: self::info::CODESIZE,
    #[doc = "0x28 - Device type"]
    pub devicetype: self::info::DEVICETYPE,
}
#[doc = r"Register block"]
#[doc = "Device info"]
pub mod info;
#[doc = r"Register block"]
#[repr(C)]
pub struct TRIMCNF {
    #[doc = "0x00 - Description cluster: Address"]
    pub addr: self::trimcnf::ADDR,
    #[doc = "0x04 - Description cluster: Data"]
    pub data: self::trimcnf::DATA,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod trimcnf;
#[doc = r"Register block"]
#[repr(C)]
pub struct TRNG90B {
    #[doc = "0x00 - Amount of bytes for the required entropy bits"]
    pub bytes: self::trng90b::BYTES,
    #[doc = "0x04 - Repetition counter cutoff"]
    pub rccutoff: self::trng90b::RCCUTOFF,
    #[doc = "0x08 - Adaptive proportion cutoff"]
    pub apcutoff: self::trng90b::APCUTOFF,
    #[doc = "0x0c - Amount of bytes for the startup tests"]
    pub startup: self::trng90b::STARTUP,
    #[doc = "0x10 - Sample count for ring oscillator 1"]
    pub rosc1: self::trng90b::ROSC1,
    #[doc = "0x14 - Sample count for ring oscillator 2"]
    pub rosc2: self::trng90b::ROSC2,
    #[doc = "0x18 - Sample count for ring oscillator 3"]
    pub rosc3: self::trng90b::ROSC3,
    #[doc = "0x1c - Sample count for ring oscillator 4"]
    pub rosc4: self::trng90b::ROSC4,
}
#[doc = r"Register block"]
#[doc = "NIST800-90B RNG calibration data"]
pub mod trng90b;

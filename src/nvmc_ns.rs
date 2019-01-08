#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - Ready flag"]
    pub ready: READY,
    _reserved1: [u8; 4usize],
    #[doc = "0x408 - Ready flag"]
    pub readynext: READYNEXT,
    _reserved2: [u8; 248usize],
    #[doc = "0x504 - Configuration register"]
    pub config: CONFIG,
    _reserved3: [u8; 4usize],
    #[doc = "0x50c - Register for erasing all non-volatile user memory"]
    pub eraseall: ERASEALL,
    _reserved4: [u8; 12usize],
    #[doc = "0x51c - Register for partial erase configuration"]
    pub erasepagepartialcfg: ERASEPAGEPARTIALCFG,
    _reserved5: [u8; 32usize],
    #[doc = "0x540 - I-code cache configuration register"]
    pub icachecnf: ICACHECNF,
    _reserved6: [u8; 4usize],
    #[doc = "0x548 - I-code cache hit counter"]
    pub ihit: IHIT,
    #[doc = "0x54c - I-code cache miss counter"]
    pub imiss: IMISS,
    _reserved7: [u8; 52usize],
    #[doc = "0x584 - Unspecified"]
    pub configns: CONFIGNS,
    #[doc = "0x588 - Non-secure APPROTECT enable register"]
    pub writeuicrns: WRITEUICRNS,
    _reserved8: [u8; 372usize],
    #[doc = "0x700 - Force on all NVM supplies. Also see the internal section in the NVMC chapter."]
    pub forceonnvm: FORCEONNVM,
    _reserved9: [u8; 36usize],
    #[doc = "0x728 - Force off NVM supply. Also see the internal section in the NVMC chapter."]
    pub forceoffnvm: FORCEOFFNVM,
}
#[doc = "Ready flag"]
pub struct READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ready flag"]
pub mod ready;
#[doc = "Ready flag"]
pub struct READYNEXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ready flag"]
pub mod readynext;
#[doc = "Configuration register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Register for erasing all non-volatile user memory"]
pub struct ERASEALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing all non-volatile user memory"]
pub mod eraseall;
#[doc = "Register for partial erase configuration"]
pub struct ERASEPAGEPARTIALCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for partial erase configuration"]
pub mod erasepagepartialcfg;
#[doc = "I-code cache configuration register"]
pub struct ICACHECNF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I-code cache configuration register"]
pub mod icachecnf;
#[doc = "I-code cache hit counter"]
pub struct IHIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I-code cache hit counter"]
pub mod ihit;
#[doc = "I-code cache miss counter"]
pub struct IMISS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I-code cache miss counter"]
pub mod imiss;
#[doc = "Unspecified"]
pub struct CONFIGNS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unspecified"]
pub mod configns;
#[doc = "Non-secure APPROTECT enable register"]
pub struct WRITEUICRNS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-secure APPROTECT enable register"]
pub mod writeuicrns;
#[doc = "Force on all NVM supplies. Also see the internal section in the NVMC chapter."]
pub struct FORCEONNVM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Force on all NVM supplies. Also see the internal section in the NVMC chapter."]
pub mod forceonnvm;
#[doc = "Force off NVM supply. Also see the internal section in the NVMC chapter."]
pub struct FORCEOFFNVM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Force off NVM supply. Also see the internal section in the NVMC chapter."]
pub mod forceoffnvm;

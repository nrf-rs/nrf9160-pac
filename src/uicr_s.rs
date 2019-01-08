use uicr_s::keyslot::CONFIG;
use uicr_s::keyslot::KEY;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access port protection"]
    pub approtect: APPROTECT,
    _reserved0: [u8; 16usize],
    #[doc = "0x14 - Oscillator control"]
    pub xosc32m: XOSC32M,
    _reserved1: [u8; 4usize],
    #[doc = "0x1c - HFXO clock source selection"]
    pub hfxosrc: HFXOSRC,
    #[doc = "0x20 - HFXO startup counter"]
    pub hfxocnt: HFXOCNT,
    _reserved2: [u8; 8usize],
    #[doc = "0x2c - Secure access port protection"]
    pub secureapprotect: SECUREAPPROTECT,
    #[doc = "0x30 - Erase protection"]
    pub eraseprotect: ERASEPROTECT,
    _reserved3: [u8; 212usize],
    #[doc = "0x108 - Description collection: OTP bits \\[31+n*32:0+n*32\\]."]
    pub otp: [OTP; 190],
    #[doc = "0x400 - Unspecified"]
    pub keyslot: KEYSLOT,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct KEYSLOT {
    #[doc = "0x00 - Unspecified"]
    pub config: [CONFIG; 128],
    #[doc = "0x400 - Unspecified"]
    pub key: [KEY; 128],
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod keyslot;
#[doc = "Access port protection"]
pub struct APPROTECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access port protection"]
pub mod approtect;
#[doc = "Oscillator control"]
pub struct XOSC32M {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oscillator control"]
pub mod xosc32m;
#[doc = "HFXO clock source selection"]
pub struct HFXOSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFXO clock source selection"]
pub mod hfxosrc;
#[doc = "HFXO startup counter"]
pub struct HFXOCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFXO startup counter"]
pub mod hfxocnt;
#[doc = "Secure access port protection"]
pub struct SECUREAPPROTECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure access port protection"]
pub mod secureapprotect;
#[doc = "Erase protection"]
pub struct ERASEPROTECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Erase protection"]
pub mod eraseprotect;
#[doc = "Description collection: OTP bits \\[31+n*32:0+n*32\\]."]
pub struct OTP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: OTP bits \\[31+n*32:0+n*32\\]."]
pub mod otp;

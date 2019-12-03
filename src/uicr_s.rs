#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access port protection"]
    pub approtect: APPROTECT,
    _reserved1: [u8; 16usize],
    #[doc = "0x14 - Oscillator control"]
    pub xosc32m: XOSC32M,
    _reserved2: [u8; 4usize],
    #[doc = "0x1c - HFXO clock source selection"]
    pub hfxosrc: HFXOSRC,
    #[doc = "0x20 - HFXO startup counter"]
    pub hfxocnt: HFXOCNT,
    _reserved4: [u8; 8usize],
    #[doc = "0x2c - Secure access port protection"]
    pub secureapprotect: SECUREAPPROTECT,
    #[doc = "0x30 - Erase protection"]
    pub eraseprotect: ERASEPROTECT,
    _reserved6: [u8; 212usize],
    #[doc = "0x108 - Description collection: OTP bits \\[31+n*32:0+n*32\\]."]
    pub otp: [OTP; 190],
    #[doc = "0x400 - Unspecified"]
    pub keyslot: KEYSLOT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct KEYSLOT {
    #[doc = "0x00 - Unspecified"]
    pub config: [keyslot::CONFIG; 128],
    #[doc = "0x400 - Unspecified"]
    pub key: [keyslot::KEY; 128],
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod keyslot;
#[doc = "Access port protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [approtect](approtect) module"]
pub type APPROTECT = crate::Reg<u32, _APPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPROTECT;
#[doc = "`read()` method returns [approtect::R](approtect::R) reader structure"]
impl crate::Readable for APPROTECT {}
#[doc = "`write(|w| ..)` method takes [approtect::W](approtect::W) writer structure"]
impl crate::Writable for APPROTECT {}
#[doc = "Access port protection"]
pub mod approtect;
#[doc = "Oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xosc32m](xosc32m) module"]
pub type XOSC32M = crate::Reg<u32, _XOSC32M>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XOSC32M;
#[doc = "`read()` method returns [xosc32m::R](xosc32m::R) reader structure"]
impl crate::Readable for XOSC32M {}
#[doc = "`write(|w| ..)` method takes [xosc32m::W](xosc32m::W) writer structure"]
impl crate::Writable for XOSC32M {}
#[doc = "Oscillator control"]
pub mod xosc32m;
#[doc = "HFXO clock source selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfxosrc](hfxosrc) module"]
pub type HFXOSRC = crate::Reg<u32, _HFXOSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFXOSRC;
#[doc = "`read()` method returns [hfxosrc::R](hfxosrc::R) reader structure"]
impl crate::Readable for HFXOSRC {}
#[doc = "`write(|w| ..)` method takes [hfxosrc::W](hfxosrc::W) writer structure"]
impl crate::Writable for HFXOSRC {}
#[doc = "HFXO clock source selection"]
pub mod hfxosrc;
#[doc = "HFXO startup counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfxocnt](hfxocnt) module"]
pub type HFXOCNT = crate::Reg<u32, _HFXOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFXOCNT;
#[doc = "`read()` method returns [hfxocnt::R](hfxocnt::R) reader structure"]
impl crate::Readable for HFXOCNT {}
#[doc = "`write(|w| ..)` method takes [hfxocnt::W](hfxocnt::W) writer structure"]
impl crate::Writable for HFXOCNT {}
#[doc = "HFXO startup counter"]
pub mod hfxocnt;
#[doc = "Secure access port protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secureapprotect](secureapprotect) module"]
pub type SECUREAPPROTECT = crate::Reg<u32, _SECUREAPPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECUREAPPROTECT;
#[doc = "`read()` method returns [secureapprotect::R](secureapprotect::R) reader structure"]
impl crate::Readable for SECUREAPPROTECT {}
#[doc = "`write(|w| ..)` method takes [secureapprotect::W](secureapprotect::W) writer structure"]
impl crate::Writable for SECUREAPPROTECT {}
#[doc = "Secure access port protection"]
pub mod secureapprotect;
#[doc = "Erase protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eraseprotect](eraseprotect) module"]
pub type ERASEPROTECT = crate::Reg<u32, _ERASEPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEPROTECT;
#[doc = "`read()` method returns [eraseprotect::R](eraseprotect::R) reader structure"]
impl crate::Readable for ERASEPROTECT {}
#[doc = "`write(|w| ..)` method takes [eraseprotect::W](eraseprotect::W) writer structure"]
impl crate::Writable for ERASEPROTECT {}
#[doc = "Erase protection"]
pub mod eraseprotect;
#[doc = "Description collection: OTP bits \\[31+n*32:0+n*32\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [otp](otp) module"]
pub type OTP = crate::Reg<u32, _OTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTP;
#[doc = "`read()` method returns [otp::R](otp::R) reader structure"]
impl crate::Readable for OTP {}
#[doc = "`write(|w| ..)` method takes [otp::W](otp::W) writer structure"]
impl crate::Writable for OTP {}
#[doc = "Description collection: OTP bits \\[31+n*32:0+n*32\\]."]
pub mod otp;

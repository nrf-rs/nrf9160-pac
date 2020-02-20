#[doc = r"Register block"]
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
    _reserved8: [u8; 52usize],
    #[doc = "0x584 - Unspecified"]
    pub configns: CONFIGNS,
    #[doc = "0x588 - Non-secure APPROTECT enable register"]
    pub writeuicrns: WRITEUICRNS,
}
#[doc = "Ready flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ready](ready) module"]
pub type READY = crate::Reg<u32, _READY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READY;
#[doc = "`read()` method returns [ready::R](ready::R) reader structure"]
impl crate::Readable for READY {}
#[doc = "Ready flag"]
pub mod ready;
#[doc = "Ready flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readynext](readynext) module"]
pub type READYNEXT = crate::Reg<u32, _READYNEXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READYNEXT;
#[doc = "`read()` method returns [readynext::R](readynext::R) reader structure"]
impl crate::Readable for READYNEXT {}
#[doc = "Ready flag"]
pub mod readynext;
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Register for erasing all non-volatile user memory\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eraseall](eraseall) module"]
pub type ERASEALL = crate::Reg<u32, _ERASEALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEALL;
#[doc = "`write(|w| ..)` method takes [eraseall::W](eraseall::W) writer structure"]
impl crate::Writable for ERASEALL {}
#[doc = "Register for erasing all non-volatile user memory"]
pub mod eraseall;
#[doc = "Register for partial erase configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erasepagepartialcfg](erasepagepartialcfg) module"]
pub type ERASEPAGEPARTIALCFG = crate::Reg<u32, _ERASEPAGEPARTIALCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEPAGEPARTIALCFG;
#[doc = "`read()` method returns [erasepagepartialcfg::R](erasepagepartialcfg::R) reader structure"]
impl crate::Readable for ERASEPAGEPARTIALCFG {}
#[doc = "`write(|w| ..)` method takes [erasepagepartialcfg::W](erasepagepartialcfg::W) writer structure"]
impl crate::Writable for ERASEPAGEPARTIALCFG {}
#[doc = "Register for partial erase configuration"]
pub mod erasepagepartialcfg;
#[doc = "I-code cache configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icachecnf](icachecnf) module"]
pub type ICACHECNF = crate::Reg<u32, _ICACHECNF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICACHECNF;
#[doc = "`read()` method returns [icachecnf::R](icachecnf::R) reader structure"]
impl crate::Readable for ICACHECNF {}
#[doc = "`write(|w| ..)` method takes [icachecnf::W](icachecnf::W) writer structure"]
impl crate::Writable for ICACHECNF {}
#[doc = "I-code cache configuration register"]
pub mod icachecnf;
#[doc = "I-code cache hit counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ihit](ihit) module"]
pub type IHIT = crate::Reg<u32, _IHIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IHIT;
#[doc = "`read()` method returns [ihit::R](ihit::R) reader structure"]
impl crate::Readable for IHIT {}
#[doc = "`write(|w| ..)` method takes [ihit::W](ihit::W) writer structure"]
impl crate::Writable for IHIT {}
#[doc = "I-code cache hit counter"]
pub mod ihit;
#[doc = "I-code cache miss counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imiss](imiss) module"]
pub type IMISS = crate::Reg<u32, _IMISS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMISS;
#[doc = "`read()` method returns [imiss::R](imiss::R) reader structure"]
impl crate::Readable for IMISS {}
#[doc = "`write(|w| ..)` method takes [imiss::W](imiss::W) writer structure"]
impl crate::Writable for IMISS {}
#[doc = "I-code cache miss counter"]
pub mod imiss;
#[doc = "Unspecified\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [configns](configns) module"]
pub type CONFIGNS = crate::Reg<u32, _CONFIGNS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIGNS;
#[doc = "`read()` method returns [configns::R](configns::R) reader structure"]
impl crate::Readable for CONFIGNS {}
#[doc = "`write(|w| ..)` method takes [configns::W](configns::W) writer structure"]
impl crate::Writable for CONFIGNS {}
#[doc = "Unspecified"]
pub mod configns;
#[doc = "Non-secure APPROTECT enable register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writeuicrns](writeuicrns) module"]
pub type WRITEUICRNS = crate::Reg<u32, _WRITEUICRNS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRITEUICRNS;
#[doc = "`write(|w| ..)` method takes [writeuicrns::W](writeuicrns::W) writer structure"]
impl crate::Writable for WRITEUICRNS {}
#[doc = "Non-secure APPROTECT enable register"]
pub mod writeuicrns;

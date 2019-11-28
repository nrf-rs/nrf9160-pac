#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1280usize],
    #[doc = "0x500 - Enable debug domain and aquire selected GPIOs"]
    pub enable: ENABLE,
    #[doc = "0x504 - Unspecified"]
    pub psel: PSEL,
    #[doc = "0x518 - Clocking options for the Trace Port debug interface"]
    pub traceportspeed: TRACEPORTSPEED,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin number configuration for TRACECLK"]
    pub traceclk: self::psel::TRACECLK,
    #[doc = "0x04 - Pin number configuration for TRACEDATA\\[0\\]"]
    pub tracedata0: self::psel::TRACEDATA0,
    #[doc = "0x08 - Pin number configuration for TRACEDATA\\[1\\]"]
    pub tracedata1: self::psel::TRACEDATA1,
    #[doc = "0x0c - Pin number configuration for TRACEDATA\\[2\\]"]
    pub tracedata2: self::psel::TRACEDATA2,
    #[doc = "0x10 - Pin number configuration for TRACEDATA\\[3\\]"]
    pub tracedata3: self::psel::TRACEDATA3,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Enable debug domain and aquire selected GPIOs\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable debug domain and aquire selected GPIOs"]
pub mod enable;
#[doc = "Clocking options for the Trace Port debug interface\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [traceportspeed](traceportspeed) module"]
pub type TRACEPORTSPEED = crate::Reg<u32, _TRACEPORTSPEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRACEPORTSPEED;
#[doc = "`read()` method returns [traceportspeed::R](traceportspeed::R) reader structure"]
impl crate::Readable for TRACEPORTSPEED {}
#[doc = "`write(|w| ..)` method takes [traceportspeed::W](traceportspeed::W) writer structure"]
impl crate::Writable for TRACEPORTSPEED {}
#[doc = "Clocking options for the Trace Port debug interface"]
pub mod traceportspeed;

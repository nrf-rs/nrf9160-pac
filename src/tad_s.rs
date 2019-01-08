#[doc = r" Register block"]
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
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin number configuration for TRACECLK"]
    pub traceclk: self::psel::TRACECLK,
    #[doc = "0x04 - Pin number configuration for TRACEDATA[0]"]
    pub tracedata0: self::psel::TRACEDATA0,
    #[doc = "0x08 - Pin number configuration for TRACEDATA[1]"]
    pub tracedata1: self::psel::TRACEDATA1,
    #[doc = "0x0c - Pin number configuration for TRACEDATA[2]"]
    pub tracedata2: self::psel::TRACEDATA2,
    #[doc = "0x10 - Pin number configuration for TRACEDATA[3]"]
    pub tracedata3: self::psel::TRACEDATA3,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Enable debug domain and aquire selected GPIOs"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable debug domain and aquire selected GPIOs"]
pub mod enable;
#[doc = "Clocking options for the Trace Port debug interface"]
pub struct TRACEPORTSPEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clocking options for the Trace Port debug interface"]
pub mod traceportspeed;

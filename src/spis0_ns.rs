#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 36usize],
    #[doc = "0x24 - Acquire SPI semaphore"]
    pub tasks_acquire: TASKS_ACQUIRE,
    #[doc = "0x28 - Release SPI semaphore, enabling the SPI slave to acquire it"]
    pub tasks_release: TASKS_RELEASE,
    _reserved1: [u8; 120usize],
    #[doc = "0xa4 - Subscribe configuration for task ACQUIRE"]
    pub subscribe_acquire: SUBSCRIBE_ACQUIRE,
    #[doc = "0xa8 - Subscribe configuration for task RELEASE"]
    pub subscribe_release: SUBSCRIBE_RELEASE,
    _reserved2: [u8; 88usize],
    #[doc = "0x104 - Granted transaction completed"]
    pub events_end: EVENTS_END,
    _reserved3: [u8; 8usize],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved4: [u8; 20usize],
    #[doc = "0x128 - Semaphore acquired"]
    pub events_acquired: EVENTS_ACQUIRED,
    _reserved5: [u8; 88usize],
    #[doc = "0x184 - Publish configuration for event END"]
    pub publish_end: PUBLISH_END,
    _reserved6: [u8; 8usize],
    #[doc = "0x190 - Publish configuration for event ENDRX"]
    pub publish_endrx: PUBLISH_ENDRX,
    _reserved7: [u8; 20usize],
    #[doc = "0x1a8 - Publish configuration for event ACQUIRED"]
    pub publish_acquired: PUBLISH_ACQUIRED,
    _reserved8: [u8; 84usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved9: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 244usize],
    #[doc = "0x400 - Semaphore status register"]
    pub semstat: SEMSTAT,
    _reserved11: [u8; 60usize],
    #[doc = "0x440 - Status from last transaction"]
    pub status: STATUS,
    _reserved12: [u8; 188usize],
    #[doc = "0x500 - Enable SPI slave"]
    pub enable: ENABLE,
    _reserved13: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved14: [u8; 28usize],
    #[doc = "0x534 - Unspecified"]
    pub rxd: RXD,
    _reserved15: [u8; 4usize],
    #[doc = "0x544 - Unspecified"]
    pub txd: TXD,
    _reserved16: [u8; 4usize],
    #[doc = "0x554 - Configuration register"]
    pub config: CONFIG,
    _reserved17: [u8; 4usize],
    #[doc = "0x55c - Default character. Character clocked out in case of an ignored transaction."]
    pub def: DEF,
    _reserved18: [u8; 96usize],
    #[doc = "0x5c0 - Over-read character"]
    pub orc: ORC,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCK"]
    pub sck: self::psel::SCK,
    #[doc = "0x04 - Pin select for MISO signal"]
    pub miso: self::psel::MISO,
    #[doc = "0x08 - Pin select for MOSI signal"]
    pub mosi: self::psel::MOSI,
    #[doc = "0x0c - Pin select for CSN signal"]
    pub csn: self::psel::CSN,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r" Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - RXD data pointer"]
    pub ptr: self::rxd::PTR,
    #[doc = "0x04 - Maximum number of bytes in receive buffer"]
    pub maxcnt: self::rxd::MAXCNT,
    #[doc = "0x08 - Number of bytes received in last granted transaction"]
    pub amount: self::rxd::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = r" Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - TXD data pointer"]
    pub ptr: self::txd::PTR,
    #[doc = "0x04 - Maximum number of bytes in transmit buffer"]
    pub maxcnt: self::txd::MAXCNT,
    #[doc = "0x08 - Number of bytes transmitted in last granted transaction"]
    pub amount: self::txd::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = "Acquire SPI semaphore"]
pub struct TASKS_ACQUIRE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Acquire SPI semaphore"]
pub mod tasks_acquire;
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
pub struct TASKS_RELEASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
pub mod tasks_release;
#[doc = "Subscribe configuration for task ACQUIRE"]
pub struct SUBSCRIBE_ACQUIRE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task ACQUIRE"]
pub mod subscribe_acquire;
#[doc = "Subscribe configuration for task RELEASE"]
pub struct SUBSCRIBE_RELEASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task RELEASE"]
pub mod subscribe_release;
#[doc = "Granted transaction completed"]
pub struct EVENTS_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Granted transaction completed"]
pub mod events_end;
#[doc = "End of RXD buffer reached"]
pub struct EVENTS_ENDRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "Semaphore acquired"]
pub struct EVENTS_ACQUIRED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore acquired"]
pub mod events_acquired;
#[doc = "Publish configuration for event END"]
pub struct PUBLISH_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "Publish configuration for event ENDRX"]
pub struct PUBLISH_ENDRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event ENDRX"]
pub mod publish_endrx;
#[doc = "Publish configuration for event ACQUIRED"]
pub struct PUBLISH_ACQUIRED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event ACQUIRED"]
pub mod publish_acquired;
#[doc = "Shortcuts between local events and tasks"]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "Enable interrupt"]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Semaphore status register"]
pub struct SEMSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Semaphore status register"]
pub mod semstat;
#[doc = "Status from last transaction"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status from last transaction"]
pub mod status;
#[doc = "Enable SPI slave"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable SPI slave"]
pub mod enable;
#[doc = "Configuration register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Default character. Character clocked out in case of an ignored transaction."]
pub struct DEF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Default character. Character clocked out in case of an ignored transaction."]
pub mod def;
#[doc = "Over-read character"]
pub struct ORC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Over-read character"]
pub mod orc;

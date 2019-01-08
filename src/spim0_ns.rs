#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Start SPI transaction"]
    pub tasks_start: TASKS_START,
    #[doc = "0x14 - Stop SPI transaction"]
    pub tasks_stop: TASKS_STOP,
    _reserved1: [u8; 4usize],
    #[doc = "0x1c - Suspend SPI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume SPI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved2: [u8; 108usize],
    #[doc = "0x90 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x94 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    _reserved3: [u8; 4usize],
    #[doc = "0x9c - Subscribe configuration for task SUSPEND"]
    pub subscribe_suspend: SUBSCRIBE_SUSPEND,
    #[doc = "0xa0 - Subscribe configuration for task RESUME"]
    pub subscribe_resume: SUBSCRIBE_RESUME,
    _reserved4: [u8; 96usize],
    #[doc = "0x104 - SPI transaction has stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved5: [u8; 8usize],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved6: [u8; 4usize],
    #[doc = "0x118 - End of RXD buffer and TXD buffer reached"]
    pub events_end: EVENTS_END,
    _reserved7: [u8; 4usize],
    #[doc = "0x120 - End of TXD buffer reached"]
    pub events_endtx: EVENTS_ENDTX,
    _reserved8: [u8; 40usize],
    #[doc = "0x14c - Transaction started"]
    pub events_started: EVENTS_STARTED,
    _reserved9: [u8; 52usize],
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    pub publish_stopped: PUBLISH_STOPPED,
    _reserved10: [u8; 8usize],
    #[doc = "0x190 - Publish configuration for event ENDRX"]
    pub publish_endrx: PUBLISH_ENDRX,
    _reserved11: [u8; 4usize],
    #[doc = "0x198 - Publish configuration for event END"]
    pub publish_end: PUBLISH_END,
    _reserved12: [u8; 4usize],
    #[doc = "0x1a0 - Publish configuration for event ENDTX"]
    pub publish_endtx: PUBLISH_ENDTX,
    _reserved13: [u8; 40usize],
    #[doc = "0x1cc - Publish configuration for event STARTED"]
    pub publish_started: PUBLISH_STARTED,
    _reserved14: [u8; 48usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved15: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved16: [u8; 500usize],
    #[doc = "0x500 - Enable SPIM"]
    pub enable: ENABLE,
    _reserved17: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved18: [u8; 16usize],
    #[doc = "0x524 - SPI frequency. Accuracy depends on the HFCLK source selected."]
    pub frequency: FREQUENCY,
    _reserved19: [u8; 12usize],
    #[doc = "0x534 - RXD EasyDMA channel"]
    pub rxd: RXD,
    #[doc = "0x544 - TXD EasyDMA channel"]
    pub txd: TXD,
    #[doc = "0x554 - Configuration register"]
    pub config: CONFIG,
    _reserved20: [u8; 104usize],
    #[doc = "0x5c0 - Over-read character. Character clocked out in case and over-read of the TXD buffer."]
    pub orc: ORC,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCK"]
    pub sck: self::psel::SCK,
    #[doc = "0x04 - Pin select for MOSI signal"]
    pub mosi: self::psel::MOSI,
    #[doc = "0x08 - Pin select for MISO signal"]
    pub miso: self::psel::MISO,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r" Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::rxd::PTR,
    #[doc = "0x04 - Maximum number of bytes in receive buffer"]
    pub maxcnt: self::rxd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::rxd::AMOUNT,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: self::rxd::LIST,
}
#[doc = r" Register block"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = r" Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::txd::PTR,
    #[doc = "0x04 - Maximum number of bytes in transmit buffer"]
    pub maxcnt: self::txd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::txd::AMOUNT,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: self::txd::LIST,
}
#[doc = r" Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = "Start SPI transaction"]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start SPI transaction"]
pub mod tasks_start;
#[doc = "Stop SPI transaction"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop SPI transaction"]
pub mod tasks_stop;
#[doc = "Suspend SPI transaction"]
pub struct TASKS_SUSPEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend SPI transaction"]
pub mod tasks_suspend;
#[doc = "Resume SPI transaction"]
pub struct TASKS_RESUME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Resume SPI transaction"]
pub mod tasks_resume;
#[doc = "Subscribe configuration for task START"]
pub struct SUBSCRIBE_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "Subscribe configuration for task STOP"]
pub struct SUBSCRIBE_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "Subscribe configuration for task SUSPEND"]
pub struct SUBSCRIBE_SUSPEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task SUSPEND"]
pub mod subscribe_suspend;
#[doc = "Subscribe configuration for task RESUME"]
pub struct SUBSCRIBE_RESUME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task RESUME"]
pub mod subscribe_resume;
#[doc = "SPI transaction has stopped"]
pub struct EVENTS_STOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI transaction has stopped"]
pub mod events_stopped;
#[doc = "End of RXD buffer reached"]
pub struct EVENTS_ENDRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "End of RXD buffer and TXD buffer reached"]
pub struct EVENTS_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End of RXD buffer and TXD buffer reached"]
pub mod events_end;
#[doc = "End of TXD buffer reached"]
pub struct EVENTS_ENDTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End of TXD buffer reached"]
pub mod events_endtx;
#[doc = "Transaction started"]
pub struct EVENTS_STARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transaction started"]
pub mod events_started;
#[doc = "Publish configuration for event STOPPED"]
pub struct PUBLISH_STOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "Publish configuration for event ENDRX"]
pub struct PUBLISH_ENDRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event ENDRX"]
pub mod publish_endrx;
#[doc = "Publish configuration for event END"]
pub struct PUBLISH_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "Publish configuration for event ENDTX"]
pub struct PUBLISH_ENDTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event ENDTX"]
pub mod publish_endtx;
#[doc = "Publish configuration for event STARTED"]
pub struct PUBLISH_STARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event STARTED"]
pub mod publish_started;
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
#[doc = "Enable SPIM"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable SPIM"]
pub mod enable;
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
pub struct FREQUENCY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "Configuration register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
pub struct ORC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer."]
pub mod orc;

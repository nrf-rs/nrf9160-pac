#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start UART receiver"]
    pub tasks_startrx: TASKS_STARTRX,
    #[doc = "0x04 - Stop UART receiver"]
    pub tasks_stoprx: TASKS_STOPRX,
    #[doc = "0x08 - Start UART transmitter"]
    pub tasks_starttx: TASKS_STARTTX,
    #[doc = "0x0c - Stop UART transmitter"]
    pub tasks_stoptx: TASKS_STOPTX,
    _reserved0: [u8; 28usize],
    #[doc = "0x2c - Flush RX FIFO into RX buffer"]
    pub tasks_flushrx: TASKS_FLUSHRX,
    _reserved1: [u8; 80usize],
    #[doc = "0x80 - Subscribe configuration for task STARTRX"]
    pub subscribe_startrx: SUBSCRIBE_STARTRX,
    #[doc = "0x84 - Subscribe configuration for task STOPRX"]
    pub subscribe_stoprx: SUBSCRIBE_STOPRX,
    #[doc = "0x88 - Subscribe configuration for task STARTTX"]
    pub subscribe_starttx: SUBSCRIBE_STARTTX,
    #[doc = "0x8c - Subscribe configuration for task STOPTX"]
    pub subscribe_stoptx: SUBSCRIBE_STOPTX,
    _reserved2: [u8; 28usize],
    #[doc = "0xac - Subscribe configuration for task FLUSHRX"]
    pub subscribe_flushrx: SUBSCRIBE_FLUSHRX,
    _reserved3: [u8; 80usize],
    #[doc = "0x100 - CTS is activated (set low). Clear To Send."]
    pub events_cts: EVENTS_CTS,
    #[doc = "0x104 - CTS is deactivated (set high). Not Clear To Send."]
    pub events_ncts: EVENTS_NCTS,
    #[doc = "0x108 - Data received in RXD (but potentially not yet transferred to Data RAM)"]
    pub events_rxdrdy: EVENTS_RXDRDY,
    _reserved4: [u8; 4usize],
    #[doc = "0x110 - Receive buffer is filled up"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved5: [u8; 8usize],
    #[doc = "0x11c - Data sent from TXD"]
    pub events_txdrdy: EVENTS_TXDRDY,
    #[doc = "0x120 - Last TX byte transmitted"]
    pub events_endtx: EVENTS_ENDTX,
    #[doc = "0x124 - Error detected"]
    pub events_error: EVENTS_ERROR,
    _reserved6: [u8; 28usize],
    #[doc = "0x144 - Receiver timeout"]
    pub events_rxto: EVENTS_RXTO,
    _reserved7: [u8; 4usize],
    #[doc = "0x14c - UART receiver has started"]
    pub events_rxstarted: EVENTS_RXSTARTED,
    #[doc = "0x150 - UART transmitter has started"]
    pub events_txstarted: EVENTS_TXSTARTED,
    _reserved8: [u8; 4usize],
    #[doc = "0x158 - Transmitter stopped"]
    pub events_txstopped: EVENTS_TXSTOPPED,
    _reserved9: [u8; 36usize],
    #[doc = "0x180 - Publish configuration for event CTS"]
    pub publish_cts: PUBLISH_CTS,
    #[doc = "0x184 - Publish configuration for event NCTS"]
    pub publish_ncts: PUBLISH_NCTS,
    #[doc = "0x188 - Publish configuration for event RXDRDY"]
    pub publish_rxdrdy: PUBLISH_RXDRDY,
    _reserved10: [u8; 4usize],
    #[doc = "0x190 - Publish configuration for event ENDRX"]
    pub publish_endrx: PUBLISH_ENDRX,
    _reserved11: [u8; 8usize],
    #[doc = "0x19c - Publish configuration for event TXDRDY"]
    pub publish_txdrdy: PUBLISH_TXDRDY,
    #[doc = "0x1a0 - Publish configuration for event ENDTX"]
    pub publish_endtx: PUBLISH_ENDTX,
    #[doc = "0x1a4 - Publish configuration for event ERROR"]
    pub publish_error: PUBLISH_ERROR,
    _reserved12: [u8; 28usize],
    #[doc = "0x1c4 - Publish configuration for event RXTO"]
    pub publish_rxto: PUBLISH_RXTO,
    _reserved13: [u8; 4usize],
    #[doc = "0x1cc - Publish configuration for event RXSTARTED"]
    pub publish_rxstarted: PUBLISH_RXSTARTED,
    #[doc = "0x1d0 - Publish configuration for event TXSTARTED"]
    pub publish_txstarted: PUBLISH_TXSTARTED,
    _reserved14: [u8; 4usize],
    #[doc = "0x1d8 - Publish configuration for event TXSTOPPED"]
    pub publish_txstopped: PUBLISH_TXSTOPPED,
    _reserved15: [u8; 36usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved16: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved17: [u8; 372usize],
    #[doc = "0x480 - Error source Note : this register is read / write one to clear."]
    pub errorsrc: ERRORSRC,
    _reserved18: [u8; 124usize],
    #[doc = "0x500 - Enable UART"]
    pub enable: ENABLE,
    _reserved19: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved20: [u8; 12usize],
    #[doc = "0x524 - Baud rate. Accuracy depends on the HFCLK source selected."]
    pub baudrate: BAUDRATE,
    _reserved21: [u8; 12usize],
    #[doc = "0x534 - RXD EasyDMA channel"]
    pub rxd: RXD,
    _reserved22: [u8; 4usize],
    #[doc = "0x544 - TXD EasyDMA channel"]
    pub txd: TXD,
    _reserved23: [u8; 28usize],
    #[doc = "0x56c - Configuration of parity and hardware flow control"]
    pub config: CONFIG,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for RTS signal"]
    pub rts: self::psel::RTS,
    #[doc = "0x04 - Pin select for TXD signal"]
    pub txd: self::psel::TXD,
    #[doc = "0x08 - Pin select for CTS signal"]
    pub cts: self::psel::CTS,
    #[doc = "0x0c - Pin select for RXD signal"]
    pub rxd: self::psel::RXD,
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
}
#[doc = r" Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = "Start UART receiver"]
pub struct TASKS_STARTRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start UART receiver"]
pub mod tasks_startrx;
#[doc = "Stop UART receiver"]
pub struct TASKS_STOPRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop UART receiver"]
pub mod tasks_stoprx;
#[doc = "Start UART transmitter"]
pub struct TASKS_STARTTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start UART transmitter"]
pub mod tasks_starttx;
#[doc = "Stop UART transmitter"]
pub struct TASKS_STOPTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop UART transmitter"]
pub mod tasks_stoptx;
#[doc = "Flush RX FIFO into RX buffer"]
pub struct TASKS_FLUSHRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flush RX FIFO into RX buffer"]
pub mod tasks_flushrx;
#[doc = "Subscribe configuration for task STARTRX"]
pub struct SUBSCRIBE_STARTRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task STARTRX"]
pub mod subscribe_startrx;
#[doc = "Subscribe configuration for task STOPRX"]
pub struct SUBSCRIBE_STOPRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task STOPRX"]
pub mod subscribe_stoprx;
#[doc = "Subscribe configuration for task STARTTX"]
pub struct SUBSCRIBE_STARTTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task STARTTX"]
pub mod subscribe_starttx;
#[doc = "Subscribe configuration for task STOPTX"]
pub struct SUBSCRIBE_STOPTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task STOPTX"]
pub mod subscribe_stoptx;
#[doc = "Subscribe configuration for task FLUSHRX"]
pub struct SUBSCRIBE_FLUSHRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task FLUSHRX"]
pub mod subscribe_flushrx;
#[doc = "CTS is activated (set low). Clear To Send."]
pub struct EVENTS_CTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTS is activated (set low). Clear To Send."]
pub mod events_cts;
#[doc = "CTS is deactivated (set high). Not Clear To Send."]
pub struct EVENTS_NCTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTS is deactivated (set high). Not Clear To Send."]
pub mod events_ncts;
#[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)"]
pub struct EVENTS_RXDRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)"]
pub mod events_rxdrdy;
#[doc = "Receive buffer is filled up"]
pub struct EVENTS_ENDRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive buffer is filled up"]
pub mod events_endrx;
#[doc = "Data sent from TXD"]
pub struct EVENTS_TXDRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data sent from TXD"]
pub mod events_txdrdy;
#[doc = "Last TX byte transmitted"]
pub struct EVENTS_ENDTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last TX byte transmitted"]
pub mod events_endtx;
#[doc = "Error detected"]
pub struct EVENTS_ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error detected"]
pub mod events_error;
#[doc = "Receiver timeout"]
pub struct EVENTS_RXTO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver timeout"]
pub mod events_rxto;
#[doc = "UART receiver has started"]
pub struct EVENTS_RXSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART receiver has started"]
pub mod events_rxstarted;
#[doc = "UART transmitter has started"]
pub struct EVENTS_TXSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART transmitter has started"]
pub mod events_txstarted;
#[doc = "Transmitter stopped"]
pub struct EVENTS_TXSTOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter stopped"]
pub mod events_txstopped;
#[doc = "Publish configuration for event CTS"]
pub struct PUBLISH_CTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event CTS"]
pub mod publish_cts;
#[doc = "Publish configuration for event NCTS"]
pub struct PUBLISH_NCTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event NCTS"]
pub mod publish_ncts;
#[doc = "Publish configuration for event RXDRDY"]
pub struct PUBLISH_RXDRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event RXDRDY"]
pub mod publish_rxdrdy;
#[doc = "Publish configuration for event ENDRX"]
pub struct PUBLISH_ENDRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event ENDRX"]
pub mod publish_endrx;
#[doc = "Publish configuration for event TXDRDY"]
pub struct PUBLISH_TXDRDY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event TXDRDY"]
pub mod publish_txdrdy;
#[doc = "Publish configuration for event ENDTX"]
pub struct PUBLISH_ENDTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event ENDTX"]
pub mod publish_endtx;
#[doc = "Publish configuration for event ERROR"]
pub struct PUBLISH_ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event ERROR"]
pub mod publish_error;
#[doc = "Publish configuration for event RXTO"]
pub struct PUBLISH_RXTO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event RXTO"]
pub mod publish_rxto;
#[doc = "Publish configuration for event RXSTARTED"]
pub struct PUBLISH_RXSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event RXSTARTED"]
pub mod publish_rxstarted;
#[doc = "Publish configuration for event TXSTARTED"]
pub struct PUBLISH_TXSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event TXSTARTED"]
pub mod publish_txstarted;
#[doc = "Publish configuration for event TXSTOPPED"]
pub struct PUBLISH_TXSTOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event TXSTOPPED"]
pub mod publish_txstopped;
#[doc = "Shortcuts between local events and tasks"]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "Enable or disable interrupt"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable or disable interrupt"]
pub mod inten;
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
#[doc = "Error source Note : this register is read / write one to clear."]
pub struct ERRORSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error source Note : this register is read / write one to clear."]
pub mod errorsrc;
#[doc = "Enable UART"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable UART"]
pub mod enable;
#[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
pub struct BAUDRATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
pub mod baudrate;
#[doc = "Configuration of parity and hardware flow control"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of parity and hardware flow control"]
pub mod config;

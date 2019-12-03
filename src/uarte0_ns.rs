#[doc = r"Register block"]
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
    _reserved4: [u8; 28usize],
    #[doc = "0x2c - Flush RX FIFO into RX buffer"]
    pub tasks_flushrx: TASKS_FLUSHRX,
    _reserved5: [u8; 80usize],
    #[doc = "0x80 - Subscribe configuration for task STARTRX"]
    pub subscribe_startrx: SUBSCRIBE_STARTRX,
    #[doc = "0x84 - Subscribe configuration for task STOPRX"]
    pub subscribe_stoprx: SUBSCRIBE_STOPRX,
    #[doc = "0x88 - Subscribe configuration for task STARTTX"]
    pub subscribe_starttx: SUBSCRIBE_STARTTX,
    #[doc = "0x8c - Subscribe configuration for task STOPTX"]
    pub subscribe_stoptx: SUBSCRIBE_STOPTX,
    _reserved9: [u8; 28usize],
    #[doc = "0xac - Subscribe configuration for task FLUSHRX"]
    pub subscribe_flushrx: SUBSCRIBE_FLUSHRX,
    _reserved10: [u8; 80usize],
    #[doc = "0x100 - CTS is activated (set low). Clear To Send."]
    pub events_cts: EVENTS_CTS,
    #[doc = "0x104 - CTS is deactivated (set high). Not Clear To Send."]
    pub events_ncts: EVENTS_NCTS,
    #[doc = "0x108 - Data received in RXD (but potentially not yet transferred to Data RAM)"]
    pub events_rxdrdy: EVENTS_RXDRDY,
    _reserved13: [u8; 4usize],
    #[doc = "0x110 - Receive buffer is filled up"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved14: [u8; 8usize],
    #[doc = "0x11c - Data sent from TXD"]
    pub events_txdrdy: EVENTS_TXDRDY,
    #[doc = "0x120 - Last TX byte transmitted"]
    pub events_endtx: EVENTS_ENDTX,
    #[doc = "0x124 - Error detected"]
    pub events_error: EVENTS_ERROR,
    _reserved17: [u8; 28usize],
    #[doc = "0x144 - Receiver timeout"]
    pub events_rxto: EVENTS_RXTO,
    _reserved18: [u8; 4usize],
    #[doc = "0x14c - UART receiver has started"]
    pub events_rxstarted: EVENTS_RXSTARTED,
    #[doc = "0x150 - UART transmitter has started"]
    pub events_txstarted: EVENTS_TXSTARTED,
    _reserved20: [u8; 4usize],
    #[doc = "0x158 - Transmitter stopped"]
    pub events_txstopped: EVENTS_TXSTOPPED,
    _reserved21: [u8; 36usize],
    #[doc = "0x180 - Publish configuration for event CTS"]
    pub publish_cts: PUBLISH_CTS,
    #[doc = "0x184 - Publish configuration for event NCTS"]
    pub publish_ncts: PUBLISH_NCTS,
    #[doc = "0x188 - Publish configuration for event RXDRDY"]
    pub publish_rxdrdy: PUBLISH_RXDRDY,
    _reserved24: [u8; 4usize],
    #[doc = "0x190 - Publish configuration for event ENDRX"]
    pub publish_endrx: PUBLISH_ENDRX,
    _reserved25: [u8; 8usize],
    #[doc = "0x19c - Publish configuration for event TXDRDY"]
    pub publish_txdrdy: PUBLISH_TXDRDY,
    #[doc = "0x1a0 - Publish configuration for event ENDTX"]
    pub publish_endtx: PUBLISH_ENDTX,
    #[doc = "0x1a4 - Publish configuration for event ERROR"]
    pub publish_error: PUBLISH_ERROR,
    _reserved28: [u8; 28usize],
    #[doc = "0x1c4 - Publish configuration for event RXTO"]
    pub publish_rxto: PUBLISH_RXTO,
    _reserved29: [u8; 4usize],
    #[doc = "0x1cc - Publish configuration for event RXSTARTED"]
    pub publish_rxstarted: PUBLISH_RXSTARTED,
    #[doc = "0x1d0 - Publish configuration for event TXSTARTED"]
    pub publish_txstarted: PUBLISH_TXSTARTED,
    _reserved31: [u8; 4usize],
    #[doc = "0x1d8 - Publish configuration for event TXSTOPPED"]
    pub publish_txstopped: PUBLISH_TXSTOPPED,
    _reserved32: [u8; 36usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved33: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved36: [u8; 372usize],
    #[doc = "0x480 - Error source Note : this register is read / write one to clear."]
    pub errorsrc: ERRORSRC,
    _reserved37: [u8; 124usize],
    #[doc = "0x500 - Enable UART"]
    pub enable: ENABLE,
    _reserved38: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved39: [u8; 12usize],
    #[doc = "0x524 - Baud rate. Accuracy depends on the HFCLK source selected."]
    pub baudrate: BAUDRATE,
    _reserved40: [u8; 12usize],
    #[doc = "0x534 - RXD EasyDMA channel"]
    pub rxd: RXD,
    _reserved41: [u8; 4usize],
    #[doc = "0x544 - TXD EasyDMA channel"]
    pub txd: TXD,
    _reserved42: [u8; 28usize],
    #[doc = "0x56c - Configuration of parity and hardware flow control"]
    pub config: CONFIG,
}
#[doc = r"Register block"]
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
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::rxd::PTR,
    #[doc = "0x04 - Maximum number of bytes in receive buffer"]
    pub maxcnt: self::rxd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::rxd::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::txd::PTR,
    #[doc = "0x04 - Maximum number of bytes in transmit buffer"]
    pub maxcnt: self::txd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::txd::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = "Start UART receiver\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_startrx](tasks_startrx) module"]
pub type TASKS_STARTRX = crate::Reg<u32, _TASKS_STARTRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTRX;
#[doc = "`write(|w| ..)` method takes [tasks_startrx::W](tasks_startrx::W) writer structure"]
impl crate::Writable for TASKS_STARTRX {}
#[doc = "Start UART receiver"]
pub mod tasks_startrx;
#[doc = "Stop UART receiver\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_stoprx](tasks_stoprx) module"]
pub type TASKS_STOPRX = crate::Reg<u32, _TASKS_STOPRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOPRX;
#[doc = "`write(|w| ..)` method takes [tasks_stoprx::W](tasks_stoprx::W) writer structure"]
impl crate::Writable for TASKS_STOPRX {}
#[doc = "Stop UART receiver"]
pub mod tasks_stoprx;
#[doc = "Start UART transmitter\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_starttx](tasks_starttx) module"]
pub type TASKS_STARTTX = crate::Reg<u32, _TASKS_STARTTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTTX;
#[doc = "`write(|w| ..)` method takes [tasks_starttx::W](tasks_starttx::W) writer structure"]
impl crate::Writable for TASKS_STARTTX {}
#[doc = "Start UART transmitter"]
pub mod tasks_starttx;
#[doc = "Stop UART transmitter\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_stoptx](tasks_stoptx) module"]
pub type TASKS_STOPTX = crate::Reg<u32, _TASKS_STOPTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOPTX;
#[doc = "`write(|w| ..)` method takes [tasks_stoptx::W](tasks_stoptx::W) writer structure"]
impl crate::Writable for TASKS_STOPTX {}
#[doc = "Stop UART transmitter"]
pub mod tasks_stoptx;
#[doc = "Flush RX FIFO into RX buffer\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_flushrx](tasks_flushrx) module"]
pub type TASKS_FLUSHRX = crate::Reg<u32, _TASKS_FLUSHRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_FLUSHRX;
#[doc = "`write(|w| ..)` method takes [tasks_flushrx::W](tasks_flushrx::W) writer structure"]
impl crate::Writable for TASKS_FLUSHRX {}
#[doc = "Flush RX FIFO into RX buffer"]
pub mod tasks_flushrx;
#[doc = "Subscribe configuration for task STARTRX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_startrx](subscribe_startrx) module"]
pub type SUBSCRIBE_STARTRX = crate::Reg<u32, _SUBSCRIBE_STARTRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_STARTRX;
#[doc = "`read()` method returns [subscribe_startrx::R](subscribe_startrx::R) reader structure"]
impl crate::Readable for SUBSCRIBE_STARTRX {}
#[doc = "`write(|w| ..)` method takes [subscribe_startrx::W](subscribe_startrx::W) writer structure"]
impl crate::Writable for SUBSCRIBE_STARTRX {}
#[doc = "Subscribe configuration for task STARTRX"]
pub mod subscribe_startrx;
#[doc = "Subscribe configuration for task STOPRX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_stoprx](subscribe_stoprx) module"]
pub type SUBSCRIBE_STOPRX = crate::Reg<u32, _SUBSCRIBE_STOPRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_STOPRX;
#[doc = "`read()` method returns [subscribe_stoprx::R](subscribe_stoprx::R) reader structure"]
impl crate::Readable for SUBSCRIBE_STOPRX {}
#[doc = "`write(|w| ..)` method takes [subscribe_stoprx::W](subscribe_stoprx::W) writer structure"]
impl crate::Writable for SUBSCRIBE_STOPRX {}
#[doc = "Subscribe configuration for task STOPRX"]
pub mod subscribe_stoprx;
#[doc = "Subscribe configuration for task STARTTX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_starttx](subscribe_starttx) module"]
pub type SUBSCRIBE_STARTTX = crate::Reg<u32, _SUBSCRIBE_STARTTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_STARTTX;
#[doc = "`read()` method returns [subscribe_starttx::R](subscribe_starttx::R) reader structure"]
impl crate::Readable for SUBSCRIBE_STARTTX {}
#[doc = "`write(|w| ..)` method takes [subscribe_starttx::W](subscribe_starttx::W) writer structure"]
impl crate::Writable for SUBSCRIBE_STARTTX {}
#[doc = "Subscribe configuration for task STARTTX"]
pub mod subscribe_starttx;
#[doc = "Subscribe configuration for task STOPTX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_stoptx](subscribe_stoptx) module"]
pub type SUBSCRIBE_STOPTX = crate::Reg<u32, _SUBSCRIBE_STOPTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_STOPTX;
#[doc = "`read()` method returns [subscribe_stoptx::R](subscribe_stoptx::R) reader structure"]
impl crate::Readable for SUBSCRIBE_STOPTX {}
#[doc = "`write(|w| ..)` method takes [subscribe_stoptx::W](subscribe_stoptx::W) writer structure"]
impl crate::Writable for SUBSCRIBE_STOPTX {}
#[doc = "Subscribe configuration for task STOPTX"]
pub mod subscribe_stoptx;
#[doc = "Subscribe configuration for task FLUSHRX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_flushrx](subscribe_flushrx) module"]
pub type SUBSCRIBE_FLUSHRX = crate::Reg<u32, _SUBSCRIBE_FLUSHRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_FLUSHRX;
#[doc = "`read()` method returns [subscribe_flushrx::R](subscribe_flushrx::R) reader structure"]
impl crate::Readable for SUBSCRIBE_FLUSHRX {}
#[doc = "`write(|w| ..)` method takes [subscribe_flushrx::W](subscribe_flushrx::W) writer structure"]
impl crate::Writable for SUBSCRIBE_FLUSHRX {}
#[doc = "Subscribe configuration for task FLUSHRX"]
pub mod subscribe_flushrx;
#[doc = "CTS is activated (set low). Clear To Send.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_cts](events_cts) module"]
pub type EVENTS_CTS = crate::Reg<u32, _EVENTS_CTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_CTS;
#[doc = "`read()` method returns [events_cts::R](events_cts::R) reader structure"]
impl crate::Readable for EVENTS_CTS {}
#[doc = "`write(|w| ..)` method takes [events_cts::W](events_cts::W) writer structure"]
impl crate::Writable for EVENTS_CTS {}
#[doc = "CTS is activated (set low). Clear To Send."]
pub mod events_cts;
#[doc = "CTS is deactivated (set high). Not Clear To Send.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_ncts](events_ncts) module"]
pub type EVENTS_NCTS = crate::Reg<u32, _EVENTS_NCTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_NCTS;
#[doc = "`read()` method returns [events_ncts::R](events_ncts::R) reader structure"]
impl crate::Readable for EVENTS_NCTS {}
#[doc = "`write(|w| ..)` method takes [events_ncts::W](events_ncts::W) writer structure"]
impl crate::Writable for EVENTS_NCTS {}
#[doc = "CTS is deactivated (set high). Not Clear To Send."]
pub mod events_ncts;
#[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_rxdrdy](events_rxdrdy) module"]
pub type EVENTS_RXDRDY = crate::Reg<u32, _EVENTS_RXDRDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXDRDY;
#[doc = "`read()` method returns [events_rxdrdy::R](events_rxdrdy::R) reader structure"]
impl crate::Readable for EVENTS_RXDRDY {}
#[doc = "`write(|w| ..)` method takes [events_rxdrdy::W](events_rxdrdy::W) writer structure"]
impl crate::Writable for EVENTS_RXDRDY {}
#[doc = "Data received in RXD (but potentially not yet transferred to Data RAM)"]
pub mod events_rxdrdy;
#[doc = "Receive buffer is filled up\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_endrx](events_endrx) module"]
pub type EVENTS_ENDRX = crate::Reg<u32, _EVENTS_ENDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDRX;
#[doc = "`read()` method returns [events_endrx::R](events_endrx::R) reader structure"]
impl crate::Readable for EVENTS_ENDRX {}
#[doc = "`write(|w| ..)` method takes [events_endrx::W](events_endrx::W) writer structure"]
impl crate::Writable for EVENTS_ENDRX {}
#[doc = "Receive buffer is filled up"]
pub mod events_endrx;
#[doc = "Data sent from TXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_txdrdy](events_txdrdy) module"]
pub type EVENTS_TXDRDY = crate::Reg<u32, _EVENTS_TXDRDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXDRDY;
#[doc = "`read()` method returns [events_txdrdy::R](events_txdrdy::R) reader structure"]
impl crate::Readable for EVENTS_TXDRDY {}
#[doc = "`write(|w| ..)` method takes [events_txdrdy::W](events_txdrdy::W) writer structure"]
impl crate::Writable for EVENTS_TXDRDY {}
#[doc = "Data sent from TXD"]
pub mod events_txdrdy;
#[doc = "Last TX byte transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_endtx](events_endtx) module"]
pub type EVENTS_ENDTX = crate::Reg<u32, _EVENTS_ENDTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDTX;
#[doc = "`read()` method returns [events_endtx::R](events_endtx::R) reader structure"]
impl crate::Readable for EVENTS_ENDTX {}
#[doc = "`write(|w| ..)` method takes [events_endtx::W](events_endtx::W) writer structure"]
impl crate::Writable for EVENTS_ENDTX {}
#[doc = "Last TX byte transmitted"]
pub mod events_endtx;
#[doc = "Error detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_error](events_error) module"]
pub type EVENTS_ERROR = crate::Reg<u32, _EVENTS_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ERROR;
#[doc = "`read()` method returns [events_error::R](events_error::R) reader structure"]
impl crate::Readable for EVENTS_ERROR {}
#[doc = "`write(|w| ..)` method takes [events_error::W](events_error::W) writer structure"]
impl crate::Writable for EVENTS_ERROR {}
#[doc = "Error detected"]
pub mod events_error;
#[doc = "Receiver timeout\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_rxto](events_rxto) module"]
pub type EVENTS_RXTO = crate::Reg<u32, _EVENTS_RXTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXTO;
#[doc = "`read()` method returns [events_rxto::R](events_rxto::R) reader structure"]
impl crate::Readable for EVENTS_RXTO {}
#[doc = "`write(|w| ..)` method takes [events_rxto::W](events_rxto::W) writer structure"]
impl crate::Writable for EVENTS_RXTO {}
#[doc = "Receiver timeout"]
pub mod events_rxto;
#[doc = "UART receiver has started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_rxstarted](events_rxstarted) module"]
pub type EVENTS_RXSTARTED = crate::Reg<u32, _EVENTS_RXSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXSTARTED;
#[doc = "`read()` method returns [events_rxstarted::R](events_rxstarted::R) reader structure"]
impl crate::Readable for EVENTS_RXSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_rxstarted::W](events_rxstarted::W) writer structure"]
impl crate::Writable for EVENTS_RXSTARTED {}
#[doc = "UART receiver has started"]
pub mod events_rxstarted;
#[doc = "UART transmitter has started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_txstarted](events_txstarted) module"]
pub type EVENTS_TXSTARTED = crate::Reg<u32, _EVENTS_TXSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXSTARTED;
#[doc = "`read()` method returns [events_txstarted::R](events_txstarted::R) reader structure"]
impl crate::Readable for EVENTS_TXSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_txstarted::W](events_txstarted::W) writer structure"]
impl crate::Writable for EVENTS_TXSTARTED {}
#[doc = "UART transmitter has started"]
pub mod events_txstarted;
#[doc = "Transmitter stopped\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_txstopped](events_txstopped) module"]
pub type EVENTS_TXSTOPPED = crate::Reg<u32, _EVENTS_TXSTOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXSTOPPED;
#[doc = "`read()` method returns [events_txstopped::R](events_txstopped::R) reader structure"]
impl crate::Readable for EVENTS_TXSTOPPED {}
#[doc = "`write(|w| ..)` method takes [events_txstopped::W](events_txstopped::W) writer structure"]
impl crate::Writable for EVENTS_TXSTOPPED {}
#[doc = "Transmitter stopped"]
pub mod events_txstopped;
#[doc = "Publish configuration for event CTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_cts](publish_cts) module"]
pub type PUBLISH_CTS = crate::Reg<u32, _PUBLISH_CTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_CTS;
#[doc = "`read()` method returns [publish_cts::R](publish_cts::R) reader structure"]
impl crate::Readable for PUBLISH_CTS {}
#[doc = "`write(|w| ..)` method takes [publish_cts::W](publish_cts::W) writer structure"]
impl crate::Writable for PUBLISH_CTS {}
#[doc = "Publish configuration for event CTS"]
pub mod publish_cts;
#[doc = "Publish configuration for event NCTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_ncts](publish_ncts) module"]
pub type PUBLISH_NCTS = crate::Reg<u32, _PUBLISH_NCTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_NCTS;
#[doc = "`read()` method returns [publish_ncts::R](publish_ncts::R) reader structure"]
impl crate::Readable for PUBLISH_NCTS {}
#[doc = "`write(|w| ..)` method takes [publish_ncts::W](publish_ncts::W) writer structure"]
impl crate::Writable for PUBLISH_NCTS {}
#[doc = "Publish configuration for event NCTS"]
pub mod publish_ncts;
#[doc = "Publish configuration for event RXDRDY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_rxdrdy](publish_rxdrdy) module"]
pub type PUBLISH_RXDRDY = crate::Reg<u32, _PUBLISH_RXDRDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_RXDRDY;
#[doc = "`read()` method returns [publish_rxdrdy::R](publish_rxdrdy::R) reader structure"]
impl crate::Readable for PUBLISH_RXDRDY {}
#[doc = "`write(|w| ..)` method takes [publish_rxdrdy::W](publish_rxdrdy::W) writer structure"]
impl crate::Writable for PUBLISH_RXDRDY {}
#[doc = "Publish configuration for event RXDRDY"]
pub mod publish_rxdrdy;
#[doc = "Publish configuration for event ENDRX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_endrx](publish_endrx) module"]
pub type PUBLISH_ENDRX = crate::Reg<u32, _PUBLISH_ENDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_ENDRX;
#[doc = "`read()` method returns [publish_endrx::R](publish_endrx::R) reader structure"]
impl crate::Readable for PUBLISH_ENDRX {}
#[doc = "`write(|w| ..)` method takes [publish_endrx::W](publish_endrx::W) writer structure"]
impl crate::Writable for PUBLISH_ENDRX {}
#[doc = "Publish configuration for event ENDRX"]
pub mod publish_endrx;
#[doc = "Publish configuration for event TXDRDY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_txdrdy](publish_txdrdy) module"]
pub type PUBLISH_TXDRDY = crate::Reg<u32, _PUBLISH_TXDRDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_TXDRDY;
#[doc = "`read()` method returns [publish_txdrdy::R](publish_txdrdy::R) reader structure"]
impl crate::Readable for PUBLISH_TXDRDY {}
#[doc = "`write(|w| ..)` method takes [publish_txdrdy::W](publish_txdrdy::W) writer structure"]
impl crate::Writable for PUBLISH_TXDRDY {}
#[doc = "Publish configuration for event TXDRDY"]
pub mod publish_txdrdy;
#[doc = "Publish configuration for event ENDTX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_endtx](publish_endtx) module"]
pub type PUBLISH_ENDTX = crate::Reg<u32, _PUBLISH_ENDTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_ENDTX;
#[doc = "`read()` method returns [publish_endtx::R](publish_endtx::R) reader structure"]
impl crate::Readable for PUBLISH_ENDTX {}
#[doc = "`write(|w| ..)` method takes [publish_endtx::W](publish_endtx::W) writer structure"]
impl crate::Writable for PUBLISH_ENDTX {}
#[doc = "Publish configuration for event ENDTX"]
pub mod publish_endtx;
#[doc = "Publish configuration for event ERROR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_error](publish_error) module"]
pub type PUBLISH_ERROR = crate::Reg<u32, _PUBLISH_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_ERROR;
#[doc = "`read()` method returns [publish_error::R](publish_error::R) reader structure"]
impl crate::Readable for PUBLISH_ERROR {}
#[doc = "`write(|w| ..)` method takes [publish_error::W](publish_error::W) writer structure"]
impl crate::Writable for PUBLISH_ERROR {}
#[doc = "Publish configuration for event ERROR"]
pub mod publish_error;
#[doc = "Publish configuration for event RXTO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_rxto](publish_rxto) module"]
pub type PUBLISH_RXTO = crate::Reg<u32, _PUBLISH_RXTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_RXTO;
#[doc = "`read()` method returns [publish_rxto::R](publish_rxto::R) reader structure"]
impl crate::Readable for PUBLISH_RXTO {}
#[doc = "`write(|w| ..)` method takes [publish_rxto::W](publish_rxto::W) writer structure"]
impl crate::Writable for PUBLISH_RXTO {}
#[doc = "Publish configuration for event RXTO"]
pub mod publish_rxto;
#[doc = "Publish configuration for event RXSTARTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_rxstarted](publish_rxstarted) module"]
pub type PUBLISH_RXSTARTED = crate::Reg<u32, _PUBLISH_RXSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_RXSTARTED;
#[doc = "`read()` method returns [publish_rxstarted::R](publish_rxstarted::R) reader structure"]
impl crate::Readable for PUBLISH_RXSTARTED {}
#[doc = "`write(|w| ..)` method takes [publish_rxstarted::W](publish_rxstarted::W) writer structure"]
impl crate::Writable for PUBLISH_RXSTARTED {}
#[doc = "Publish configuration for event RXSTARTED"]
pub mod publish_rxstarted;
#[doc = "Publish configuration for event TXSTARTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_txstarted](publish_txstarted) module"]
pub type PUBLISH_TXSTARTED = crate::Reg<u32, _PUBLISH_TXSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_TXSTARTED;
#[doc = "`read()` method returns [publish_txstarted::R](publish_txstarted::R) reader structure"]
impl crate::Readable for PUBLISH_TXSTARTED {}
#[doc = "`write(|w| ..)` method takes [publish_txstarted::W](publish_txstarted::W) writer structure"]
impl crate::Writable for PUBLISH_TXSTARTED {}
#[doc = "Publish configuration for event TXSTARTED"]
pub mod publish_txstarted;
#[doc = "Publish configuration for event TXSTOPPED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_txstopped](publish_txstopped) module"]
pub type PUBLISH_TXSTOPPED = crate::Reg<u32, _PUBLISH_TXSTOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_TXSTOPPED;
#[doc = "`read()` method returns [publish_txstopped::R](publish_txstopped::R) reader structure"]
impl crate::Readable for PUBLISH_TXSTOPPED {}
#[doc = "`write(|w| ..)` method takes [publish_txstopped::W](publish_txstopped::W) writer structure"]
impl crate::Writable for PUBLISH_TXSTOPPED {}
#[doc = "Publish configuration for event TXSTOPPED"]
pub mod publish_txstopped;
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Error source Note : this register is read / write one to clear.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [errorsrc](errorsrc) module"]
pub type ERRORSRC = crate::Reg<u32, _ERRORSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRORSRC;
#[doc = "`read()` method returns [errorsrc::R](errorsrc::R) reader structure"]
impl crate::Readable for ERRORSRC {}
#[doc = "`write(|w| ..)` method takes [errorsrc::W](errorsrc::W) writer structure"]
impl crate::Writable for ERRORSRC {}
#[doc = "Error source Note : this register is read / write one to clear."]
pub mod errorsrc;
#[doc = "Enable UART\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable UART"]
pub mod enable;
#[doc = "Baud rate. Accuracy depends on the HFCLK source selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [baudrate](baudrate) module"]
pub type BAUDRATE = crate::Reg<u32, _BAUDRATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUDRATE;
#[doc = "`read()` method returns [baudrate::R](baudrate::R) reader structure"]
impl crate::Readable for BAUDRATE {}
#[doc = "`write(|w| ..)` method takes [baudrate::W](baudrate::W) writer structure"]
impl crate::Writable for BAUDRATE {}
#[doc = "Baud rate. Accuracy depends on the HFCLK source selected."]
pub mod baudrate;
#[doc = "Configuration of parity and hardware flow control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configuration of parity and hardware flow control"]
pub mod config;

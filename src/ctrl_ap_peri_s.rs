#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - Unspecified"]
    pub mailbox: MAILBOX,
    _reserved1: [u8; 120usize],
    #[doc = "0x500 - Unspecified"]
    pub eraseprotect: ERASEPROTECT,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct MAILBOX {
    #[doc = "0x00 - Data sent from the debugger to the CPU"]
    pub rxdata: self::mailbox::RXDATA,
    #[doc = "0x04 - Status to indicate if data sent from the debugger to the CPU has been read"]
    pub rxstatus: self::mailbox::RXSTATUS,
    _reserved0: [u8; 120usize],
    #[doc = "0x80 - Data sent from the CPU to the debugger"]
    pub txdata: self::mailbox::TXDATA,
    #[doc = "0x84 - Status to indicate if data sent from the CPU to the debugger status has been read"]
    pub txstatus: self::mailbox::TXSTATUS,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod mailbox;
#[doc = r" Register block"]
#[repr(C)]
pub struct ERASEPROTECT {
    #[doc = "0x00 - Lock ERASEALL mechanism"]
    pub lock: self::eraseprotect::LOCK,
    #[doc = "0x04 - Unlock ERASEPROTECT and perform ERASEALL"]
    pub disable: self::eraseprotect::DISABLE,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod eraseprotect;

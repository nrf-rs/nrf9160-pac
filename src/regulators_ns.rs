#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1280usize],
    #[doc = "0x500 - System OFF register"]
    pub systemoff: SYSTEMOFF,
    _reserved1: [u8; 12usize],
    #[doc = "0x510 - Power-fail comparator configuration"]
    pub pofcon: POFCON,
    _reserved2: [u8; 100usize],
    #[doc = "0x578 - Enable DC/DC mode of the main voltage regulator"]
    pub dcdcen: DCDCEN,
}
#[doc = "System OFF register"]
pub struct SYSTEMOFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "Power-fail comparator configuration"]
pub struct POFCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power-fail comparator configuration"]
pub mod pofcon;
#[doc = "Enable DC/DC mode of the main voltage regulator"]
pub struct DCDCEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable DC/DC mode of the main voltage regulator"]
pub mod dcdcen;

#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel group tasks"]
    pub tasks_chg: [TASKS_CHG; 6],
    _reserved0: [u8; 80usize],
    #[doc = "0x80 - Subscribe configuration for tasks"]
    pub subscribe_chg: [SUBSCRIBE_CHG; 6],
    _reserved1: [u8; 1104usize],
    #[doc = "0x500 - Channel enable register"]
    pub chen: CHEN,
    #[doc = "0x504 - Channel enable set register"]
    pub chenset: CHENSET,
    #[doc = "0x508 - Channel enable clear register"]
    pub chenclr: CHENCLR,
    _reserved2: [u8; 756usize],
    #[doc = "0x800 - Description collection: Channel group n Note: Writes to this register is ignored if either SUBSCRIBE_CHG[n].EN/DIS are enabled."]
    pub chg: [CHG; 6],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct TASKS_CHG {
    #[doc = "0x00 - Description cluster: Enable channel group n"]
    pub en: self::tasks_chg::EN,
    #[doc = "0x04 - Description cluster: Disable channel group n"]
    pub dis: self::tasks_chg::DIS,
}
#[doc = r" Register block"]
#[doc = "Channel group tasks"]
pub mod tasks_chg;
#[doc = r" Register block"]
#[repr(C)]
pub struct SUBSCRIBE_CHG {
    #[doc = "0x00 - Description cluster: Subscribe configuration for task CHG[n].EN"]
    pub en: self::subscribe_chg::EN,
    #[doc = "0x04 - Description cluster: Subscribe configuration for task CHG[n].DIS"]
    pub dis: self::subscribe_chg::DIS,
}
#[doc = r" Register block"]
#[doc = "Subscribe configuration for tasks"]
pub mod subscribe_chg;
#[doc = "Channel enable register"]
pub struct CHEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel enable register"]
pub mod chen;
#[doc = "Channel enable set register"]
pub struct CHENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel enable set register"]
pub mod chenset;
#[doc = "Channel enable clear register"]
pub struct CHENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel enable clear register"]
pub mod chenclr;
#[doc = "Description collection: Channel group n Note: Writes to this register is ignored if either SUBSCRIBE_CHG[n].EN/DIS are enabled."]
pub struct CHG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Channel group n Note: Writes to this register is ignored if either SUBSCRIBE_CHG[n].EN/DIS are enabled."]
pub mod chg;

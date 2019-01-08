#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 120usize],
    #[doc = "0x78 - Enable constant latency mode."]
    pub tasks_constlat: TASKS_CONSTLAT,
    #[doc = "0x7c - Enable low power mode (variable latency)"]
    pub tasks_lowpwr: TASKS_LOWPWR,
    _reserved1: [u8; 120usize],
    #[doc = "0xf8 - Subscribe configuration for task CONSTLAT"]
    pub subscribe_constlat: SUBSCRIBE_CONSTLAT,
    #[doc = "0xfc - Subscribe configuration for task LOWPWR"]
    pub subscribe_lowpwr: SUBSCRIBE_LOWPWR,
    _reserved2: [u8; 8usize],
    #[doc = "0x108 - Power failure warning"]
    pub events_pofwarn: EVENTS_POFWARN,
    _reserved3: [u8; 8usize],
    #[doc = "0x114 - CPU entered WFI/WFE sleep"]
    pub events_sleepenter: EVENTS_SLEEPENTER,
    #[doc = "0x118 - CPU exited WFI/WFE sleep"]
    pub events_sleepexit: EVENTS_SLEEPEXIT,
    _reserved4: [u8; 108usize],
    #[doc = "0x188 - Publish configuration for event POFWARN"]
    pub publish_pofwarn: PUBLISH_POFWARN,
    _reserved5: [u8; 8usize],
    #[doc = "0x194 - Publish configuration for event SLEEPENTER"]
    pub publish_sleepenter: PUBLISH_SLEEPENTER,
    #[doc = "0x198 - Publish configuration for event SLEEPEXIT"]
    pub publish_sleepexit: PUBLISH_SLEEPEXIT,
    _reserved6: [u8; 356usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved7: [u8; 244usize],
    #[doc = "0x400 - Reset reason"]
    pub resetreas: RESETREAS,
    _reserved8: [u8; 60usize],
    #[doc = "0x440 - Modem domain power status"]
    pub powerstatus: POWERSTATUS,
    _reserved9: [u8; 216usize],
    #[doc = "0x51c - Description collection: General purpose retention register"]
    pub gpregret: [GPREGRET; 2],
}
#[doc = "Enable constant latency mode."]
pub struct TASKS_CONSTLAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable constant latency mode."]
pub mod tasks_constlat;
#[doc = "Enable low power mode (variable latency)"]
pub struct TASKS_LOWPWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable low power mode (variable latency)"]
pub mod tasks_lowpwr;
#[doc = "Subscribe configuration for task CONSTLAT"]
pub struct SUBSCRIBE_CONSTLAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task CONSTLAT"]
pub mod subscribe_constlat;
#[doc = "Subscribe configuration for task LOWPWR"]
pub struct SUBSCRIBE_LOWPWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task LOWPWR"]
pub mod subscribe_lowpwr;
#[doc = "Power failure warning"]
pub struct EVENTS_POFWARN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power failure warning"]
pub mod events_pofwarn;
#[doc = "CPU entered WFI/WFE sleep"]
pub struct EVENTS_SLEEPENTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU entered WFI/WFE sleep"]
pub mod events_sleepenter;
#[doc = "CPU exited WFI/WFE sleep"]
pub struct EVENTS_SLEEPEXIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU exited WFI/WFE sleep"]
pub mod events_sleepexit;
#[doc = "Publish configuration for event POFWARN"]
pub struct PUBLISH_POFWARN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event POFWARN"]
pub mod publish_pofwarn;
#[doc = "Publish configuration for event SLEEPENTER"]
pub struct PUBLISH_SLEEPENTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event SLEEPENTER"]
pub mod publish_sleepenter;
#[doc = "Publish configuration for event SLEEPEXIT"]
pub struct PUBLISH_SLEEPEXIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event SLEEPEXIT"]
pub mod publish_sleepexit;
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
#[doc = "Reset reason"]
pub struct RESETREAS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset reason"]
pub mod resetreas;
#[doc = "Modem domain power status"]
pub struct POWERSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Modem domain power status"]
pub mod powerstatus;
#[doc = "Description collection: General purpose retention register"]
pub struct GPREGRET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: General purpose retention register"]
pub mod gpregret;

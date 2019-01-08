#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is configured in CONFIG[n].POLARITY."]
    pub tasks_out: [TASKS_OUT; 8],
    _reserved0: [u8; 16usize],
    #[doc = "0x30 - Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is to set it high."]
    pub tasks_set: [TASKS_SET; 8],
    _reserved1: [u8; 16usize],
    #[doc = "0x60 - Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is to set it low."]
    pub tasks_clr: [TASKS_CLR; 8],
    #[doc = "0x80 - Description collection: Subscribe configuration for task OUT[n]"]
    pub subscribe_out: [SUBSCRIBE_OUT; 8],
    _reserved2: [u8; 16usize],
    #[doc = "0xb0 - Description collection: Subscribe configuration for task SET[n]"]
    pub subscribe_set: [SUBSCRIBE_SET; 8],
    _reserved3: [u8; 16usize],
    #[doc = "0xe0 - Description collection: Subscribe configuration for task CLR[n]"]
    pub subscribe_clr: [SUBSCRIBE_CLR; 8],
    #[doc = "0x100 - Description collection: Event generated from pin specified in CONFIG[n].PSEL"]
    pub events_in: [EVENTS_IN; 8],
    _reserved4: [u8; 92usize],
    #[doc = "0x17c - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
    pub events_port: EVENTS_PORT,
    #[doc = "0x180 - Description collection: Publish configuration for event IN[n]"]
    pub publish_in: [PUBLISH_IN; 8],
    _reserved5: [u8; 92usize],
    #[doc = "0x1fc - Publish configuration for event PORT"]
    pub publish_port: PUBLISH_PORT,
    _reserved6: [u8; 260usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved7: [u8; 516usize],
    #[doc = "0x510 - Description collection: Configuration for OUT[n], SET[n] and CLR[n] tasks and IN[n] event"]
    pub config: [CONFIG; 8],
}
#[doc = "Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is configured in CONFIG[n].POLARITY."]
pub struct TASKS_OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is configured in CONFIG[n].POLARITY."]
pub mod tasks_out;
#[doc = "Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is to set it high."]
pub struct TASKS_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is to set it high."]
pub mod tasks_set;
#[doc = "Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is to set it low."]
pub struct TASKS_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is to set it low."]
pub mod tasks_clr;
#[doc = "Description collection: Subscribe configuration for task OUT[n]"]
pub struct SUBSCRIBE_OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Subscribe configuration for task OUT[n]"]
pub mod subscribe_out;
#[doc = "Description collection: Subscribe configuration for task SET[n]"]
pub struct SUBSCRIBE_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Subscribe configuration for task SET[n]"]
pub mod subscribe_set;
#[doc = "Description collection: Subscribe configuration for task CLR[n]"]
pub struct SUBSCRIBE_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Subscribe configuration for task CLR[n]"]
pub mod subscribe_clr;
#[doc = "Description collection: Event generated from pin specified in CONFIG[n].PSEL"]
pub struct EVENTS_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Event generated from pin specified in CONFIG[n].PSEL"]
pub mod events_in;
#[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
pub struct EVENTS_PORT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
pub mod events_port;
#[doc = "Description collection: Publish configuration for event IN[n]"]
pub struct PUBLISH_IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Publish configuration for event IN[n]"]
pub mod publish_in;
#[doc = "Publish configuration for event PORT"]
pub struct PUBLISH_PORT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event PORT"]
pub mod publish_port;
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
#[doc = "Description collection: Configuration for OUT[n], SET[n] and CLR[n] tasks and IN[n] event"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Configuration for OUT[n], SET[n] and CLR[n] tasks and IN[n] event"]
pub mod config;

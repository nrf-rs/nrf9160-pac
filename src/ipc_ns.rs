#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Description collection: Trigger events on channel enabled in SEND_CNF\\[n\\]."]
    pub tasks_send: [TASKS_SEND; 8],
    _reserved0: [u8; 96usize],
    #[doc = "0x80 - Description collection: Subscribe configuration for task SEND\\[n\\]"]
    pub subscribe_send: [SUBSCRIBE_SEND; 8],
    _reserved1: [u8; 96usize],
    #[doc = "0x100 - Description collection: Event received on one or more of the enabled channels in RECEIVE_CNF\\[n\\]."]
    pub events_receive: [EVENTS_RECEIVE; 8],
    _reserved2: [u8; 96usize],
    #[doc = "0x180 - Description collection: Publish configuration for event RECEIVE\\[n\\]"]
    pub publish_receive: [PUBLISH_RECEIVE; 8],
    _reserved3: [u8; 352usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    #[doc = "0x30c - Pending interrupts"]
    pub intpend: INTPEND,
    _reserved4: [u8; 512usize],
    #[doc = "0x510 - Description collection: Send event configuration for TASKS_SEND\\[n\\]."]
    pub send_cnf: [SEND_CNF; 8],
    _reserved5: [u8; 96usize],
    #[doc = "0x590 - Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]."]
    pub receive_cnf: [RECEIVE_CNF; 8],
    _reserved6: [u8; 96usize],
    #[doc = "0x610 - Description collection: General purpose memory."]
    pub gpmem: [GPMEM; 4],
}
#[doc = "Description collection: Trigger events on channel enabled in SEND_CNF\\[n\\]."]
pub struct TASKS_SEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Trigger events on channel enabled in SEND_CNF\\[n\\]."]
pub mod tasks_send;
#[doc = "Description collection: Subscribe configuration for task SEND\\[n\\]"]
pub struct SUBSCRIBE_SEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Subscribe configuration for task SEND\\[n\\]"]
pub mod subscribe_send;
#[doc = "Description collection: Event received on one or more of the enabled channels in RECEIVE_CNF\\[n\\]."]
pub struct EVENTS_RECEIVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Event received on one or more of the enabled channels in RECEIVE_CNF\\[n\\]."]
pub mod events_receive;
#[doc = "Description collection: Publish configuration for event RECEIVE\\[n\\]"]
pub struct PUBLISH_RECEIVE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Publish configuration for event RECEIVE\\[n\\]"]
pub mod publish_receive;
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
#[doc = "Pending interrupts"]
pub struct INTPEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pending interrupts"]
pub mod intpend;
#[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]."]
pub struct SEND_CNF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]."]
pub mod send_cnf;
#[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]."]
pub struct RECEIVE_CNF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]."]
pub mod receive_cnf;
#[doc = "Description collection: General purpose memory."]
pub struct GPMEM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: General purpose memory."]
pub mod gpmem;

#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start RTC counter"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop RTC counter"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Clear RTC counter"]
    pub tasks_clear: TASKS_CLEAR,
    #[doc = "0x0c - Set counter to 0xFFFFF0"]
    pub tasks_trigovrflw: TASKS_TRIGOVRFLW,
    _reserved0: [u8; 112usize],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    #[doc = "0x88 - Subscribe configuration for task CLEAR"]
    pub subscribe_clear: SUBSCRIBE_CLEAR,
    #[doc = "0x8c - Subscribe configuration for task TRIGOVRFLW"]
    pub subscribe_trigovrflw: SUBSCRIBE_TRIGOVRFLW,
    _reserved1: [u8; 112usize],
    #[doc = "0x100 - Event on counter increment"]
    pub events_tick: EVENTS_TICK,
    #[doc = "0x104 - Event on counter overflow"]
    pub events_ovrflw: EVENTS_OVRFLW,
    _reserved2: [u8; 56usize],
    #[doc = "0x140 - Description collection: Compare event on CC\\[n\\] match"]
    pub events_compare: [EVENTS_COMPARE; 4],
    _reserved3: [u8; 48usize],
    #[doc = "0x180 - Publish configuration for event TICK"]
    pub publish_tick: PUBLISH_TICK,
    #[doc = "0x184 - Publish configuration for event OVRFLW"]
    pub publish_ovrflw: PUBLISH_OVRFLW,
    _reserved4: [u8; 56usize],
    #[doc = "0x1c0 - Description collection: Publish configuration for event COMPARE\\[n\\]"]
    pub publish_compare: [PUBLISH_COMPARE; 4],
    _reserved5: [u8; 308usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved6: [u8; 52usize],
    #[doc = "0x340 - Enable or disable event routing"]
    pub evten: EVTEN,
    #[doc = "0x344 - Enable event routing"]
    pub evtenset: EVTENSET,
    #[doc = "0x348 - Disable event routing"]
    pub evtenclr: EVTENCLR,
    _reserved7: [u8; 440usize],
    #[doc = "0x504 - Current counter value"]
    pub counter: COUNTER,
    #[doc = "0x508 - 12-bit prescaler for counter frequency (32768/(PRESCALER+1)). Must be written when RTC is stopped."]
    pub prescaler: PRESCALER,
    _reserved8: [u8; 52usize],
    #[doc = "0x540 - Description collection: Compare register n"]
    pub cc: [CC; 4],
}
#[doc = "Start RTC counter"]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start RTC counter"]
pub mod tasks_start;
#[doc = "Stop RTC counter"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop RTC counter"]
pub mod tasks_stop;
#[doc = "Clear RTC counter"]
pub struct TASKS_CLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear RTC counter"]
pub mod tasks_clear;
#[doc = "Set counter to 0xFFFFF0"]
pub struct TASKS_TRIGOVRFLW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set counter to 0xFFFFF0"]
pub mod tasks_trigovrflw;
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
#[doc = "Subscribe configuration for task CLEAR"]
pub struct SUBSCRIBE_CLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task CLEAR"]
pub mod subscribe_clear;
#[doc = "Subscribe configuration for task TRIGOVRFLW"]
pub struct SUBSCRIBE_TRIGOVRFLW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task TRIGOVRFLW"]
pub mod subscribe_trigovrflw;
#[doc = "Event on counter increment"]
pub struct EVENTS_TICK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event on counter increment"]
pub mod events_tick;
#[doc = "Event on counter overflow"]
pub struct EVENTS_OVRFLW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event on counter overflow"]
pub mod events_ovrflw;
#[doc = "Description collection: Compare event on CC\\[n\\] match"]
pub struct EVENTS_COMPARE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Compare event on CC\\[n\\] match"]
pub mod events_compare;
#[doc = "Publish configuration for event TICK"]
pub struct PUBLISH_TICK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event TICK"]
pub mod publish_tick;
#[doc = "Publish configuration for event OVRFLW"]
pub struct PUBLISH_OVRFLW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event OVRFLW"]
pub mod publish_ovrflw;
#[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
pub struct PUBLISH_COMPARE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
pub mod publish_compare;
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
#[doc = "Enable or disable event routing"]
pub struct EVTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable or disable event routing"]
pub mod evten;
#[doc = "Enable event routing"]
pub struct EVTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable event routing"]
pub mod evtenset;
#[doc = "Disable event routing"]
pub struct EVTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable event routing"]
pub mod evtenclr;
#[doc = "Current counter value"]
pub struct COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current counter value"]
pub mod counter;
#[doc = "12-bit prescaler for counter frequency (32768/(PRESCALER+1)). Must be written when RTC is stopped."]
pub struct PRESCALER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "12-bit prescaler for counter frequency (32768/(PRESCALER+1)). Must be written when RTC is stopped."]
pub mod prescaler;
#[doc = "Description collection: Compare register n"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Compare register n"]
pub mod cc;

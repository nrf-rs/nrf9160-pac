#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start HFCLK crystal oscillator"]
    pub tasks_hfclkstart: TASKS_HFCLKSTART,
    #[doc = "0x04 - Stop HFCLK crystal oscillator"]
    pub tasks_hfclkstop: TASKS_HFCLKSTOP,
    #[doc = "0x08 - Start LFCLK source"]
    pub tasks_lfclkstart: TASKS_LFCLKSTART,
    #[doc = "0x0c - Stop LFCLK source"]
    pub tasks_lfclkstop: TASKS_LFCLKSTOP,
    _reserved0: [u8; 112usize],
    #[doc = "0x80 - Subscribe configuration for task HFCLKSTART"]
    pub subscribe_hfclkstart: SUBSCRIBE_HFCLKSTART,
    #[doc = "0x84 - Subscribe configuration for task HFCLKSTOP"]
    pub subscribe_hfclkstop: SUBSCRIBE_HFCLKSTOP,
    #[doc = "0x88 - Subscribe configuration for task LFCLKSTART"]
    pub subscribe_lfclkstart: SUBSCRIBE_LFCLKSTART,
    #[doc = "0x8c - Subscribe configuration for task LFCLKSTOP"]
    pub subscribe_lfclkstop: SUBSCRIBE_LFCLKSTOP,
    _reserved1: [u8; 112usize],
    #[doc = "0x100 - HFCLK oscillator started"]
    pub events_hfclkstarted: EVENTS_HFCLKSTARTED,
    #[doc = "0x104 - LFCLK started"]
    pub events_lfclkstarted: EVENTS_LFCLKSTARTED,
    _reserved2: [u8; 120usize],
    #[doc = "0x180 - Publish configuration for event HFCLKSTARTED"]
    pub publish_hfclkstarted: PUBLISH_HFCLKSTARTED,
    #[doc = "0x184 - Publish configuration for event LFCLKSTARTED"]
    pub publish_lfclkstarted: PUBLISH_LFCLKSTARTED,
    _reserved3: [u8; 376usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    #[doc = "0x30c - Pending interrupts"]
    pub intpend: INTPEND,
    _reserved4: [u8; 248usize],
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    pub hfclkrun: HFCLKRUN,
    #[doc = "0x40c - The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)"]
    pub hfclkstat: HFCLKSTAT,
    _reserved5: [u8; 4usize],
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    pub lfclkrun: LFCLKRUN,
    #[doc = "0x418 - The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)"]
    pub lfclkstat: LFCLKSTAT,
    #[doc = "0x41c - Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered"]
    pub lfclksrccopy: LFCLKSRCCOPY,
    _reserved6: [u8; 248usize],
    #[doc = "0x518 - Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register."]
    pub lfclksrc: LFCLKSRC,
}
#[doc = "Start HFCLK crystal oscillator"]
pub struct TASKS_HFCLKSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start HFCLK crystal oscillator"]
pub mod tasks_hfclkstart;
#[doc = "Stop HFCLK crystal oscillator"]
pub struct TASKS_HFCLKSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop HFCLK crystal oscillator"]
pub mod tasks_hfclkstop;
#[doc = "Start LFCLK source"]
pub struct TASKS_LFCLKSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start LFCLK source"]
pub mod tasks_lfclkstart;
#[doc = "Stop LFCLK source"]
pub struct TASKS_LFCLKSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop LFCLK source"]
pub mod tasks_lfclkstop;
#[doc = "Subscribe configuration for task HFCLKSTART"]
pub struct SUBSCRIBE_HFCLKSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task HFCLKSTART"]
pub mod subscribe_hfclkstart;
#[doc = "Subscribe configuration for task HFCLKSTOP"]
pub struct SUBSCRIBE_HFCLKSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task HFCLKSTOP"]
pub mod subscribe_hfclkstop;
#[doc = "Subscribe configuration for task LFCLKSTART"]
pub struct SUBSCRIBE_LFCLKSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task LFCLKSTART"]
pub mod subscribe_lfclkstart;
#[doc = "Subscribe configuration for task LFCLKSTOP"]
pub struct SUBSCRIBE_LFCLKSTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task LFCLKSTOP"]
pub mod subscribe_lfclkstop;
#[doc = "HFCLK oscillator started"]
pub struct EVENTS_HFCLKSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HFCLK oscillator started"]
pub mod events_hfclkstarted;
#[doc = "LFCLK started"]
pub struct EVENTS_LFCLKSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LFCLK started"]
pub mod events_lfclkstarted;
#[doc = "Publish configuration for event HFCLKSTARTED"]
pub struct PUBLISH_HFCLKSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event HFCLKSTARTED"]
pub mod publish_hfclkstarted;
#[doc = "Publish configuration for event LFCLKSTARTED"]
pub struct PUBLISH_LFCLKSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event LFCLKSTARTED"]
pub mod publish_lfclkstarted;
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
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub struct HFCLKRUN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub mod hfclkrun;
#[doc = "The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)"]
pub struct HFCLKSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)"]
pub mod hfclkstat;
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub struct LFCLKRUN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub mod lfclkrun;
#[doc = "The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)"]
pub struct LFCLKSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)"]
pub mod lfclkstat;
#[doc = "Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered"]
pub struct LFCLKSRCCOPY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered"]
pub mod lfclksrccopy;
#[doc = "Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register."]
pub struct LFCLKSRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register."]
pub mod lfclksrc;

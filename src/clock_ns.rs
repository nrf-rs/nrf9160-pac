#[doc = r"Register block"]
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
    _reserved4: [u8; 112usize],
    #[doc = "0x80 - Subscribe configuration for task HFCLKSTART"]
    pub subscribe_hfclkstart: SUBSCRIBE_HFCLKSTART,
    #[doc = "0x84 - Subscribe configuration for task HFCLKSTOP"]
    pub subscribe_hfclkstop: SUBSCRIBE_HFCLKSTOP,
    #[doc = "0x88 - Subscribe configuration for task LFCLKSTART"]
    pub subscribe_lfclkstart: SUBSCRIBE_LFCLKSTART,
    #[doc = "0x8c - Subscribe configuration for task LFCLKSTOP"]
    pub subscribe_lfclkstop: SUBSCRIBE_LFCLKSTOP,
    _reserved8: [u8; 112usize],
    #[doc = "0x100 - HFCLK oscillator started"]
    pub events_hfclkstarted: EVENTS_HFCLKSTARTED,
    #[doc = "0x104 - LFCLK started"]
    pub events_lfclkstarted: EVENTS_LFCLKSTARTED,
    _reserved10: [u8; 120usize],
    #[doc = "0x180 - Publish configuration for event HFCLKSTARTED"]
    pub publish_hfclkstarted: PUBLISH_HFCLKSTARTED,
    #[doc = "0x184 - Publish configuration for event LFCLKSTARTED"]
    pub publish_lfclkstarted: PUBLISH_LFCLKSTARTED,
    _reserved12: [u8; 376usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    #[doc = "0x30c - Pending interrupts"]
    pub intpend: INTPEND,
    _reserved16: [u8; 248usize],
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    pub hfclkrun: HFCLKRUN,
    #[doc = "0x40c - The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)"]
    pub hfclkstat: HFCLKSTAT,
    _reserved18: [u8; 4usize],
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    pub lfclkrun: LFCLKRUN,
    #[doc = "0x418 - The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)"]
    pub lfclkstat: LFCLKSTAT,
    #[doc = "0x41c - Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered"]
    pub lfclksrccopy: LFCLKSRCCOPY,
    _reserved21: [u8; 248usize],
    #[doc = "0x518 - Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register."]
    pub lfclksrc: LFCLKSRC,
}
#[doc = "Start HFCLK crystal oscillator\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_hfclkstart](tasks_hfclkstart) module"]
pub type TASKS_HFCLKSTART = crate::Reg<u32, _TASKS_HFCLKSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_HFCLKSTART;
#[doc = "`write(|w| ..)` method takes [tasks_hfclkstart::W](tasks_hfclkstart::W) writer structure"]
impl crate::Writable for TASKS_HFCLKSTART {}
#[doc = "Start HFCLK crystal oscillator"]
pub mod tasks_hfclkstart;
#[doc = "Stop HFCLK crystal oscillator\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_hfclkstop](tasks_hfclkstop) module"]
pub type TASKS_HFCLKSTOP = crate::Reg<u32, _TASKS_HFCLKSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_HFCLKSTOP;
#[doc = "`write(|w| ..)` method takes [tasks_hfclkstop::W](tasks_hfclkstop::W) writer structure"]
impl crate::Writable for TASKS_HFCLKSTOP {}
#[doc = "Stop HFCLK crystal oscillator"]
pub mod tasks_hfclkstop;
#[doc = "Start LFCLK source\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_lfclkstart](tasks_lfclkstart) module"]
pub type TASKS_LFCLKSTART = crate::Reg<u32, _TASKS_LFCLKSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_LFCLKSTART;
#[doc = "`write(|w| ..)` method takes [tasks_lfclkstart::W](tasks_lfclkstart::W) writer structure"]
impl crate::Writable for TASKS_LFCLKSTART {}
#[doc = "Start LFCLK source"]
pub mod tasks_lfclkstart;
#[doc = "Stop LFCLK source\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_lfclkstop](tasks_lfclkstop) module"]
pub type TASKS_LFCLKSTOP = crate::Reg<u32, _TASKS_LFCLKSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_LFCLKSTOP;
#[doc = "`write(|w| ..)` method takes [tasks_lfclkstop::W](tasks_lfclkstop::W) writer structure"]
impl crate::Writable for TASKS_LFCLKSTOP {}
#[doc = "Stop LFCLK source"]
pub mod tasks_lfclkstop;
#[doc = "Subscribe configuration for task HFCLKSTART\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_hfclkstart](subscribe_hfclkstart) module"]
pub type SUBSCRIBE_HFCLKSTART = crate::Reg<u32, _SUBSCRIBE_HFCLKSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_HFCLKSTART;
#[doc = "`read()` method returns [subscribe_hfclkstart::R](subscribe_hfclkstart::R) reader structure"]
impl crate::Readable for SUBSCRIBE_HFCLKSTART {}
#[doc = "`write(|w| ..)` method takes [subscribe_hfclkstart::W](subscribe_hfclkstart::W) writer structure"]
impl crate::Writable for SUBSCRIBE_HFCLKSTART {}
#[doc = "Subscribe configuration for task HFCLKSTART"]
pub mod subscribe_hfclkstart;
#[doc = "Subscribe configuration for task HFCLKSTOP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_hfclkstop](subscribe_hfclkstop) module"]
pub type SUBSCRIBE_HFCLKSTOP = crate::Reg<u32, _SUBSCRIBE_HFCLKSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_HFCLKSTOP;
#[doc = "`read()` method returns [subscribe_hfclkstop::R](subscribe_hfclkstop::R) reader structure"]
impl crate::Readable for SUBSCRIBE_HFCLKSTOP {}
#[doc = "`write(|w| ..)` method takes [subscribe_hfclkstop::W](subscribe_hfclkstop::W) writer structure"]
impl crate::Writable for SUBSCRIBE_HFCLKSTOP {}
#[doc = "Subscribe configuration for task HFCLKSTOP"]
pub mod subscribe_hfclkstop;
#[doc = "Subscribe configuration for task LFCLKSTART\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_lfclkstart](subscribe_lfclkstart) module"]
pub type SUBSCRIBE_LFCLKSTART = crate::Reg<u32, _SUBSCRIBE_LFCLKSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_LFCLKSTART;
#[doc = "`read()` method returns [subscribe_lfclkstart::R](subscribe_lfclkstart::R) reader structure"]
impl crate::Readable for SUBSCRIBE_LFCLKSTART {}
#[doc = "`write(|w| ..)` method takes [subscribe_lfclkstart::W](subscribe_lfclkstart::W) writer structure"]
impl crate::Writable for SUBSCRIBE_LFCLKSTART {}
#[doc = "Subscribe configuration for task LFCLKSTART"]
pub mod subscribe_lfclkstart;
#[doc = "Subscribe configuration for task LFCLKSTOP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_lfclkstop](subscribe_lfclkstop) module"]
pub type SUBSCRIBE_LFCLKSTOP = crate::Reg<u32, _SUBSCRIBE_LFCLKSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_LFCLKSTOP;
#[doc = "`read()` method returns [subscribe_lfclkstop::R](subscribe_lfclkstop::R) reader structure"]
impl crate::Readable for SUBSCRIBE_LFCLKSTOP {}
#[doc = "`write(|w| ..)` method takes [subscribe_lfclkstop::W](subscribe_lfclkstop::W) writer structure"]
impl crate::Writable for SUBSCRIBE_LFCLKSTOP {}
#[doc = "Subscribe configuration for task LFCLKSTOP"]
pub mod subscribe_lfclkstop;
#[doc = "HFCLK oscillator started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_hfclkstarted](events_hfclkstarted) module"]
pub type EVENTS_HFCLKSTARTED = crate::Reg<u32, _EVENTS_HFCLKSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_HFCLKSTARTED;
#[doc = "`read()` method returns [events_hfclkstarted::R](events_hfclkstarted::R) reader structure"]
impl crate::Readable for EVENTS_HFCLKSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_hfclkstarted::W](events_hfclkstarted::W) writer structure"]
impl crate::Writable for EVENTS_HFCLKSTARTED {}
#[doc = "HFCLK oscillator started"]
pub mod events_hfclkstarted;
#[doc = "LFCLK started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_lfclkstarted](events_lfclkstarted) module"]
pub type EVENTS_LFCLKSTARTED = crate::Reg<u32, _EVENTS_LFCLKSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_LFCLKSTARTED;
#[doc = "`read()` method returns [events_lfclkstarted::R](events_lfclkstarted::R) reader structure"]
impl crate::Readable for EVENTS_LFCLKSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_lfclkstarted::W](events_lfclkstarted::W) writer structure"]
impl crate::Writable for EVENTS_LFCLKSTARTED {}
#[doc = "LFCLK started"]
pub mod events_lfclkstarted;
#[doc = "Publish configuration for event HFCLKSTARTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_hfclkstarted](publish_hfclkstarted) module"]
pub type PUBLISH_HFCLKSTARTED = crate::Reg<u32, _PUBLISH_HFCLKSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_HFCLKSTARTED;
#[doc = "`read()` method returns [publish_hfclkstarted::R](publish_hfclkstarted::R) reader structure"]
impl crate::Readable for PUBLISH_HFCLKSTARTED {}
#[doc = "`write(|w| ..)` method takes [publish_hfclkstarted::W](publish_hfclkstarted::W) writer structure"]
impl crate::Writable for PUBLISH_HFCLKSTARTED {}
#[doc = "Publish configuration for event HFCLKSTARTED"]
pub mod publish_hfclkstarted;
#[doc = "Publish configuration for event LFCLKSTARTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_lfclkstarted](publish_lfclkstarted) module"]
pub type PUBLISH_LFCLKSTARTED = crate::Reg<u32, _PUBLISH_LFCLKSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_LFCLKSTARTED;
#[doc = "`read()` method returns [publish_lfclkstarted::R](publish_lfclkstarted::R) reader structure"]
impl crate::Readable for PUBLISH_LFCLKSTARTED {}
#[doc = "`write(|w| ..)` method takes [publish_lfclkstarted::W](publish_lfclkstarted::W) writer structure"]
impl crate::Writable for PUBLISH_LFCLKSTARTED {}
#[doc = "Publish configuration for event LFCLKSTARTED"]
pub mod publish_lfclkstarted;
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
#[doc = "Pending interrupts\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intpend](intpend) module"]
pub type INTPEND = crate::Reg<u32, _INTPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPEND;
#[doc = "`read()` method returns [intpend::R](intpend::R) reader structure"]
impl crate::Readable for INTPEND {}
#[doc = "Pending interrupts"]
pub mod intpend;
#[doc = "Status indicating that HFCLKSTART task has been triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclkrun](hfclkrun) module"]
pub type HFCLKRUN = crate::Reg<u32, _HFCLKRUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKRUN;
#[doc = "`read()` method returns [hfclkrun::R](hfclkrun::R) reader structure"]
impl crate::Readable for HFCLKRUN {}
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub mod hfclkrun;
#[doc = "The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfclkstat](hfclkstat) module"]
pub type HFCLKSTAT = crate::Reg<u32, _HFCLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFCLKSTAT;
#[doc = "`read()` method returns [hfclkstat::R](hfclkstat::R) reader structure"]
impl crate::Readable for HFCLKSTAT {}
#[doc = "The register shows if HFXO has been requested by triggering HFCLKSTART task and if it has been started (STATE)"]
pub mod hfclkstat;
#[doc = "Status indicating that LFCLKSTART task has been triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfclkrun](lfclkrun) module"]
pub type LFCLKRUN = crate::Reg<u32, _LFCLKRUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKRUN;
#[doc = "`read()` method returns [lfclkrun::R](lfclkrun::R) reader structure"]
impl crate::Readable for LFCLKRUN {}
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub mod lfclkrun;
#[doc = "The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfclkstat](lfclkstat) module"]
pub type LFCLKSTAT = crate::Reg<u32, _LFCLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKSTAT;
#[doc = "`read()` method returns [lfclkstat::R](lfclkstat::R) reader structure"]
impl crate::Readable for LFCLKSTAT {}
#[doc = "The register shows which LFCLK source has been requested (SRC) when triggering LFCLKSTART task and if the source has been started (STATE)"]
pub mod lfclkstat;
#[doc = "Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfclksrccopy](lfclksrccopy) module"]
pub type LFCLKSRCCOPY = crate::Reg<u32, _LFCLKSRCCOPY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKSRCCOPY;
#[doc = "`read()` method returns [lfclksrccopy::R](lfclksrccopy::R) reader structure"]
impl crate::Readable for LFCLKSRCCOPY {}
#[doc = "Copy of LFCLKSRC register, set after LFCLKSTART task has been triggered"]
pub mod lfclksrccopy;
#[doc = "Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lfclksrc](lfclksrc) module"]
pub type LFCLKSRC = crate::Reg<u32, _LFCLKSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFCLKSRC;
#[doc = "`read()` method returns [lfclksrc::R](lfclksrc::R) reader structure"]
impl crate::Readable for LFCLKSRC {}
#[doc = "`write(|w| ..)` method takes [lfclksrc::W](lfclksrc::W) writer structure"]
impl crate::Writable for LFCLKSRC {}
#[doc = "Clock source for the LFCLK. LFCLKSTART task starts starts a clock source selected with this register."]
pub mod lfclksrc;

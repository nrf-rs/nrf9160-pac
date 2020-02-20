#[doc = r"Register block"]
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
    _reserved4: [u8; 112usize],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    #[doc = "0x88 - Subscribe configuration for task CLEAR"]
    pub subscribe_clear: SUBSCRIBE_CLEAR,
    #[doc = "0x8c - Subscribe configuration for task TRIGOVRFLW"]
    pub subscribe_trigovrflw: SUBSCRIBE_TRIGOVRFLW,
    _reserved8: [u8; 112usize],
    #[doc = "0x100 - Event on counter increment"]
    pub events_tick: EVENTS_TICK,
    #[doc = "0x104 - Event on counter overflow"]
    pub events_ovrflw: EVENTS_OVRFLW,
    _reserved10: [u8; 56usize],
    #[doc = "0x140 - Description collection: Compare event on CC\\[n\\]
match"]
    pub events_compare: [EVENTS_COMPARE; 4],
    _reserved11: [u8; 48usize],
    #[doc = "0x180 - Publish configuration for event TICK"]
    pub publish_tick: PUBLISH_TICK,
    #[doc = "0x184 - Publish configuration for event OVRFLW"]
    pub publish_ovrflw: PUBLISH_OVRFLW,
    _reserved13: [u8; 56usize],
    #[doc = "0x1c0 - Description collection: Publish configuration for event COMPARE\\[n\\]"]
    pub publish_compare: [PUBLISH_COMPARE; 4],
    _reserved14: [u8; 308usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved16: [u8; 52usize],
    #[doc = "0x340 - Enable or disable event routing"]
    pub evten: EVTEN,
    #[doc = "0x344 - Enable event routing"]
    pub evtenset: EVTENSET,
    #[doc = "0x348 - Disable event routing"]
    pub evtenclr: EVTENCLR,
    _reserved19: [u8; 440usize],
    #[doc = "0x504 - Current counter value"]
    pub counter: COUNTER,
    #[doc = "0x508 - 12-bit prescaler for counter frequency (32768/(PRESCALER+1)). Must be written when RTC is stopped."]
    pub prescaler: PRESCALER,
    _reserved21: [u8; 52usize],
    #[doc = "0x540 - Description collection: Compare register n"]
    pub cc: [CC; 4],
}
#[doc = "Start RTC counter\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start RTC counter"]
pub mod tasks_start;
#[doc = "Stop RTC counter\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop RTC counter"]
pub mod tasks_stop;
#[doc = "Clear RTC counter\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_clear](tasks_clear) module"]
pub type TASKS_CLEAR = crate::Reg<u32, _TASKS_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CLEAR;
#[doc = "`write(|w| ..)` method takes [tasks_clear::W](tasks_clear::W) writer structure"]
impl crate::Writable for TASKS_CLEAR {}
#[doc = "Clear RTC counter"]
pub mod tasks_clear;
#[doc = "Set counter to 0xFFFFF0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_trigovrflw](tasks_trigovrflw) module"]
pub type TASKS_TRIGOVRFLW = crate::Reg<u32, _TASKS_TRIGOVRFLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_TRIGOVRFLW;
#[doc = "`write(|w| ..)` method takes [tasks_trigovrflw::W](tasks_trigovrflw::W) writer structure"]
impl crate::Writable for TASKS_TRIGOVRFLW {}
#[doc = "Set counter to 0xFFFFF0"]
pub mod tasks_trigovrflw;
#[doc = "Subscribe configuration for task START\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_start](subscribe_start) module"]
pub type SUBSCRIBE_START = crate::Reg<u32, _SUBSCRIBE_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_START;
#[doc = "`read()` method returns [subscribe_start::R](subscribe_start::R) reader structure"]
impl crate::Readable for SUBSCRIBE_START {}
#[doc = "`write(|w| ..)` method takes [subscribe_start::W](subscribe_start::W) writer structure"]
impl crate::Writable for SUBSCRIBE_START {}
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "Subscribe configuration for task STOP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_stop](subscribe_stop) module"]
pub type SUBSCRIBE_STOP = crate::Reg<u32, _SUBSCRIBE_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_STOP;
#[doc = "`read()` method returns [subscribe_stop::R](subscribe_stop::R) reader structure"]
impl crate::Readable for SUBSCRIBE_STOP {}
#[doc = "`write(|w| ..)` method takes [subscribe_stop::W](subscribe_stop::W) writer structure"]
impl crate::Writable for SUBSCRIBE_STOP {}
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "Subscribe configuration for task CLEAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_clear](subscribe_clear) module"]
pub type SUBSCRIBE_CLEAR = crate::Reg<u32, _SUBSCRIBE_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_CLEAR;
#[doc = "`read()` method returns [subscribe_clear::R](subscribe_clear::R) reader structure"]
impl crate::Readable for SUBSCRIBE_CLEAR {}
#[doc = "`write(|w| ..)` method takes [subscribe_clear::W](subscribe_clear::W) writer structure"]
impl crate::Writable for SUBSCRIBE_CLEAR {}
#[doc = "Subscribe configuration for task CLEAR"]
pub mod subscribe_clear;
#[doc = "Subscribe configuration for task TRIGOVRFLW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_trigovrflw](subscribe_trigovrflw) module"]
pub type SUBSCRIBE_TRIGOVRFLW = crate::Reg<u32, _SUBSCRIBE_TRIGOVRFLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_TRIGOVRFLW;
#[doc = "`read()` method returns [subscribe_trigovrflw::R](subscribe_trigovrflw::R) reader structure"]
impl crate::Readable for SUBSCRIBE_TRIGOVRFLW {}
#[doc = "`write(|w| ..)` method takes [subscribe_trigovrflw::W](subscribe_trigovrflw::W) writer structure"]
impl crate::Writable for SUBSCRIBE_TRIGOVRFLW {}
#[doc = "Subscribe configuration for task TRIGOVRFLW"]
pub mod subscribe_trigovrflw;
#[doc = "Event on counter increment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_tick](events_tick) module"]
pub type EVENTS_TICK = crate::Reg<u32, _EVENTS_TICK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TICK;
#[doc = "`read()` method returns [events_tick::R](events_tick::R) reader structure"]
impl crate::Readable for EVENTS_TICK {}
#[doc = "`write(|w| ..)` method takes [events_tick::W](events_tick::W) writer structure"]
impl crate::Writable for EVENTS_TICK {}
#[doc = "Event on counter increment"]
pub mod events_tick;
#[doc = "Event on counter overflow\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ovrflw](events_ovrflw) module"]
pub type EVENTS_OVRFLW = crate::Reg<u32, _EVENTS_OVRFLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_OVRFLW;
#[doc = "`read()` method returns [events_ovrflw::R](events_ovrflw::R) reader structure"]
impl crate::Readable for EVENTS_OVRFLW {}
#[doc = "`write(|w| ..)` method takes [events_ovrflw::W](events_ovrflw::W) writer structure"]
impl crate::Writable for EVENTS_OVRFLW {}
#[doc = "Event on counter overflow"]
pub mod events_ovrflw;
#[doc = "Description collection: Compare event on CC\\[n\\]
match\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_compare](events_compare) module"]
pub type EVENTS_COMPARE = crate::Reg<u32, _EVENTS_COMPARE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_COMPARE;
#[doc = "`read()` method returns [events_compare::R](events_compare::R) reader structure"]
impl crate::Readable for EVENTS_COMPARE {}
#[doc = "`write(|w| ..)` method takes [events_compare::W](events_compare::W) writer structure"]
impl crate::Writable for EVENTS_COMPARE {}
#[doc = "Description collection: Compare event on CC\\[n\\]
match"]
pub mod events_compare;
#[doc = "Publish configuration for event TICK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_tick](publish_tick) module"]
pub type PUBLISH_TICK = crate::Reg<u32, _PUBLISH_TICK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_TICK;
#[doc = "`read()` method returns [publish_tick::R](publish_tick::R) reader structure"]
impl crate::Readable for PUBLISH_TICK {}
#[doc = "`write(|w| ..)` method takes [publish_tick::W](publish_tick::W) writer structure"]
impl crate::Writable for PUBLISH_TICK {}
#[doc = "Publish configuration for event TICK"]
pub mod publish_tick;
#[doc = "Publish configuration for event OVRFLW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_ovrflw](publish_ovrflw) module"]
pub type PUBLISH_OVRFLW = crate::Reg<u32, _PUBLISH_OVRFLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_OVRFLW;
#[doc = "`read()` method returns [publish_ovrflw::R](publish_ovrflw::R) reader structure"]
impl crate::Readable for PUBLISH_OVRFLW {}
#[doc = "`write(|w| ..)` method takes [publish_ovrflw::W](publish_ovrflw::W) writer structure"]
impl crate::Writable for PUBLISH_OVRFLW {}
#[doc = "Publish configuration for event OVRFLW"]
pub mod publish_ovrflw;
#[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_compare](publish_compare) module"]
pub type PUBLISH_COMPARE = crate::Reg<u32, _PUBLISH_COMPARE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_COMPARE;
#[doc = "`read()` method returns [publish_compare::R](publish_compare::R) reader structure"]
impl crate::Readable for PUBLISH_COMPARE {}
#[doc = "`write(|w| ..)` method takes [publish_compare::W](publish_compare::W) writer structure"]
impl crate::Writable for PUBLISH_COMPARE {}
#[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
pub mod publish_compare;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
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
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
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
#[doc = "Enable or disable event routing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evten](evten) module"]
pub type EVTEN = crate::Reg<u32, _EVTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTEN;
#[doc = "`read()` method returns [evten::R](evten::R) reader structure"]
impl crate::Readable for EVTEN {}
#[doc = "`write(|w| ..)` method takes [evten::W](evten::W) writer structure"]
impl crate::Writable for EVTEN {}
#[doc = "Enable or disable event routing"]
pub mod evten;
#[doc = "Enable event routing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtenset](evtenset) module"]
pub type EVTENSET = crate::Reg<u32, _EVTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTENSET;
#[doc = "`read()` method returns [evtenset::R](evtenset::R) reader structure"]
impl crate::Readable for EVTENSET {}
#[doc = "`write(|w| ..)` method takes [evtenset::W](evtenset::W) writer structure"]
impl crate::Writable for EVTENSET {}
#[doc = "Enable event routing"]
pub mod evtenset;
#[doc = "Disable event routing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtenclr](evtenclr) module"]
pub type EVTENCLR = crate::Reg<u32, _EVTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTENCLR;
#[doc = "`read()` method returns [evtenclr::R](evtenclr::R) reader structure"]
impl crate::Readable for EVTENCLR {}
#[doc = "`write(|w| ..)` method takes [evtenclr::W](evtenclr::W) writer structure"]
impl crate::Writable for EVTENCLR {}
#[doc = "Disable event routing"]
pub mod evtenclr;
#[doc = "Current counter value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter](counter) module"]
pub type COUNTER = crate::Reg<u32, _COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNTER;
#[doc = "`read()` method returns [counter::R](counter::R) reader structure"]
impl crate::Readable for COUNTER {}
#[doc = "Current counter value"]
pub mod counter;
#[doc = "12-bit prescaler for counter frequency (32768/(PRESCALER+1)). Must be written when RTC is stopped.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prescaler](prescaler) module"]
pub type PRESCALER = crate::Reg<u32, _PRESCALER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESCALER;
#[doc = "`read()` method returns [prescaler::R](prescaler::R) reader structure"]
impl crate::Readable for PRESCALER {}
#[doc = "`write(|w| ..)` method takes [prescaler::W](prescaler::W) writer structure"]
impl crate::Writable for PRESCALER {}
#[doc = "12-bit prescaler for counter frequency (32768/(PRESCALER+1)). Must be written when RTC is stopped."]
pub mod prescaler;
#[doc = "Description collection: Compare register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "`write(|w| ..)` method takes [cc::W](cc::W) writer structure"]
impl crate::Writable for CC {}
#[doc = "Description collection: Compare register n"]
pub mod cc;

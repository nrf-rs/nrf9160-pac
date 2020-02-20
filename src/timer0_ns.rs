#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start Timer"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop Timer"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Increment Timer (Counter mode only)"]
    pub tasks_count: TASKS_COUNT,
    #[doc = "0x0c - Clear time"]
    pub tasks_clear: TASKS_CLEAR,
    #[doc = "0x10 - Deprecated register - Shut down timer"]
    pub tasks_shutdown: TASKS_SHUTDOWN,
    _reserved5: [u8; 44usize],
    #[doc = "0x40 - Description collection: Capture Timer value to CC\\[n\\]
register"]
    pub tasks_capture: [TASKS_CAPTURE; 6],
    _reserved6: [u8; 40usize],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    #[doc = "0x88 - Subscribe configuration for task COUNT"]
    pub subscribe_count: SUBSCRIBE_COUNT,
    #[doc = "0x8c - Subscribe configuration for task CLEAR"]
    pub subscribe_clear: SUBSCRIBE_CLEAR,
    #[doc = "0x90 - Deprecated register - Subscribe configuration for task SHUTDOWN"]
    pub subscribe_shutdown: SUBSCRIBE_SHUTDOWN,
    _reserved11: [u8; 44usize],
    #[doc = "0xc0 - Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
    pub subscribe_capture: [SUBSCRIBE_CAPTURE; 6],
    _reserved12: [u8; 104usize],
    #[doc = "0x140 - Description collection: Compare event on CC\\[n\\]
match"]
    pub events_compare: [EVENTS_COMPARE; 6],
    _reserved13: [u8; 104usize],
    #[doc = "0x1c0 - Description collection: Publish configuration for event COMPARE\\[n\\]"]
    pub publish_compare: [PUBLISH_COMPARE; 6],
    _reserved14: [u8; 40usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved15: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved17: [u8; 504usize],
    #[doc = "0x504 - Timer mode selection"]
    pub mode: MODE,
    #[doc = "0x508 - Configure the number of bits used by the TIMER"]
    pub bitmode: BITMODE,
    _reserved19: [u8; 4usize],
    #[doc = "0x510 - Timer prescaler register"]
    pub prescaler: PRESCALER,
    #[doc = "0x514 - Description collection: Enable one-shot operation for Capture/Compare channel n"]
    pub oneshoten: [ONESHOTEN; 6],
    _reserved21: [u8; 20usize],
    #[doc = "0x540 - Description collection: Capture/Compare register n"]
    pub cc: [CC; 6],
}
#[doc = "Start Timer\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start Timer"]
pub mod tasks_start;
#[doc = "Stop Timer\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop Timer"]
pub mod tasks_stop;
#[doc = "Increment Timer (Counter mode only)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_count](tasks_count) module"]
pub type TASKS_COUNT = crate::Reg<u32, _TASKS_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_COUNT;
#[doc = "`write(|w| ..)` method takes [tasks_count::W](tasks_count::W) writer structure"]
impl crate::Writable for TASKS_COUNT {}
#[doc = "Increment Timer (Counter mode only)"]
pub mod tasks_count;
#[doc = "Clear time\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_clear](tasks_clear) module"]
pub type TASKS_CLEAR = crate::Reg<u32, _TASKS_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CLEAR;
#[doc = "`write(|w| ..)` method takes [tasks_clear::W](tasks_clear::W) writer structure"]
impl crate::Writable for TASKS_CLEAR {}
#[doc = "Clear time"]
pub mod tasks_clear;
#[doc = "Deprecated register - Shut down timer\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_shutdown](tasks_shutdown) module"]
pub type TASKS_SHUTDOWN = crate::Reg<u32, _TASKS_SHUTDOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SHUTDOWN;
#[doc = "`write(|w| ..)` method takes [tasks_shutdown::W](tasks_shutdown::W) writer structure"]
impl crate::Writable for TASKS_SHUTDOWN {}
#[doc = "Deprecated register - Shut down timer"]
pub mod tasks_shutdown;
#[doc = "Description collection: Capture Timer value to CC\\[n\\]
register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_capture](tasks_capture) module"]
pub type TASKS_CAPTURE = crate::Reg<u32, _TASKS_CAPTURE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CAPTURE;
#[doc = "`write(|w| ..)` method takes [tasks_capture::W](tasks_capture::W) writer structure"]
impl crate::Writable for TASKS_CAPTURE {}
#[doc = "Description collection: Capture Timer value to CC\\[n\\]
register"]
pub mod tasks_capture;
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
#[doc = "Subscribe configuration for task COUNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_count](subscribe_count) module"]
pub type SUBSCRIBE_COUNT = crate::Reg<u32, _SUBSCRIBE_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_COUNT;
#[doc = "`read()` method returns [subscribe_count::R](subscribe_count::R) reader structure"]
impl crate::Readable for SUBSCRIBE_COUNT {}
#[doc = "`write(|w| ..)` method takes [subscribe_count::W](subscribe_count::W) writer structure"]
impl crate::Writable for SUBSCRIBE_COUNT {}
#[doc = "Subscribe configuration for task COUNT"]
pub mod subscribe_count;
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
#[doc = "Deprecated register - Subscribe configuration for task SHUTDOWN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_shutdown](subscribe_shutdown) module"]
pub type SUBSCRIBE_SHUTDOWN = crate::Reg<u32, _SUBSCRIBE_SHUTDOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_SHUTDOWN;
#[doc = "`read()` method returns [subscribe_shutdown::R](subscribe_shutdown::R) reader structure"]
impl crate::Readable for SUBSCRIBE_SHUTDOWN {}
#[doc = "`write(|w| ..)` method takes [subscribe_shutdown::W](subscribe_shutdown::W) writer structure"]
impl crate::Writable for SUBSCRIBE_SHUTDOWN {}
#[doc = "Deprecated register - Subscribe configuration for task SHUTDOWN"]
pub mod subscribe_shutdown;
#[doc = "Description collection: Subscribe configuration for task CAPTURE\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_capture](subscribe_capture) module"]
pub type SUBSCRIBE_CAPTURE = crate::Reg<u32, _SUBSCRIBE_CAPTURE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_CAPTURE;
#[doc = "`read()` method returns [subscribe_capture::R](subscribe_capture::R) reader structure"]
impl crate::Readable for SUBSCRIBE_CAPTURE {}
#[doc = "`write(|w| ..)` method takes [subscribe_capture::W](subscribe_capture::W) writer structure"]
impl crate::Writable for SUBSCRIBE_CAPTURE {}
#[doc = "Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
pub mod subscribe_capture;
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
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
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
#[doc = "Timer mode selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Timer mode selection"]
pub mod mode;
#[doc = "Configure the number of bits used by the TIMER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bitmode](bitmode) module"]
pub type BITMODE = crate::Reg<u32, _BITMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BITMODE;
#[doc = "`read()` method returns [bitmode::R](bitmode::R) reader structure"]
impl crate::Readable for BITMODE {}
#[doc = "`write(|w| ..)` method takes [bitmode::W](bitmode::W) writer structure"]
impl crate::Writable for BITMODE {}
#[doc = "Configure the number of bits used by the TIMER"]
pub mod bitmode;
#[doc = "Timer prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prescaler](prescaler) module"]
pub type PRESCALER = crate::Reg<u32, _PRESCALER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESCALER;
#[doc = "`read()` method returns [prescaler::R](prescaler::R) reader structure"]
impl crate::Readable for PRESCALER {}
#[doc = "`write(|w| ..)` method takes [prescaler::W](prescaler::W) writer structure"]
impl crate::Writable for PRESCALER {}
#[doc = "Timer prescaler register"]
pub mod prescaler;
#[doc = "Description collection: Enable one-shot operation for Capture/Compare channel n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oneshoten](oneshoten) module"]
pub type ONESHOTEN = crate::Reg<u32, _ONESHOTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ONESHOTEN;
#[doc = "`read()` method returns [oneshoten::R](oneshoten::R) reader structure"]
impl crate::Readable for ONESHOTEN {}
#[doc = "`write(|w| ..)` method takes [oneshoten::W](oneshoten::W) writer structure"]
impl crate::Writable for ONESHOTEN {}
#[doc = "Description collection: Enable one-shot operation for Capture/Compare channel n"]
pub mod oneshoten;
#[doc = "Description collection: Capture/Compare register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "`write(|w| ..)` method takes [cc::W](cc::W) writer structure"]
impl crate::Writable for CC {}
#[doc = "Description collection: Capture/Compare register n"]
pub mod cc;

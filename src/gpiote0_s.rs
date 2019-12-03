#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
    pub tasks_out: [TASKS_OUT; 8],
    _reserved1: [u8; 16usize],
    #[doc = "0x30 - Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
    pub tasks_set: [TASKS_SET; 8],
    _reserved2: [u8; 16usize],
    #[doc = "0x60 - Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
    pub tasks_clr: [TASKS_CLR; 8],
    #[doc = "0x80 - Description collection: Subscribe configuration for task OUT\\[n\\]"]
    pub subscribe_out: [SUBSCRIBE_OUT; 8],
    _reserved4: [u8; 16usize],
    #[doc = "0xb0 - Description collection: Subscribe configuration for task SET\\[n\\]"]
    pub subscribe_set: [SUBSCRIBE_SET; 8],
    _reserved5: [u8; 16usize],
    #[doc = "0xe0 - Description collection: Subscribe configuration for task CLR\\[n\\]"]
    pub subscribe_clr: [SUBSCRIBE_CLR; 8],
    #[doc = "0x100 - Description collection: Event generated from pin specified in CONFIG\\[n\\].PSEL"]
    pub events_in: [EVENTS_IN; 8],
    _reserved7: [u8; 92usize],
    #[doc = "0x17c - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
    pub events_port: EVENTS_PORT,
    #[doc = "0x180 - Description collection: Publish configuration for event IN\\[n\\]"]
    pub publish_in: [PUBLISH_IN; 8],
    _reserved9: [u8; 92usize],
    #[doc = "0x1fc - Publish configuration for event PORT"]
    pub publish_port: PUBLISH_PORT,
    _reserved10: [u8; 260usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved12: [u8; 516usize],
    #[doc = "0x510 - Description collection: Configuration for OUT\\[n\\], SET\\[n\\] and CLR\\[n\\] tasks and IN\\[n\\] event"]
    pub config: [CONFIG; 8],
}
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_out](tasks_out) module"]
pub type TASKS_OUT = crate::Reg<u32, _TASKS_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_OUT;
#[doc = "`write(|w| ..)` method takes [tasks_out::W](tasks_out::W) writer structure"]
impl crate::Writable for TASKS_OUT {}
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
pub mod tasks_out;
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_set](tasks_set) module"]
pub type TASKS_SET = crate::Reg<u32, _TASKS_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SET;
#[doc = "`write(|w| ..)` method takes [tasks_set::W](tasks_set::W) writer structure"]
impl crate::Writable for TASKS_SET {}
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
pub mod tasks_set;
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_clr](tasks_clr) module"]
pub type TASKS_CLR = crate::Reg<u32, _TASKS_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CLR;
#[doc = "`write(|w| ..)` method takes [tasks_clr::W](tasks_clr::W) writer structure"]
impl crate::Writable for TASKS_CLR {}
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
pub mod tasks_clr;
#[doc = "Description collection: Subscribe configuration for task OUT\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_out](subscribe_out) module"]
pub type SUBSCRIBE_OUT = crate::Reg<u32, _SUBSCRIBE_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_OUT;
#[doc = "`read()` method returns [subscribe_out::R](subscribe_out::R) reader structure"]
impl crate::Readable for SUBSCRIBE_OUT {}
#[doc = "`write(|w| ..)` method takes [subscribe_out::W](subscribe_out::W) writer structure"]
impl crate::Writable for SUBSCRIBE_OUT {}
#[doc = "Description collection: Subscribe configuration for task OUT\\[n\\]"]
pub mod subscribe_out;
#[doc = "Description collection: Subscribe configuration for task SET\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_set](subscribe_set) module"]
pub type SUBSCRIBE_SET = crate::Reg<u32, _SUBSCRIBE_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_SET;
#[doc = "`read()` method returns [subscribe_set::R](subscribe_set::R) reader structure"]
impl crate::Readable for SUBSCRIBE_SET {}
#[doc = "`write(|w| ..)` method takes [subscribe_set::W](subscribe_set::W) writer structure"]
impl crate::Writable for SUBSCRIBE_SET {}
#[doc = "Description collection: Subscribe configuration for task SET\\[n\\]"]
pub mod subscribe_set;
#[doc = "Description collection: Subscribe configuration for task CLR\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_clr](subscribe_clr) module"]
pub type SUBSCRIBE_CLR = crate::Reg<u32, _SUBSCRIBE_CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_CLR;
#[doc = "`read()` method returns [subscribe_clr::R](subscribe_clr::R) reader structure"]
impl crate::Readable for SUBSCRIBE_CLR {}
#[doc = "`write(|w| ..)` method takes [subscribe_clr::W](subscribe_clr::W) writer structure"]
impl crate::Writable for SUBSCRIBE_CLR {}
#[doc = "Description collection: Subscribe configuration for task CLR\\[n\\]"]
pub mod subscribe_clr;
#[doc = "Description collection: Event generated from pin specified in CONFIG\\[n\\].PSEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_in](events_in) module"]
pub type EVENTS_IN = crate::Reg<u32, _EVENTS_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_IN;
#[doc = "`read()` method returns [events_in::R](events_in::R) reader structure"]
impl crate::Readable for EVENTS_IN {}
#[doc = "`write(|w| ..)` method takes [events_in::W](events_in::W) writer structure"]
impl crate::Writable for EVENTS_IN {}
#[doc = "Description collection: Event generated from pin specified in CONFIG\\[n\\].PSEL"]
pub mod events_in;
#[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_port](events_port) module"]
pub type EVENTS_PORT = crate::Reg<u32, _EVENTS_PORT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_PORT;
#[doc = "`read()` method returns [events_port::R](events_port::R) reader structure"]
impl crate::Readable for EVENTS_PORT {}
#[doc = "`write(|w| ..)` method takes [events_port::W](events_port::W) writer structure"]
impl crate::Writable for EVENTS_PORT {}
#[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
pub mod events_port;
#[doc = "Description collection: Publish configuration for event IN\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_in](publish_in) module"]
pub type PUBLISH_IN = crate::Reg<u32, _PUBLISH_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_IN;
#[doc = "`read()` method returns [publish_in::R](publish_in::R) reader structure"]
impl crate::Readable for PUBLISH_IN {}
#[doc = "`write(|w| ..)` method takes [publish_in::W](publish_in::W) writer structure"]
impl crate::Writable for PUBLISH_IN {}
#[doc = "Description collection: Publish configuration for event IN\\[n\\]"]
pub mod publish_in;
#[doc = "Publish configuration for event PORT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_port](publish_port) module"]
pub type PUBLISH_PORT = crate::Reg<u32, _PUBLISH_PORT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_PORT;
#[doc = "`read()` method returns [publish_port::R](publish_port::R) reader structure"]
impl crate::Readable for PUBLISH_PORT {}
#[doc = "`write(|w| ..)` method takes [publish_port::W](publish_port::W) writer structure"]
impl crate::Writable for PUBLISH_PORT {}
#[doc = "Publish configuration for event PORT"]
pub mod publish_port;
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
#[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\] and CLR\\[n\\] tasks and IN\\[n\\] event\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\] and CLR\\[n\\] tasks and IN\\[n\\] event"]
pub mod config;

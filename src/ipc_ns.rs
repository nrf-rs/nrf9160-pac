#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Description collection: Trigger events on channel enabled in SEND_CNF\\[n\\]."]
    pub tasks_send: [TASKS_SEND; 8],
    _reserved1: [u8; 96usize],
    #[doc = "0x80 - Description collection: Subscribe configuration for task SEND\\[n\\]"]
    pub subscribe_send: [SUBSCRIBE_SEND; 8],
    _reserved2: [u8; 96usize],
    #[doc = "0x100 - Description collection: Event received on one or more of the enabled channels in RECEIVE_CNF\\[n\\]."]
    pub events_receive: [EVENTS_RECEIVE; 8],
    _reserved3: [u8; 96usize],
    #[doc = "0x180 - Description collection: Publish configuration for event RECEIVE\\[n\\]"]
    pub publish_receive: [PUBLISH_RECEIVE; 8],
    _reserved4: [u8; 352usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    #[doc = "0x30c - Pending interrupts"]
    pub intpend: INTPEND,
    _reserved8: [u8; 512usize],
    #[doc = "0x510 - Description collection: Send event configuration for TASKS_SEND\\[n\\]."]
    pub send_cnf: [SEND_CNF; 8],
    _reserved9: [u8; 96usize],
    #[doc = "0x590 - Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]."]
    pub receive_cnf: [RECEIVE_CNF; 8],
    _reserved10: [u8; 96usize],
    #[doc = "0x610 - Description collection: General purpose memory."]
    pub gpmem: [GPMEM; 4],
}
#[doc = "Description collection: Trigger events on channel enabled in SEND_CNF\\[n\\].\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_send](tasks_send) module"]
pub type TASKS_SEND = crate::Reg<u32, _TASKS_SEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SEND;
#[doc = "`write(|w| ..)` method takes [tasks_send::W](tasks_send::W) writer structure"]
impl crate::Writable for TASKS_SEND {}
#[doc = "Description collection: Trigger events on channel enabled in SEND_CNF\\[n\\]."]
pub mod tasks_send;
#[doc = "Description collection: Subscribe configuration for task SEND\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_send](subscribe_send) module"]
pub type SUBSCRIBE_SEND = crate::Reg<u32, _SUBSCRIBE_SEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_SEND;
#[doc = "`read()` method returns [subscribe_send::R](subscribe_send::R) reader structure"]
impl crate::Readable for SUBSCRIBE_SEND {}
#[doc = "`write(|w| ..)` method takes [subscribe_send::W](subscribe_send::W) writer structure"]
impl crate::Writable for SUBSCRIBE_SEND {}
#[doc = "Description collection: Subscribe configuration for task SEND\\[n\\]"]
pub mod subscribe_send;
#[doc = "Description collection: Event received on one or more of the enabled channels in RECEIVE_CNF\\[n\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_receive](events_receive) module"]
pub type EVENTS_RECEIVE = crate::Reg<u32, _EVENTS_RECEIVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RECEIVE;
#[doc = "`read()` method returns [events_receive::R](events_receive::R) reader structure"]
impl crate::Readable for EVENTS_RECEIVE {}
#[doc = "`write(|w| ..)` method takes [events_receive::W](events_receive::W) writer structure"]
impl crate::Writable for EVENTS_RECEIVE {}
#[doc = "Description collection: Event received on one or more of the enabled channels in RECEIVE_CNF\\[n\\]."]
pub mod events_receive;
#[doc = "Description collection: Publish configuration for event RECEIVE\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_receive](publish_receive) module"]
pub type PUBLISH_RECEIVE = crate::Reg<u32, _PUBLISH_RECEIVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_RECEIVE;
#[doc = "`read()` method returns [publish_receive::R](publish_receive::R) reader structure"]
impl crate::Readable for PUBLISH_RECEIVE {}
#[doc = "`write(|w| ..)` method takes [publish_receive::W](publish_receive::W) writer structure"]
impl crate::Writable for PUBLISH_RECEIVE {}
#[doc = "Description collection: Publish configuration for event RECEIVE\\[n\\]"]
pub mod publish_receive;
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
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
#[doc = "Pending interrupts\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intpend](intpend) module"]
pub type INTPEND = crate::Reg<u32, _INTPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTPEND;
#[doc = "`read()` method returns [intpend::R](intpend::R) reader structure"]
impl crate::Readable for INTPEND {}
#[doc = "Pending interrupts"]
pub mod intpend;
#[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [send_cnf](send_cnf) module"]
pub type SEND_CNF = crate::Reg<u32, _SEND_CNF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEND_CNF;
#[doc = "`read()` method returns [send_cnf::R](send_cnf::R) reader structure"]
impl crate::Readable for SEND_CNF {}
#[doc = "`write(|w| ..)` method takes [send_cnf::W](send_cnf::W) writer structure"]
impl crate::Writable for SEND_CNF {}
#[doc = "Description collection: Send event configuration for TASKS_SEND\\[n\\]."]
pub mod send_cnf;
#[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receive_cnf](receive_cnf) module"]
pub type RECEIVE_CNF = crate::Reg<u32, _RECEIVE_CNF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RECEIVE_CNF;
#[doc = "`read()` method returns [receive_cnf::R](receive_cnf::R) reader structure"]
impl crate::Readable for RECEIVE_CNF {}
#[doc = "`write(|w| ..)` method takes [receive_cnf::W](receive_cnf::W) writer structure"]
impl crate::Writable for RECEIVE_CNF {}
#[doc = "Description collection: Receive event configuration for EVENTS_RECEIVE\\[n\\]."]
pub mod receive_cnf;
#[doc = "Description collection: General purpose memory.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpmem](gpmem) module"]
pub type GPMEM = crate::Reg<u32, _GPMEM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPMEM;
#[doc = "`read()` method returns [gpmem::R](gpmem::R) reader structure"]
impl crate::Readable for GPMEM {}
#[doc = "`write(|w| ..)` method takes [gpmem::W](gpmem::W) writer structure"]
impl crate::Writable for GPMEM {}
#[doc = "Description collection: General purpose memory."]
pub mod gpmem;

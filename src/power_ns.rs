#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 120usize],
    #[doc = "0x78 - Enable constant latency mode."]
    pub tasks_constlat: TASKS_CONSTLAT,
    #[doc = "0x7c - Enable low power mode (variable latency)"]
    pub tasks_lowpwr: TASKS_LOWPWR,
    _reserved2: [u8; 120usize],
    #[doc = "0xf8 - Subscribe configuration for task CONSTLAT"]
    pub subscribe_constlat: SUBSCRIBE_CONSTLAT,
    #[doc = "0xfc - Subscribe configuration for task LOWPWR"]
    pub subscribe_lowpwr: SUBSCRIBE_LOWPWR,
    _reserved4: [u8; 8usize],
    #[doc = "0x108 - Power failure warning"]
    pub events_pofwarn: EVENTS_POFWARN,
    _reserved5: [u8; 8usize],
    #[doc = "0x114 - CPU entered WFI/WFE sleep"]
    pub events_sleepenter: EVENTS_SLEEPENTER,
    #[doc = "0x118 - CPU exited WFI/WFE sleep"]
    pub events_sleepexit: EVENTS_SLEEPEXIT,
    _reserved7: [u8; 108usize],
    #[doc = "0x188 - Publish configuration for event POFWARN"]
    pub publish_pofwarn: PUBLISH_POFWARN,
    _reserved8: [u8; 8usize],
    #[doc = "0x194 - Publish configuration for event SLEEPENTER"]
    pub publish_sleepenter: PUBLISH_SLEEPENTER,
    #[doc = "0x198 - Publish configuration for event SLEEPEXIT"]
    pub publish_sleepexit: PUBLISH_SLEEPEXIT,
    _reserved10: [u8; 356usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved13: [u8; 244usize],
    #[doc = "0x400 - Reset reason"]
    pub resetreas: RESETREAS,
    _reserved14: [u8; 60usize],
    #[doc = "0x440 - Modem domain power status"]
    pub powerstatus: POWERSTATUS,
    _reserved15: [u8; 216usize],
    #[doc = "0x51c - Description collection: General purpose retention register"]
    pub gpregret: [GPREGRET; 2],
}
#[doc = "Enable constant latency mode.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_constlat](tasks_constlat) module"]
pub type TASKS_CONSTLAT = crate::Reg<u32, _TASKS_CONSTLAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CONSTLAT;
#[doc = "`write(|w| ..)` method takes [tasks_constlat::W](tasks_constlat::W) writer structure"]
impl crate::Writable for TASKS_CONSTLAT {}
#[doc = "Enable constant latency mode."]
pub mod tasks_constlat;
#[doc = "Enable low power mode (variable latency)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_lowpwr](tasks_lowpwr) module"]
pub type TASKS_LOWPWR = crate::Reg<u32, _TASKS_LOWPWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_LOWPWR;
#[doc = "`write(|w| ..)` method takes [tasks_lowpwr::W](tasks_lowpwr::W) writer structure"]
impl crate::Writable for TASKS_LOWPWR {}
#[doc = "Enable low power mode (variable latency)"]
pub mod tasks_lowpwr;
#[doc = "Subscribe configuration for task CONSTLAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_constlat](subscribe_constlat) module"]
pub type SUBSCRIBE_CONSTLAT = crate::Reg<u32, _SUBSCRIBE_CONSTLAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_CONSTLAT;
#[doc = "`read()` method returns [subscribe_constlat::R](subscribe_constlat::R) reader structure"]
impl crate::Readable for SUBSCRIBE_CONSTLAT {}
#[doc = "`write(|w| ..)` method takes [subscribe_constlat::W](subscribe_constlat::W) writer structure"]
impl crate::Writable for SUBSCRIBE_CONSTLAT {}
#[doc = "Subscribe configuration for task CONSTLAT"]
pub mod subscribe_constlat;
#[doc = "Subscribe configuration for task LOWPWR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_lowpwr](subscribe_lowpwr) module"]
pub type SUBSCRIBE_LOWPWR = crate::Reg<u32, _SUBSCRIBE_LOWPWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_LOWPWR;
#[doc = "`read()` method returns [subscribe_lowpwr::R](subscribe_lowpwr::R) reader structure"]
impl crate::Readable for SUBSCRIBE_LOWPWR {}
#[doc = "`write(|w| ..)` method takes [subscribe_lowpwr::W](subscribe_lowpwr::W) writer structure"]
impl crate::Writable for SUBSCRIBE_LOWPWR {}
#[doc = "Subscribe configuration for task LOWPWR"]
pub mod subscribe_lowpwr;
#[doc = "Power failure warning\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_pofwarn](events_pofwarn) module"]
pub type EVENTS_POFWARN = crate::Reg<u32, _EVENTS_POFWARN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_POFWARN;
#[doc = "`read()` method returns [events_pofwarn::R](events_pofwarn::R) reader structure"]
impl crate::Readable for EVENTS_POFWARN {}
#[doc = "`write(|w| ..)` method takes [events_pofwarn::W](events_pofwarn::W) writer structure"]
impl crate::Writable for EVENTS_POFWARN {}
#[doc = "Power failure warning"]
pub mod events_pofwarn;
#[doc = "CPU entered WFI/WFE sleep\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_sleepenter](events_sleepenter) module"]
pub type EVENTS_SLEEPENTER = crate::Reg<u32, _EVENTS_SLEEPENTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SLEEPENTER;
#[doc = "`read()` method returns [events_sleepenter::R](events_sleepenter::R) reader structure"]
impl crate::Readable for EVENTS_SLEEPENTER {}
#[doc = "`write(|w| ..)` method takes [events_sleepenter::W](events_sleepenter::W) writer structure"]
impl crate::Writable for EVENTS_SLEEPENTER {}
#[doc = "CPU entered WFI/WFE sleep"]
pub mod events_sleepenter;
#[doc = "CPU exited WFI/WFE sleep\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_sleepexit](events_sleepexit) module"]
pub type EVENTS_SLEEPEXIT = crate::Reg<u32, _EVENTS_SLEEPEXIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SLEEPEXIT;
#[doc = "`read()` method returns [events_sleepexit::R](events_sleepexit::R) reader structure"]
impl crate::Readable for EVENTS_SLEEPEXIT {}
#[doc = "`write(|w| ..)` method takes [events_sleepexit::W](events_sleepexit::W) writer structure"]
impl crate::Writable for EVENTS_SLEEPEXIT {}
#[doc = "CPU exited WFI/WFE sleep"]
pub mod events_sleepexit;
#[doc = "Publish configuration for event POFWARN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_pofwarn](publish_pofwarn) module"]
pub type PUBLISH_POFWARN = crate::Reg<u32, _PUBLISH_POFWARN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_POFWARN;
#[doc = "`read()` method returns [publish_pofwarn::R](publish_pofwarn::R) reader structure"]
impl crate::Readable for PUBLISH_POFWARN {}
#[doc = "`write(|w| ..)` method takes [publish_pofwarn::W](publish_pofwarn::W) writer structure"]
impl crate::Writable for PUBLISH_POFWARN {}
#[doc = "Publish configuration for event POFWARN"]
pub mod publish_pofwarn;
#[doc = "Publish configuration for event SLEEPENTER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_sleepenter](publish_sleepenter) module"]
pub type PUBLISH_SLEEPENTER = crate::Reg<u32, _PUBLISH_SLEEPENTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_SLEEPENTER;
#[doc = "`read()` method returns [publish_sleepenter::R](publish_sleepenter::R) reader structure"]
impl crate::Readable for PUBLISH_SLEEPENTER {}
#[doc = "`write(|w| ..)` method takes [publish_sleepenter::W](publish_sleepenter::W) writer structure"]
impl crate::Writable for PUBLISH_SLEEPENTER {}
#[doc = "Publish configuration for event SLEEPENTER"]
pub mod publish_sleepenter;
#[doc = "Publish configuration for event SLEEPEXIT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_sleepexit](publish_sleepexit) module"]
pub type PUBLISH_SLEEPEXIT = crate::Reg<u32, _PUBLISH_SLEEPEXIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_SLEEPEXIT;
#[doc = "`read()` method returns [publish_sleepexit::R](publish_sleepexit::R) reader structure"]
impl crate::Readable for PUBLISH_SLEEPEXIT {}
#[doc = "`write(|w| ..)` method takes [publish_sleepexit::W](publish_sleepexit::W) writer structure"]
impl crate::Writable for PUBLISH_SLEEPEXIT {}
#[doc = "Publish configuration for event SLEEPEXIT"]
pub mod publish_sleepexit;
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
#[doc = "Reset reason\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resetreas](resetreas) module"]
pub type RESETREAS = crate::Reg<u32, _RESETREAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETREAS;
#[doc = "`read()` method returns [resetreas::R](resetreas::R) reader structure"]
impl crate::Readable for RESETREAS {}
#[doc = "`write(|w| ..)` method takes [resetreas::W](resetreas::W) writer structure"]
impl crate::Writable for RESETREAS {}
#[doc = "Reset reason"]
pub mod resetreas;
#[doc = "Modem domain power status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [powerstatus](powerstatus) module"]
pub type POWERSTATUS = crate::Reg<u32, _POWERSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWERSTATUS;
#[doc = "`read()` method returns [powerstatus::R](powerstatus::R) reader structure"]
impl crate::Readable for POWERSTATUS {}
#[doc = "Modem domain power status"]
pub mod powerstatus;
#[doc = "Description collection: General purpose retention register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpregret](gpregret) module"]
pub type GPREGRET = crate::Reg<u32, _GPREGRET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPREGRET;
#[doc = "`read()` method returns [gpregret::R](gpregret::R) reader structure"]
impl crate::Readable for GPREGRET {}
#[doc = "`write(|w| ..)` method takes [gpregret::W](gpregret::W) writer structure"]
impl crate::Writable for GPREGRET {}
#[doc = "Description collection: General purpose retention register"]
pub mod gpregret;

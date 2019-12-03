#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Push a key slot over secure APB"]
    pub tasks_push_keyslot: TASKS_PUSH_KEYSLOT,
    _reserved1: [u8; 252usize],
    #[doc = "0x100 - Key successfully pushed over secure APB"]
    pub events_keyslot_pushed: EVENTS_KEYSLOT_PUSHED,
    #[doc = "0x104 - Key has been revoked and cannot be tasked for selection"]
    pub events_keyslot_revoked: EVENTS_KEYSLOT_REVOKED,
    #[doc = "0x108 - No key slot selected, no destination address defined, or error during push operation"]
    pub events_keyslot_error: EVENTS_KEYSLOT_ERROR,
    _reserved4: [u8; 500usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    #[doc = "0x30c - Pending interrupts"]
    pub intpend: INTPEND,
    _reserved8: [u8; 252usize],
    #[doc = "0x40c - Status bits for KMU operation"]
    pub status: STATUS,
    _reserved9: [u8; 240usize],
    #[doc = "0x500 - Select key slot ID to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started"]
    pub selectkeyslot: SELECTKEYSLOT,
}
#[doc = "Push a key slot over secure APB\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_push_keyslot](tasks_push_keyslot) module"]
pub type TASKS_PUSH_KEYSLOT = crate::Reg<u32, _TASKS_PUSH_KEYSLOT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_PUSH_KEYSLOT;
#[doc = "`write(|w| ..)` method takes [tasks_push_keyslot::W](tasks_push_keyslot::W) writer structure"]
impl crate::Writable for TASKS_PUSH_KEYSLOT {}
#[doc = "Push a key slot over secure APB"]
pub mod tasks_push_keyslot;
#[doc = "Key successfully pushed over secure APB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_keyslot_pushed](events_keyslot_pushed) module"]
pub type EVENTS_KEYSLOT_PUSHED = crate::Reg<u32, _EVENTS_KEYSLOT_PUSHED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_KEYSLOT_PUSHED;
#[doc = "`read()` method returns [events_keyslot_pushed::R](events_keyslot_pushed::R) reader structure"]
impl crate::Readable for EVENTS_KEYSLOT_PUSHED {}
#[doc = "`write(|w| ..)` method takes [events_keyslot_pushed::W](events_keyslot_pushed::W) writer structure"]
impl crate::Writable for EVENTS_KEYSLOT_PUSHED {}
#[doc = "Key successfully pushed over secure APB"]
pub mod events_keyslot_pushed;
#[doc = "Key has been revoked and cannot be tasked for selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_keyslot_revoked](events_keyslot_revoked) module"]
pub type EVENTS_KEYSLOT_REVOKED = crate::Reg<u32, _EVENTS_KEYSLOT_REVOKED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_KEYSLOT_REVOKED;
#[doc = "`read()` method returns [events_keyslot_revoked::R](events_keyslot_revoked::R) reader structure"]
impl crate::Readable for EVENTS_KEYSLOT_REVOKED {}
#[doc = "`write(|w| ..)` method takes [events_keyslot_revoked::W](events_keyslot_revoked::W) writer structure"]
impl crate::Writable for EVENTS_KEYSLOT_REVOKED {}
#[doc = "Key has been revoked and cannot be tasked for selection"]
pub mod events_keyslot_revoked;
#[doc = "No key slot selected, no destination address defined, or error during push operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_keyslot_error](events_keyslot_error) module"]
pub type EVENTS_KEYSLOT_ERROR = crate::Reg<u32, _EVENTS_KEYSLOT_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_KEYSLOT_ERROR;
#[doc = "`read()` method returns [events_keyslot_error::R](events_keyslot_error::R) reader structure"]
impl crate::Readable for EVENTS_KEYSLOT_ERROR {}
#[doc = "`write(|w| ..)` method takes [events_keyslot_error::W](events_keyslot_error::W) writer structure"]
impl crate::Writable for EVENTS_KEYSLOT_ERROR {}
#[doc = "No key slot selected, no destination address defined, or error during push operation"]
pub mod events_keyslot_error;
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
#[doc = "Status bits for KMU operation\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status bits for KMU operation"]
pub mod status;
#[doc = "Select key slot ID to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [selectkeyslot](selectkeyslot) module"]
pub type SELECTKEYSLOT = crate::Reg<u32, _SELECTKEYSLOT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SELECTKEYSLOT;
#[doc = "`read()` method returns [selectkeyslot::R](selectkeyslot::R) reader structure"]
impl crate::Readable for SELECTKEYSLOT {}
#[doc = "`write(|w| ..)` method takes [selectkeyslot::W](selectkeyslot::W) writer structure"]
impl crate::Writable for SELECTKEYSLOT {}
#[doc = "Select key slot ID to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started"]
pub mod selectkeyslot;

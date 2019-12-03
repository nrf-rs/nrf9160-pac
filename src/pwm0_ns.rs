#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
    pub tasks_seqstart: [TASKS_SEQSTART; 2],
    #[doc = "0x10 - Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
    pub tasks_nextstep: TASKS_NEXTSTEP,
    _reserved3: [u8; 112usize],
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    #[doc = "0x88 - Description collection: Subscribe configuration for task SEQSTART\\[n\\]"]
    pub subscribe_seqstart: [SUBSCRIBE_SEQSTART; 2],
    #[doc = "0x90 - Subscribe configuration for task NEXTSTEP"]
    pub subscribe_nextstep: SUBSCRIBE_NEXTSTEP,
    _reserved6: [u8; 112usize],
    #[doc = "0x104 - Response to STOP task, emitted when PWM pulses are no longer generated"]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x108 - Description collection: First PWM period started on sequence n"]
    pub events_seqstarted: [EVENTS_SEQSTARTED; 2],
    #[doc = "0x110 - Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
    pub events_seqend: [EVENTS_SEQEND; 2],
    #[doc = "0x118 - Emitted at the end of each PWM period"]
    pub events_pwmperiodend: EVENTS_PWMPERIODEND,
    #[doc = "0x11c - Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
    pub events_loopsdone: EVENTS_LOOPSDONE,
    _reserved11: [u8; 100usize],
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    pub publish_stopped: PUBLISH_STOPPED,
    #[doc = "0x188 - Description collection: Publish configuration for event SEQSTARTED\\[n\\]"]
    pub publish_seqstarted: [PUBLISH_SEQSTARTED; 2],
    #[doc = "0x190 - Description collection: Publish configuration for event SEQEND\\[n\\]"]
    pub publish_seqend: [PUBLISH_SEQEND; 2],
    #[doc = "0x198 - Publish configuration for event PWMPERIODEND"]
    pub publish_pwmperiodend: PUBLISH_PWMPERIODEND,
    #[doc = "0x19c - Publish configuration for event LOOPSDONE"]
    pub publish_loopsdone: PUBLISH_LOOPSDONE,
    _reserved16: [u8; 96usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved17: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved20: [u8; 500usize],
    #[doc = "0x500 - PWM module enable register"]
    pub enable: ENABLE,
    #[doc = "0x504 - Selects operating mode of the wave counter"]
    pub mode: MODE,
    #[doc = "0x508 - Value up to which the pulse generator counter counts"]
    pub countertop: COUNTERTOP,
    #[doc = "0x50c - Configuration for PWM_CLK"]
    pub prescaler: PRESCALER,
    #[doc = "0x510 - Configuration of the decoder"]
    pub decoder: DECODER,
    #[doc = "0x514 - Number of playbacks of a loop"]
    pub loop_: LOOP,
    _reserved26: [u8; 8usize],
    #[doc = "0x520 - Unspecified"]
    pub seq0: SEQ,
    _reserved27: [u8; 16usize],
    #[doc = "0x540 - Unspecified"]
    pub seq1: SEQ,
    _reserved28: [u8; 16usize],
    #[doc = "0x560 - Unspecified"]
    pub psel: PSEL,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SEQ {
    #[doc = "0x00 - Description cluster: Beginning address in RAM of this sequence"]
    pub ptr: self::seq::PTR,
    #[doc = "0x04 - Description cluster: Number of values (duty cycles) in this sequence"]
    pub cnt: self::seq::CNT,
    #[doc = "0x08 - Description cluster: Number of additional PWM periods between samples loaded into compare register"]
    pub refresh: self::seq::REFRESH,
    #[doc = "0x0c - Description cluster: Time added after the sequence"]
    pub enddelay: self::seq::ENDDELAY,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod seq;
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Description collection: Output pin select for PWM channel n"]
    pub out: [self::psel::OUT; 4],
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
pub mod tasks_stop;
#[doc = "Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_seqstart](tasks_seqstart) module"]
pub type TASKS_SEQSTART = crate::Reg<u32, _TASKS_SEQSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SEQSTART;
#[doc = "`write(|w| ..)` method takes [tasks_seqstart::W](tasks_seqstart::W) writer structure"]
impl crate::Writable for TASKS_SEQSTART {}
#[doc = "Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
pub mod tasks_seqstart;
#[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_nextstep](tasks_nextstep) module"]
pub type TASKS_NEXTSTEP = crate::Reg<u32, _TASKS_NEXTSTEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_NEXTSTEP;
#[doc = "`write(|w| ..)` method takes [tasks_nextstep::W](tasks_nextstep::W) writer structure"]
impl crate::Writable for TASKS_NEXTSTEP {}
#[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
pub mod tasks_nextstep;
#[doc = "Subscribe configuration for task STOP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_stop](subscribe_stop) module"]
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
#[doc = "Description collection: Subscribe configuration for task SEQSTART\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_seqstart](subscribe_seqstart) module"]
pub type SUBSCRIBE_SEQSTART = crate::Reg<u32, _SUBSCRIBE_SEQSTART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_SEQSTART;
#[doc = "`read()` method returns [subscribe_seqstart::R](subscribe_seqstart::R) reader structure"]
impl crate::Readable for SUBSCRIBE_SEQSTART {}
#[doc = "`write(|w| ..)` method takes [subscribe_seqstart::W](subscribe_seqstart::W) writer structure"]
impl crate::Writable for SUBSCRIBE_SEQSTART {}
#[doc = "Description collection: Subscribe configuration for task SEQSTART\\[n\\]"]
pub mod subscribe_seqstart;
#[doc = "Subscribe configuration for task NEXTSTEP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_nextstep](subscribe_nextstep) module"]
pub type SUBSCRIBE_NEXTSTEP = crate::Reg<u32, _SUBSCRIBE_NEXTSTEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_NEXTSTEP;
#[doc = "`read()` method returns [subscribe_nextstep::R](subscribe_nextstep::R) reader structure"]
impl crate::Readable for SUBSCRIBE_NEXTSTEP {}
#[doc = "`write(|w| ..)` method takes [subscribe_nextstep::W](subscribe_nextstep::W) writer structure"]
impl crate::Writable for SUBSCRIBE_NEXTSTEP {}
#[doc = "Subscribe configuration for task NEXTSTEP"]
pub mod subscribe_nextstep;
#[doc = "Response to STOP task, emitted when PWM pulses are no longer generated\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_stopped](events_stopped) module"]
pub type EVENTS_STOPPED = crate::Reg<u32, _EVENTS_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STOPPED;
#[doc = "`read()` method returns [events_stopped::R](events_stopped::R) reader structure"]
impl crate::Readable for EVENTS_STOPPED {}
#[doc = "`write(|w| ..)` method takes [events_stopped::W](events_stopped::W) writer structure"]
impl crate::Writable for EVENTS_STOPPED {}
#[doc = "Response to STOP task, emitted when PWM pulses are no longer generated"]
pub mod events_stopped;
#[doc = "Description collection: First PWM period started on sequence n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_seqstarted](events_seqstarted) module"]
pub type EVENTS_SEQSTARTED = crate::Reg<u32, _EVENTS_SEQSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SEQSTARTED;
#[doc = "`read()` method returns [events_seqstarted::R](events_seqstarted::R) reader structure"]
impl crate::Readable for EVENTS_SEQSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_seqstarted::W](events_seqstarted::W) writer structure"]
impl crate::Writable for EVENTS_SEQSTARTED {}
#[doc = "Description collection: First PWM period started on sequence n"]
pub mod events_seqstarted;
#[doc = "Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_seqend](events_seqend) module"]
pub type EVENTS_SEQEND = crate::Reg<u32, _EVENTS_SEQEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SEQEND;
#[doc = "`read()` method returns [events_seqend::R](events_seqend::R) reader structure"]
impl crate::Readable for EVENTS_SEQEND {}
#[doc = "`write(|w| ..)` method takes [events_seqend::W](events_seqend::W) writer structure"]
impl crate::Writable for EVENTS_SEQEND {}
#[doc = "Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
pub mod events_seqend;
#[doc = "Emitted at the end of each PWM period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_pwmperiodend](events_pwmperiodend) module"]
pub type EVENTS_PWMPERIODEND = crate::Reg<u32, _EVENTS_PWMPERIODEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_PWMPERIODEND;
#[doc = "`read()` method returns [events_pwmperiodend::R](events_pwmperiodend::R) reader structure"]
impl crate::Readable for EVENTS_PWMPERIODEND {}
#[doc = "`write(|w| ..)` method takes [events_pwmperiodend::W](events_pwmperiodend::W) writer structure"]
impl crate::Writable for EVENTS_PWMPERIODEND {}
#[doc = "Emitted at the end of each PWM period"]
pub mod events_pwmperiodend;
#[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_loopsdone](events_loopsdone) module"]
pub type EVENTS_LOOPSDONE = crate::Reg<u32, _EVENTS_LOOPSDONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_LOOPSDONE;
#[doc = "`read()` method returns [events_loopsdone::R](events_loopsdone::R) reader structure"]
impl crate::Readable for EVENTS_LOOPSDONE {}
#[doc = "`write(|w| ..)` method takes [events_loopsdone::W](events_loopsdone::W) writer structure"]
impl crate::Writable for EVENTS_LOOPSDONE {}
#[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
pub mod events_loopsdone;
#[doc = "Publish configuration for event STOPPED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_stopped](publish_stopped) module"]
pub type PUBLISH_STOPPED = crate::Reg<u32, _PUBLISH_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_STOPPED;
#[doc = "`read()` method returns [publish_stopped::R](publish_stopped::R) reader structure"]
impl crate::Readable for PUBLISH_STOPPED {}
#[doc = "`write(|w| ..)` method takes [publish_stopped::W](publish_stopped::W) writer structure"]
impl crate::Writable for PUBLISH_STOPPED {}
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "Description collection: Publish configuration for event SEQSTARTED\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_seqstarted](publish_seqstarted) module"]
pub type PUBLISH_SEQSTARTED = crate::Reg<u32, _PUBLISH_SEQSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_SEQSTARTED;
#[doc = "`read()` method returns [publish_seqstarted::R](publish_seqstarted::R) reader structure"]
impl crate::Readable for PUBLISH_SEQSTARTED {}
#[doc = "`write(|w| ..)` method takes [publish_seqstarted::W](publish_seqstarted::W) writer structure"]
impl crate::Writable for PUBLISH_SEQSTARTED {}
#[doc = "Description collection: Publish configuration for event SEQSTARTED\\[n\\]"]
pub mod publish_seqstarted;
#[doc = "Description collection: Publish configuration for event SEQEND\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_seqend](publish_seqend) module"]
pub type PUBLISH_SEQEND = crate::Reg<u32, _PUBLISH_SEQEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_SEQEND;
#[doc = "`read()` method returns [publish_seqend::R](publish_seqend::R) reader structure"]
impl crate::Readable for PUBLISH_SEQEND {}
#[doc = "`write(|w| ..)` method takes [publish_seqend::W](publish_seqend::W) writer structure"]
impl crate::Writable for PUBLISH_SEQEND {}
#[doc = "Description collection: Publish configuration for event SEQEND\\[n\\]"]
pub mod publish_seqend;
#[doc = "Publish configuration for event PWMPERIODEND\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_pwmperiodend](publish_pwmperiodend) module"]
pub type PUBLISH_PWMPERIODEND = crate::Reg<u32, _PUBLISH_PWMPERIODEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_PWMPERIODEND;
#[doc = "`read()` method returns [publish_pwmperiodend::R](publish_pwmperiodend::R) reader structure"]
impl crate::Readable for PUBLISH_PWMPERIODEND {}
#[doc = "`write(|w| ..)` method takes [publish_pwmperiodend::W](publish_pwmperiodend::W) writer structure"]
impl crate::Writable for PUBLISH_PWMPERIODEND {}
#[doc = "Publish configuration for event PWMPERIODEND"]
pub mod publish_pwmperiodend;
#[doc = "Publish configuration for event LOOPSDONE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_loopsdone](publish_loopsdone) module"]
pub type PUBLISH_LOOPSDONE = crate::Reg<u32, _PUBLISH_LOOPSDONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_LOOPSDONE;
#[doc = "`read()` method returns [publish_loopsdone::R](publish_loopsdone::R) reader structure"]
impl crate::Readable for PUBLISH_LOOPSDONE {}
#[doc = "`write(|w| ..)` method takes [publish_loopsdone::W](publish_loopsdone::W) writer structure"]
impl crate::Writable for PUBLISH_LOOPSDONE {}
#[doc = "Publish configuration for event LOOPSDONE"]
pub mod publish_loopsdone;
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shorts](shorts) module"]
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
#[doc = "PWM module enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "PWM module enable register"]
pub mod enable;
#[doc = "Selects operating mode of the wave counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Selects operating mode of the wave counter"]
pub mod mode;
#[doc = "Value up to which the pulse generator counter counts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [countertop](countertop) module"]
pub type COUNTERTOP = crate::Reg<u32, _COUNTERTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNTERTOP;
#[doc = "`read()` method returns [countertop::R](countertop::R) reader structure"]
impl crate::Readable for COUNTERTOP {}
#[doc = "`write(|w| ..)` method takes [countertop::W](countertop::W) writer structure"]
impl crate::Writable for COUNTERTOP {}
#[doc = "Value up to which the pulse generator counter counts"]
pub mod countertop;
#[doc = "Configuration for PWM_CLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prescaler](prescaler) module"]
pub type PRESCALER = crate::Reg<u32, _PRESCALER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESCALER;
#[doc = "`read()` method returns [prescaler::R](prescaler::R) reader structure"]
impl crate::Readable for PRESCALER {}
#[doc = "`write(|w| ..)` method takes [prescaler::W](prescaler::W) writer structure"]
impl crate::Writable for PRESCALER {}
#[doc = "Configuration for PWM_CLK"]
pub mod prescaler;
#[doc = "Configuration of the decoder\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [decoder](decoder) module"]
pub type DECODER = crate::Reg<u32, _DECODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DECODER;
#[doc = "`read()` method returns [decoder::R](decoder::R) reader structure"]
impl crate::Readable for DECODER {}
#[doc = "`write(|w| ..)` method takes [decoder::W](decoder::W) writer structure"]
impl crate::Writable for DECODER {}
#[doc = "Configuration of the decoder"]
pub mod decoder;
#[doc = "Number of playbacks of a loop\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [loop_](loop_) module"]
pub type LOOP = crate::Reg<u32, _LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOOP;
#[doc = "`read()` method returns [loop_::R](loop_::R) reader structure"]
impl crate::Readable for LOOP {}
#[doc = "`write(|w| ..)` method takes [loop_::W](loop_::W) writer structure"]
impl crate::Writable for LOOP {}
#[doc = "Number of playbacks of a loop"]
pub mod loop_;

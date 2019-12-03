#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the ADC and prepare the result buffer in RAM"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Take one ADC sample, if scan is enabled all channels are sampled"]
    pub tasks_sample: TASKS_SAMPLE,
    #[doc = "0x08 - Stop the ADC and terminate any on-going conversion"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x0c - Starts offset auto-calibration"]
    pub tasks_calibrateoffset: TASKS_CALIBRATEOFFSET,
    _reserved4: [u8; 112usize],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x84 - Subscribe configuration for task SAMPLE"]
    pub subscribe_sample: SUBSCRIBE_SAMPLE,
    #[doc = "0x88 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    #[doc = "0x8c - Subscribe configuration for task CALIBRATEOFFSET"]
    pub subscribe_calibrateoffset: SUBSCRIBE_CALIBRATEOFFSET,
    _reserved8: [u8; 112usize],
    #[doc = "0x100 - The ADC has started"]
    pub events_started: EVENTS_STARTED,
    #[doc = "0x104 - The ADC has filled up the Result buffer"]
    pub events_end: EVENTS_END,
    #[doc = "0x108 - A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
    pub events_done: EVENTS_DONE,
    #[doc = "0x10c - A result is ready to get transferred to RAM."]
    pub events_resultdone: EVENTS_RESULTDONE,
    #[doc = "0x110 - Calibration is complete"]
    pub events_calibratedone: EVENTS_CALIBRATEDONE,
    #[doc = "0x114 - The ADC has stopped"]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x118 - Peripheral events."]
    pub events_ch: [EVENTS_CH; 8],
    _reserved15: [u8; 40usize],
    #[doc = "0x180 - Publish configuration for event STARTED"]
    pub publish_started: PUBLISH_STARTED,
    #[doc = "0x184 - Publish configuration for event END"]
    pub publish_end: PUBLISH_END,
    #[doc = "0x188 - Publish configuration for event DONE"]
    pub publish_done: PUBLISH_DONE,
    #[doc = "0x18c - Publish configuration for event RESULTDONE"]
    pub publish_resultdone: PUBLISH_RESULTDONE,
    #[doc = "0x190 - Publish configuration for event CALIBRATEDONE"]
    pub publish_calibratedone: PUBLISH_CALIBRATEDONE,
    #[doc = "0x194 - Publish configuration for event STOPPED"]
    pub publish_stopped: PUBLISH_STOPPED,
    #[doc = "0x198 - Publish configuration for events"]
    pub publish_ch: [PUBLISH_CH; 8],
    _reserved22: [u8; 296usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved25: [u8; 244usize],
    #[doc = "0x400 - Status"]
    pub status: STATUS,
    _reserved26: [u8; 252usize],
    #[doc = "0x500 - Enable or disable ADC"]
    pub enable: ENABLE,
    _reserved27: [u8; 12usize],
    #[doc = "0x510 - Unspecified"]
    pub ch: [CH; 8],
    _reserved28: [u8; 96usize],
    #[doc = "0x5f0 - Resolution configuration"]
    pub resolution: RESOLUTION,
    #[doc = "0x5f4 - Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
    pub oversample: OVERSAMPLE,
    #[doc = "0x5f8 - Controls normal or continuous sample rate"]
    pub samplerate: SAMPLERATE,
    _reserved31: [u8; 48usize],
    #[doc = "0x62c - RESULT EasyDMA channel"]
    pub result: RESULT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct EVENTS_CH {
    #[doc = "0x00 - Description cluster: Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
    pub limith: self::events_ch::LIMITH,
    #[doc = "0x04 - Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW"]
    pub limitl: self::events_ch::LIMITL,
}
#[doc = r"Register block"]
#[doc = "Peripheral events."]
pub mod events_ch;
#[doc = r"Register block"]
#[repr(C)]
pub struct PUBLISH_CH {
    #[doc = "0x00 - Description cluster: Publish configuration for event CH\\[n\\].LIMITH"]
    pub limith: self::publish_ch::LIMITH,
    #[doc = "0x04 - Description cluster: Publish configuration for event CH\\[n\\].LIMITL"]
    pub limitl: self::publish_ch::LIMITL,
}
#[doc = r"Register block"]
#[doc = "Publish configuration for events"]
pub mod publish_ch;
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Description cluster: Input positive pin selection for CH\\[n\\]"]
    pub pselp: self::ch::PSELP,
    #[doc = "0x04 - Description cluster: Input negative pin selection for CH\\[n\\]"]
    pub pseln: self::ch::PSELN,
    #[doc = "0x08 - Description cluster: Input configuration for CH\\[n\\]"]
    pub config: self::ch::CONFIG,
    #[doc = "0x0c - Description cluster: High/low limits for event monitoring a channel"]
    pub limit: self::ch::LIMIT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod ch;
#[doc = r"Register block"]
#[repr(C)]
pub struct RESULT {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::result::PTR,
    #[doc = "0x04 - Maximum number of buffer words to transfer"]
    pub maxcnt: self::result::MAXCNT,
    #[doc = "0x08 - Number of buffer words transferred since last START"]
    pub amount: self::result::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "RESULT EasyDMA channel"]
pub mod result;
#[doc = "Start the ADC and prepare the result buffer in RAM\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start the ADC and prepare the result buffer in RAM"]
pub mod tasks_start;
#[doc = "Take one ADC sample, if scan is enabled all channels are sampled\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_sample](tasks_sample) module"]
pub type TASKS_SAMPLE = crate::Reg<u32, _TASKS_SAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SAMPLE;
#[doc = "`write(|w| ..)` method takes [tasks_sample::W](tasks_sample::W) writer structure"]
impl crate::Writable for TASKS_SAMPLE {}
#[doc = "Take one ADC sample, if scan is enabled all channels are sampled"]
pub mod tasks_sample;
#[doc = "Stop the ADC and terminate any on-going conversion\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop the ADC and terminate any on-going conversion"]
pub mod tasks_stop;
#[doc = "Starts offset auto-calibration\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_calibrateoffset](tasks_calibrateoffset) module"]
pub type TASKS_CALIBRATEOFFSET = crate::Reg<u32, _TASKS_CALIBRATEOFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CALIBRATEOFFSET;
#[doc = "`write(|w| ..)` method takes [tasks_calibrateoffset::W](tasks_calibrateoffset::W) writer structure"]
impl crate::Writable for TASKS_CALIBRATEOFFSET {}
#[doc = "Starts offset auto-calibration"]
pub mod tasks_calibrateoffset;
#[doc = "Subscribe configuration for task START\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_start](subscribe_start) module"]
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
#[doc = "Subscribe configuration for task SAMPLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_sample](subscribe_sample) module"]
pub type SUBSCRIBE_SAMPLE = crate::Reg<u32, _SUBSCRIBE_SAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_SAMPLE;
#[doc = "`read()` method returns [subscribe_sample::R](subscribe_sample::R) reader structure"]
impl crate::Readable for SUBSCRIBE_SAMPLE {}
#[doc = "`write(|w| ..)` method takes [subscribe_sample::W](subscribe_sample::W) writer structure"]
impl crate::Writable for SUBSCRIBE_SAMPLE {}
#[doc = "Subscribe configuration for task SAMPLE"]
pub mod subscribe_sample;
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
#[doc = "Subscribe configuration for task CALIBRATEOFFSET\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_calibrateoffset](subscribe_calibrateoffset) module"]
pub type SUBSCRIBE_CALIBRATEOFFSET = crate::Reg<u32, _SUBSCRIBE_CALIBRATEOFFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_CALIBRATEOFFSET;
#[doc = "`read()` method returns [subscribe_calibrateoffset::R](subscribe_calibrateoffset::R) reader structure"]
impl crate::Readable for SUBSCRIBE_CALIBRATEOFFSET {}
#[doc = "`write(|w| ..)` method takes [subscribe_calibrateoffset::W](subscribe_calibrateoffset::W) writer structure"]
impl crate::Writable for SUBSCRIBE_CALIBRATEOFFSET {}
#[doc = "Subscribe configuration for task CALIBRATEOFFSET"]
pub mod subscribe_calibrateoffset;
#[doc = "The ADC has started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_started](events_started) module"]
pub type EVENTS_STARTED = crate::Reg<u32, _EVENTS_STARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STARTED;
#[doc = "`read()` method returns [events_started::R](events_started::R) reader structure"]
impl crate::Readable for EVENTS_STARTED {}
#[doc = "`write(|w| ..)` method takes [events_started::W](events_started::W) writer structure"]
impl crate::Writable for EVENTS_STARTED {}
#[doc = "The ADC has started"]
pub mod events_started;
#[doc = "The ADC has filled up the Result buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_end](events_end) module"]
pub type EVENTS_END = crate::Reg<u32, _EVENTS_END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_END;
#[doc = "`read()` method returns [events_end::R](events_end::R) reader structure"]
impl crate::Readable for EVENTS_END {}
#[doc = "`write(|w| ..)` method takes [events_end::W](events_end::W) writer structure"]
impl crate::Writable for EVENTS_END {}
#[doc = "The ADC has filled up the Result buffer"]
pub mod events_end;
#[doc = "A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_done](events_done) module"]
pub type EVENTS_DONE = crate::Reg<u32, _EVENTS_DONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_DONE;
#[doc = "`read()` method returns [events_done::R](events_done::R) reader structure"]
impl crate::Readable for EVENTS_DONE {}
#[doc = "`write(|w| ..)` method takes [events_done::W](events_done::W) writer structure"]
impl crate::Writable for EVENTS_DONE {}
#[doc = "A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
pub mod events_done;
#[doc = "A result is ready to get transferred to RAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_resultdone](events_resultdone) module"]
pub type EVENTS_RESULTDONE = crate::Reg<u32, _EVENTS_RESULTDONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RESULTDONE;
#[doc = "`read()` method returns [events_resultdone::R](events_resultdone::R) reader structure"]
impl crate::Readable for EVENTS_RESULTDONE {}
#[doc = "`write(|w| ..)` method takes [events_resultdone::W](events_resultdone::W) writer structure"]
impl crate::Writable for EVENTS_RESULTDONE {}
#[doc = "A result is ready to get transferred to RAM."]
pub mod events_resultdone;
#[doc = "Calibration is complete\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_calibratedone](events_calibratedone) module"]
pub type EVENTS_CALIBRATEDONE = crate::Reg<u32, _EVENTS_CALIBRATEDONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_CALIBRATEDONE;
#[doc = "`read()` method returns [events_calibratedone::R](events_calibratedone::R) reader structure"]
impl crate::Readable for EVENTS_CALIBRATEDONE {}
#[doc = "`write(|w| ..)` method takes [events_calibratedone::W](events_calibratedone::W) writer structure"]
impl crate::Writable for EVENTS_CALIBRATEDONE {}
#[doc = "Calibration is complete"]
pub mod events_calibratedone;
#[doc = "The ADC has stopped\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_stopped](events_stopped) module"]
pub type EVENTS_STOPPED = crate::Reg<u32, _EVENTS_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STOPPED;
#[doc = "`read()` method returns [events_stopped::R](events_stopped::R) reader structure"]
impl crate::Readable for EVENTS_STOPPED {}
#[doc = "`write(|w| ..)` method takes [events_stopped::W](events_stopped::W) writer structure"]
impl crate::Writable for EVENTS_STOPPED {}
#[doc = "The ADC has stopped"]
pub mod events_stopped;
#[doc = "Publish configuration for event STARTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_started](publish_started) module"]
pub type PUBLISH_STARTED = crate::Reg<u32, _PUBLISH_STARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_STARTED;
#[doc = "`read()` method returns [publish_started::R](publish_started::R) reader structure"]
impl crate::Readable for PUBLISH_STARTED {}
#[doc = "`write(|w| ..)` method takes [publish_started::W](publish_started::W) writer structure"]
impl crate::Writable for PUBLISH_STARTED {}
#[doc = "Publish configuration for event STARTED"]
pub mod publish_started;
#[doc = "Publish configuration for event END\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_end](publish_end) module"]
pub type PUBLISH_END = crate::Reg<u32, _PUBLISH_END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_END;
#[doc = "`read()` method returns [publish_end::R](publish_end::R) reader structure"]
impl crate::Readable for PUBLISH_END {}
#[doc = "`write(|w| ..)` method takes [publish_end::W](publish_end::W) writer structure"]
impl crate::Writable for PUBLISH_END {}
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "Publish configuration for event DONE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_done](publish_done) module"]
pub type PUBLISH_DONE = crate::Reg<u32, _PUBLISH_DONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_DONE;
#[doc = "`read()` method returns [publish_done::R](publish_done::R) reader structure"]
impl crate::Readable for PUBLISH_DONE {}
#[doc = "`write(|w| ..)` method takes [publish_done::W](publish_done::W) writer structure"]
impl crate::Writable for PUBLISH_DONE {}
#[doc = "Publish configuration for event DONE"]
pub mod publish_done;
#[doc = "Publish configuration for event RESULTDONE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_resultdone](publish_resultdone) module"]
pub type PUBLISH_RESULTDONE = crate::Reg<u32, _PUBLISH_RESULTDONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_RESULTDONE;
#[doc = "`read()` method returns [publish_resultdone::R](publish_resultdone::R) reader structure"]
impl crate::Readable for PUBLISH_RESULTDONE {}
#[doc = "`write(|w| ..)` method takes [publish_resultdone::W](publish_resultdone::W) writer structure"]
impl crate::Writable for PUBLISH_RESULTDONE {}
#[doc = "Publish configuration for event RESULTDONE"]
pub mod publish_resultdone;
#[doc = "Publish configuration for event CALIBRATEDONE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_calibratedone](publish_calibratedone) module"]
pub type PUBLISH_CALIBRATEDONE = crate::Reg<u32, _PUBLISH_CALIBRATEDONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_CALIBRATEDONE;
#[doc = "`read()` method returns [publish_calibratedone::R](publish_calibratedone::R) reader structure"]
impl crate::Readable for PUBLISH_CALIBRATEDONE {}
#[doc = "`write(|w| ..)` method takes [publish_calibratedone::W](publish_calibratedone::W) writer structure"]
impl crate::Writable for PUBLISH_CALIBRATEDONE {}
#[doc = "Publish configuration for event CALIBRATEDONE"]
pub mod publish_calibratedone;
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
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "Enable or disable ADC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable or disable ADC"]
pub mod enable;
#[doc = "Resolution configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resolution](resolution) module"]
pub type RESOLUTION = crate::Reg<u32, _RESOLUTION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESOLUTION;
#[doc = "`read()` method returns [resolution::R](resolution::R) reader structure"]
impl crate::Readable for RESOLUTION {}
#[doc = "`write(|w| ..)` method takes [resolution::W](resolution::W) writer structure"]
impl crate::Writable for RESOLUTION {}
#[doc = "Resolution configuration"]
pub mod resolution;
#[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [oversample](oversample) module"]
pub type OVERSAMPLE = crate::Reg<u32, _OVERSAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVERSAMPLE;
#[doc = "`read()` method returns [oversample::R](oversample::R) reader structure"]
impl crate::Readable for OVERSAMPLE {}
#[doc = "`write(|w| ..)` method takes [oversample::W](oversample::W) writer structure"]
impl crate::Writable for OVERSAMPLE {}
#[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
pub mod oversample;
#[doc = "Controls normal or continuous sample rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [samplerate](samplerate) module"]
pub type SAMPLERATE = crate::Reg<u32, _SAMPLERATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMPLERATE;
#[doc = "`read()` method returns [samplerate::R](samplerate::R) reader structure"]
impl crate::Readable for SAMPLERATE {}
#[doc = "`write(|w| ..)` method takes [samplerate::W](samplerate::W) writer structure"]
impl crate::Writable for SAMPLERATE {}
#[doc = "Controls normal or continuous sample rate"]
pub mod samplerate;

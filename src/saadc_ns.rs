#[doc = r" Register block"]
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
    _reserved0: [u8; 112usize],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x84 - Subscribe configuration for task SAMPLE"]
    pub subscribe_sample: SUBSCRIBE_SAMPLE,
    #[doc = "0x88 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    #[doc = "0x8c - Subscribe configuration for task CALIBRATEOFFSET"]
    pub subscribe_calibrateoffset: SUBSCRIBE_CALIBRATEOFFSET,
    _reserved1: [u8; 112usize],
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
    _reserved2: [u8; 40usize],
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
    _reserved3: [u8; 296usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved4: [u8; 244usize],
    #[doc = "0x400 - Status"]
    pub status: STATUS,
    _reserved5: [u8; 252usize],
    #[doc = "0x500 - Enable or disable ADC"]
    pub enable: ENABLE,
    _reserved6: [u8; 12usize],
    #[doc = "0x510 - Unspecified"]
    pub ch: [CH; 8],
    _reserved7: [u8; 96usize],
    #[doc = "0x5f0 - Resolution configuration"]
    pub resolution: RESOLUTION,
    #[doc = "0x5f4 - Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
    pub oversample: OVERSAMPLE,
    #[doc = "0x5f8 - Controls normal or continuous sample rate"]
    pub samplerate: SAMPLERATE,
    _reserved8: [u8; 48usize],
    #[doc = "0x62c - RESULT EasyDMA channel"]
    pub result: RESULT,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct EVENTS_CH {
    #[doc = "0x00 - Description cluster: Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
    pub limith: self::events_ch::LIMITH,
    #[doc = "0x04 - Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW"]
    pub limitl: self::events_ch::LIMITL,
}
#[doc = r" Register block"]
#[doc = "Peripheral events."]
pub mod events_ch;
#[doc = r" Register block"]
#[repr(C)]
pub struct PUBLISH_CH {
    #[doc = "0x00 - Description cluster: Publish configuration for event CH\\[n\\].LIMITH"]
    pub limith: self::publish_ch::LIMITH,
    #[doc = "0x04 - Description cluster: Publish configuration for event CH\\[n\\].LIMITL"]
    pub limitl: self::publish_ch::LIMITL,
}
#[doc = r" Register block"]
#[doc = "Publish configuration for events"]
pub mod publish_ch;
#[doc = r" Register block"]
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
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod ch;
#[doc = r" Register block"]
#[repr(C)]
pub struct RESULT {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::result::PTR,
    #[doc = "0x04 - Maximum number of buffer words to transfer"]
    pub maxcnt: self::result::MAXCNT,
    #[doc = "0x08 - Number of buffer words transferred since last START"]
    pub amount: self::result::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "RESULT EasyDMA channel"]
pub mod result;
#[doc = "Start the ADC and prepare the result buffer in RAM"]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start the ADC and prepare the result buffer in RAM"]
pub mod tasks_start;
#[doc = "Take one ADC sample, if scan is enabled all channels are sampled"]
pub struct TASKS_SAMPLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Take one ADC sample, if scan is enabled all channels are sampled"]
pub mod tasks_sample;
#[doc = "Stop the ADC and terminate any on-going conversion"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop the ADC and terminate any on-going conversion"]
pub mod tasks_stop;
#[doc = "Starts offset auto-calibration"]
pub struct TASKS_CALIBRATEOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Starts offset auto-calibration"]
pub mod tasks_calibrateoffset;
#[doc = "Subscribe configuration for task START"]
pub struct SUBSCRIBE_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "Subscribe configuration for task SAMPLE"]
pub struct SUBSCRIBE_SAMPLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task SAMPLE"]
pub mod subscribe_sample;
#[doc = "Subscribe configuration for task STOP"]
pub struct SUBSCRIBE_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "Subscribe configuration for task CALIBRATEOFFSET"]
pub struct SUBSCRIBE_CALIBRATEOFFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task CALIBRATEOFFSET"]
pub mod subscribe_calibrateoffset;
#[doc = "The ADC has started"]
pub struct EVENTS_STARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The ADC has started"]
pub mod events_started;
#[doc = "The ADC has filled up the Result buffer"]
pub struct EVENTS_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The ADC has filled up the Result buffer"]
pub mod events_end;
#[doc = "A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
pub struct EVENTS_DONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
pub mod events_done;
#[doc = "A result is ready to get transferred to RAM."]
pub struct EVENTS_RESULTDONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A result is ready to get transferred to RAM."]
pub mod events_resultdone;
#[doc = "Calibration is complete"]
pub struct EVENTS_CALIBRATEDONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration is complete"]
pub mod events_calibratedone;
#[doc = "The ADC has stopped"]
pub struct EVENTS_STOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "The ADC has stopped"]
pub mod events_stopped;
#[doc = "Publish configuration for event STARTED"]
pub struct PUBLISH_STARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event STARTED"]
pub mod publish_started;
#[doc = "Publish configuration for event END"]
pub struct PUBLISH_END {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "Publish configuration for event DONE"]
pub struct PUBLISH_DONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event DONE"]
pub mod publish_done;
#[doc = "Publish configuration for event RESULTDONE"]
pub struct PUBLISH_RESULTDONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event RESULTDONE"]
pub mod publish_resultdone;
#[doc = "Publish configuration for event CALIBRATEDONE"]
pub struct PUBLISH_CALIBRATEDONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event CALIBRATEDONE"]
pub mod publish_calibratedone;
#[doc = "Publish configuration for event STOPPED"]
pub struct PUBLISH_STOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
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
#[doc = "Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status"]
pub mod status;
#[doc = "Enable or disable ADC"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable or disable ADC"]
pub mod enable;
#[doc = "Resolution configuration"]
pub struct RESOLUTION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Resolution configuration"]
pub mod resolution;
#[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
pub struct OVERSAMPLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
pub mod oversample;
#[doc = "Controls normal or continuous sample rate"]
pub struct SAMPLERATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Controls normal or continuous sample rate"]
pub mod samplerate;

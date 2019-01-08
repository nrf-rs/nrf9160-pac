#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ[n]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
    pub tasks_seqstart: [TASKS_SEQSTART; 2],
    #[doc = "0x10 - Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
    pub tasks_nextstep: TASKS_NEXTSTEP,
    _reserved1: [u8; 112usize],
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    #[doc = "0x88 - Description collection: Subscribe configuration for task SEQSTART[n]"]
    pub subscribe_seqstart: [SUBSCRIBE_SEQSTART; 2],
    #[doc = "0x90 - Subscribe configuration for task NEXTSTEP"]
    pub subscribe_nextstep: SUBSCRIBE_NEXTSTEP,
    _reserved2: [u8; 112usize],
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
    _reserved3: [u8; 100usize],
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    pub publish_stopped: PUBLISH_STOPPED,
    #[doc = "0x188 - Description collection: Publish configuration for event SEQSTARTED[n]"]
    pub publish_seqstarted: [PUBLISH_SEQSTARTED; 2],
    #[doc = "0x190 - Description collection: Publish configuration for event SEQEND[n]"]
    pub publish_seqend: [PUBLISH_SEQEND; 2],
    #[doc = "0x198 - Publish configuration for event PWMPERIODEND"]
    pub publish_pwmperiodend: PUBLISH_PWMPERIODEND,
    #[doc = "0x19c - Publish configuration for event LOOPSDONE"]
    pub publish_loopsdone: PUBLISH_LOOPSDONE,
    _reserved4: [u8; 96usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved5: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved6: [u8; 500usize],
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
    _reserved7: [u8; 8usize],
    #[doc = "0x520 - Unspecified"]
    pub seq0: SEQ,
    _reserved8: [u8; 16usize],
    #[doc = "0x540 - Unspecified"]
    pub seq1: SEQ,
    _reserved9: [u8; 16usize],
    #[doc = "0x560 - Unspecified"]
    pub psel: PSEL,
}
#[doc = r" Register block"]
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
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod seq;
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Description collection: Output pin select for PWM channel n"]
    pub out: [self::psel::OUT; 4],
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stops PWM pulse generation on all channels at the end of current PWM period, and stops sequence playback"]
pub mod tasks_stop;
#[doc = "Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ[n]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
pub struct TASKS_SEQSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ[n]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
pub mod tasks_seqstart;
#[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
pub struct TASKS_NEXTSTEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Steps by one value in the current sequence on all enabled channels if DECODER.MODE=NextStep. Does not cause PWM generation to start if not running."]
pub mod tasks_nextstep;
#[doc = "Subscribe configuration for task STOP"]
pub struct SUBSCRIBE_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "Description collection: Subscribe configuration for task SEQSTART[n]"]
pub struct SUBSCRIBE_SEQSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Subscribe configuration for task SEQSTART[n]"]
pub mod subscribe_seqstart;
#[doc = "Subscribe configuration for task NEXTSTEP"]
pub struct SUBSCRIBE_NEXTSTEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Subscribe configuration for task NEXTSTEP"]
pub mod subscribe_nextstep;
#[doc = "Response to STOP task, emitted when PWM pulses are no longer generated"]
pub struct EVENTS_STOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response to STOP task, emitted when PWM pulses are no longer generated"]
pub mod events_stopped;
#[doc = "Description collection: First PWM period started on sequence n"]
pub struct EVENTS_SEQSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: First PWM period started on sequence n"]
pub mod events_seqstarted;
#[doc = "Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
pub struct EVENTS_SEQEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Emitted at end of every sequence n, when last value from RAM has been applied to wave counter"]
pub mod events_seqend;
#[doc = "Emitted at the end of each PWM period"]
pub struct EVENTS_PWMPERIODEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Emitted at the end of each PWM period"]
pub mod events_pwmperiodend;
#[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
pub struct EVENTS_LOOPSDONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Concatenated sequences have been played the amount of times defined in LOOP.CNT"]
pub mod events_loopsdone;
#[doc = "Publish configuration for event STOPPED"]
pub struct PUBLISH_STOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "Description collection: Publish configuration for event SEQSTARTED[n]"]
pub struct PUBLISH_SEQSTARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Publish configuration for event SEQSTARTED[n]"]
pub mod publish_seqstarted;
#[doc = "Description collection: Publish configuration for event SEQEND[n]"]
pub struct PUBLISH_SEQEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Publish configuration for event SEQEND[n]"]
pub mod publish_seqend;
#[doc = "Publish configuration for event PWMPERIODEND"]
pub struct PUBLISH_PWMPERIODEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event PWMPERIODEND"]
pub mod publish_pwmperiodend;
#[doc = "Publish configuration for event LOOPSDONE"]
pub struct PUBLISH_LOOPSDONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event LOOPSDONE"]
pub mod publish_loopsdone;
#[doc = "Shortcuts between local events and tasks"]
pub struct SHORTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
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
#[doc = "PWM module enable register"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM module enable register"]
pub mod enable;
#[doc = "Selects operating mode of the wave counter"]
pub struct MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Selects operating mode of the wave counter"]
pub mod mode;
#[doc = "Value up to which the pulse generator counter counts"]
pub struct COUNTERTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value up to which the pulse generator counter counts"]
pub mod countertop;
#[doc = "Configuration for PWM_CLK"]
pub struct PRESCALER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration for PWM_CLK"]
pub mod prescaler;
#[doc = "Configuration of the decoder"]
pub struct DECODER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration of the decoder"]
pub mod decoder;
#[doc = "Number of playbacks of a loop"]
pub struct LOOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of playbacks of a loop"]
pub mod loop_;

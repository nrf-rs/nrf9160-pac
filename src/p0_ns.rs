#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Write GPIO port"]
    pub out: OUT,
    #[doc = "0x08 - Set individual bits in GPIO port"]
    pub outset: OUTSET,
    #[doc = "0x0c - Clear individual bits in GPIO port"]
    pub outclr: OUTCLR,
    #[doc = "0x10 - Read GPIO port"]
    pub in_: IN,
    #[doc = "0x14 - Direction of GPIO pins"]
    pub dir: DIR,
    #[doc = "0x18 - DIR set register"]
    pub dirset: DIRSET,
    #[doc = "0x1c - DIR clear register"]
    pub dirclr: DIRCLR,
    #[doc = "0x20 - Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF[n].SENSE registers"]
    pub latch: LATCH,
    #[doc = "0x24 - Select between default DETECT signal behaviour and LDETECT mode (For non-secure pin only)"]
    pub detectmode: DETECTMODE,
    #[doc = "0x28 - Select between default DETECT signal behaviour and LDETECT mode (For secure pin only)"]
    pub detectmode_sec: DETECTMODE_SEC,
    _reserved1: [u8; 468usize],
    #[doc = "0x200 - Description collection: Configuration of GPIO pins"]
    pub pin_cnf: [PIN_CNF; 32],
}
#[doc = "Write GPIO port"]
pub struct OUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write GPIO port"]
pub mod out;
#[doc = "Set individual bits in GPIO port"]
pub struct OUTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Set individual bits in GPIO port"]
pub mod outset;
#[doc = "Clear individual bits in GPIO port"]
pub struct OUTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear individual bits in GPIO port"]
pub mod outclr;
#[doc = "Read GPIO port"]
pub struct IN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Read GPIO port"]
pub mod in_;
#[doc = "Direction of GPIO pins"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direction of GPIO pins"]
pub mod dir;
#[doc = "DIR set register"]
pub struct DIRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DIR set register"]
pub mod dirset;
#[doc = "DIR clear register"]
pub struct DIRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DIR clear register"]
pub mod dirclr;
#[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF[n].SENSE registers"]
pub struct LATCH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF[n].SENSE registers"]
pub mod latch;
#[doc = "Select between default DETECT signal behaviour and LDETECT mode (For non-secure pin only)"]
pub struct DETECTMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select between default DETECT signal behaviour and LDETECT mode (For non-secure pin only)"]
pub mod detectmode;
#[doc = "Select between default DETECT signal behaviour and LDETECT mode (For secure pin only)"]
pub struct DETECTMODE_SEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select between default DETECT signal behaviour and LDETECT mode (For secure pin only)"]
pub mod detectmode_sec;
#[doc = "Description collection: Configuration of GPIO pins"]
pub struct PIN_CNF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection: Configuration of GPIO pins"]
pub mod pin_cnf;

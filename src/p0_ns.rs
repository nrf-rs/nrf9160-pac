#[doc = r"Register block"]
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
    #[doc = "0x20 - Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
    pub latch: LATCH,
    #[doc = "0x24 - Select between default DETECT signal behavior and LDETECT mode (For non-secure pin only)"]
    pub detectmode: DETECTMODE,
    #[doc = "0x28 - Select between default DETECT signal behavior and LDETECT mode (For secure pin only)"]
    pub detectmode_sec: DETECTMODE_SEC,
    _reserved10: [u8; 468usize],
    #[doc = "0x200 - Description collection: Configuration of GPIO pins"]
    pub pin_cnf: [PIN_CNF; 32],
}
#[doc = "Write GPIO port\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out](out) module"]
pub type OUT = crate::Reg<u32, _OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT;
#[doc = "`read()` method returns [out::R](out::R) reader structure"]
impl crate::Readable for OUT {}
#[doc = "`write(|w| ..)` method takes [out::W](out::W) writer structure"]
impl crate::Writable for OUT {}
#[doc = "Write GPIO port"]
pub mod out;
#[doc = "Set individual bits in GPIO port\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outset](outset) module"]
pub type OUTSET = crate::Reg<u32, _OUTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTSET;
#[doc = "`read()` method returns [outset::R](outset::R) reader structure"]
impl crate::Readable for OUTSET {}
#[doc = "`write(|w| ..)` method takes [outset::W](outset::W) writer structure"]
impl crate::Writable for OUTSET {}
#[doc = "Set individual bits in GPIO port"]
pub mod outset;
#[doc = "Clear individual bits in GPIO port\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outclr](outclr) module"]
pub type OUTCLR = crate::Reg<u32, _OUTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTCLR;
#[doc = "`read()` method returns [outclr::R](outclr::R) reader structure"]
impl crate::Readable for OUTCLR {}
#[doc = "`write(|w| ..)` method takes [outclr::W](outclr::W) writer structure"]
impl crate::Writable for OUTCLR {}
#[doc = "Clear individual bits in GPIO port"]
pub mod outclr;
#[doc = "Read GPIO port\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_](in_) module"]
pub type IN = crate::Reg<u32, _IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IN;
#[doc = "`read()` method returns [in_::R](in_::R) reader structure"]
impl crate::Readable for IN {}
#[doc = "Read GPIO port"]
pub mod in_;
#[doc = "Direction of GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`read()` method returns [dir::R](dir::R) reader structure"]
impl crate::Readable for DIR {}
#[doc = "`write(|w| ..)` method takes [dir::W](dir::W) writer structure"]
impl crate::Writable for DIR {}
#[doc = "Direction of GPIO pins"]
pub mod dir;
#[doc = "DIR set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirset](dirset) module"]
pub type DIRSET = crate::Reg<u32, _DIRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRSET;
#[doc = "`read()` method returns [dirset::R](dirset::R) reader structure"]
impl crate::Readable for DIRSET {}
#[doc = "`write(|w| ..)` method takes [dirset::W](dirset::W) writer structure"]
impl crate::Writable for DIRSET {}
#[doc = "DIR set register"]
pub mod dirset;
#[doc = "DIR clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dirclr](dirclr) module"]
pub type DIRCLR = crate::Reg<u32, _DIRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRCLR;
#[doc = "`read()` method returns [dirclr::R](dirclr::R) reader structure"]
impl crate::Readable for DIRCLR {}
#[doc = "`write(|w| ..)` method takes [dirclr::W](dirclr::W) writer structure"]
impl crate::Writable for DIRCLR {}
#[doc = "DIR clear register"]
pub mod dirclr;
#[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [latch](latch) module"]
pub type LATCH = crate::Reg<u32, _LATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LATCH;
#[doc = "`read()` method returns [latch::R](latch::R) reader structure"]
impl crate::Readable for LATCH {}
#[doc = "`write(|w| ..)` method takes [latch::W](latch::W) writer structure"]
impl crate::Writable for LATCH {}
#[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF\\[n\\].SENSE registers"]
pub mod latch;
#[doc = "Select between default DETECT signal behavior and LDETECT mode (For non-secure pin only)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [detectmode](detectmode) module"]
pub type DETECTMODE = crate::Reg<u32, _DETECTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DETECTMODE;
#[doc = "`read()` method returns [detectmode::R](detectmode::R) reader structure"]
impl crate::Readable for DETECTMODE {}
#[doc = "`write(|w| ..)` method takes [detectmode::W](detectmode::W) writer structure"]
impl crate::Writable for DETECTMODE {}
#[doc = "Select between default DETECT signal behavior and LDETECT mode (For non-secure pin only)"]
pub mod detectmode;
#[doc = "Select between default DETECT signal behavior and LDETECT mode (For secure pin only)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [detectmode_sec](detectmode_sec) module"]
pub type DETECTMODE_SEC = crate::Reg<u32, _DETECTMODE_SEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DETECTMODE_SEC;
#[doc = "`read()` method returns [detectmode_sec::R](detectmode_sec::R) reader structure"]
impl crate::Readable for DETECTMODE_SEC {}
#[doc = "`write(|w| ..)` method takes [detectmode_sec::W](detectmode_sec::W) writer structure"]
impl crate::Writable for DETECTMODE_SEC {}
#[doc = "Select between default DETECT signal behavior and LDETECT mode (For secure pin only)"]
pub mod detectmode_sec;
#[doc = "Description collection: Configuration of GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin_cnf](pin_cnf) module"]
pub type PIN_CNF = crate::Reg<u32, _PIN_CNF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN_CNF;
#[doc = "`read()` method returns [pin_cnf::R](pin_cnf::R) reader structure"]
impl crate::Readable for PIN_CNF {}
#[doc = "`write(|w| ..)` method takes [pin_cnf::W](pin_cnf::W) writer structure"]
impl crate::Writable for PIN_CNF {}
#[doc = "Description collection: Configuration of GPIO pins"]
pub mod pin_cnf;

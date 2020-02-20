#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - A security violation has been detected for the RAM memory space"]
    pub events_ramaccerr: EVENTS_RAMACCERR,
    #[doc = "0x104 - A security violation has been detected for the flash memory space"]
    pub events_flashaccerr: EVENTS_FLASHACCERR,
    #[doc = "0x108 - A security violation has been detected on one or several peripherals"]
    pub events_periphaccerr: EVENTS_PERIPHACCERR,
    _reserved3: [u8; 116usize],
    #[doc = "0x180 - Publish configuration for event RAMACCERR"]
    pub publish_ramaccerr: PUBLISH_RAMACCERR,
    #[doc = "0x184 - Publish configuration for event FLASHACCERR"]
    pub publish_flashaccerr: PUBLISH_FLASHACCERR,
    #[doc = "0x188 - Publish configuration for event PERIPHACCERR"]
    pub publish_periphaccerr: PUBLISH_PERIPHACCERR,
    _reserved6: [u8; 372usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 244usize],
    #[doc = "0x400 - Show implemented features for the current device"]
    pub cap: CAP,
    _reserved10: [u8; 60usize],
    #[doc = "0x440 - Unspecified"]
    pub extdomain: [EXTDOMAIN; 1],
    _reserved11: [u8; 60usize],
    #[doc = "0x480 - Unspecified"]
    pub dppi: [DPPI; 1],
    _reserved12: [u8; 56usize],
    #[doc = "0x4c0 - Unspecified"]
    pub gpioport: [GPIOPORT; 1],
    _reserved13: [u8; 56usize],
    #[doc = "0x500 - Unspecified"]
    pub flashnsc: [FLASHNSC; 2],
    _reserved14: [u8; 48usize],
    #[doc = "0x540 - Unspecified"]
    pub ramnsc: [RAMNSC; 2],
    _reserved15: [u8; 176usize],
    #[doc = "0x600 - Unspecified"]
    pub flashregion: [FLASHREGION; 32],
    _reserved16: [u8; 128usize],
    #[doc = "0x700 - Unspecified"]
    pub ramregion: [RAMREGION; 32],
    _reserved17: [u8; 128usize],
    #[doc = "0x800 - Unspecified"]
    pub periphid: [PERIPHID; 67],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct EXTDOMAIN {
    #[doc = "0x00 - Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n"]
    pub perm: self::extdomain::PERM,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod extdomain;
#[doc = r"Register block"]
#[repr(C)]
pub struct DPPI {
    #[doc = "0x00 - Description cluster: Select between secure and non-secure attribute for the DPPI channels."]
    pub perm: self::dppi::PERM,
    #[doc = "0x04 - Description cluster: Prevent further modification of the corresponding PERM register"]
    pub lock: self::dppi::LOCK,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod dppi;
#[doc = r"Register block"]
#[repr(C)]
pub struct GPIOPORT {
    #[doc = "0x00 - Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n."]
    pub perm: self::gpioport::PERM,
    #[doc = "0x04 - Description cluster: Prevent further modification of the corresponding PERM register"]
    pub lock: self::gpioport::LOCK,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod gpioport;
#[doc = r"Register block"]
#[repr(C)]
pub struct FLASHNSC {
    #[doc = "0x00 - Description cluster: Define which flash region can contain the non-secure callable (NSC) region n"]
    pub region: self::flashnsc::REGION,
    #[doc = "0x04 - Description cluster: Define the size of the non-secure callable (NSC) region n"]
    pub size: self::flashnsc::SIZE,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod flashnsc;
#[doc = r"Register block"]
#[repr(C)]
pub struct RAMNSC {
    #[doc = "0x00 - Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n"]
    pub region: self::ramnsc::REGION,
    #[doc = "0x04 - Description cluster: Define the size of the non-secure callable (NSC) region n"]
    pub size: self::ramnsc::SIZE,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod ramnsc;
#[doc = r"Register block"]
#[repr(C)]
pub struct FLASHREGION {
    #[doc = "0x00 - Description cluster: Access permissions for flash region n"]
    pub perm: self::flashregion::PERM,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod flashregion;
#[doc = r"Register block"]
#[repr(C)]
pub struct RAMREGION {
    #[doc = "0x00 - Description cluster: Access permissions for RAM region n"]
    pub perm: self::ramregion::PERM,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod ramregion;
#[doc = r"Register block"]
#[repr(C)]
pub struct PERIPHID {
    #[doc = "0x00 - Description cluster: List capabilities and access permissions for the peripheral with ID n"]
    pub perm: self::periphid::PERM,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod periphid;
#[doc = "A security violation has been detected for the RAM memory space\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ramaccerr](events_ramaccerr) module"]
pub type EVENTS_RAMACCERR = crate::Reg<u32, _EVENTS_RAMACCERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RAMACCERR;
#[doc = "`read()` method returns [events_ramaccerr::R](events_ramaccerr::R) reader structure"]
impl crate::Readable for EVENTS_RAMACCERR {}
#[doc = "`write(|w| ..)` method takes [events_ramaccerr::W](events_ramaccerr::W) writer structure"]
impl crate::Writable for EVENTS_RAMACCERR {}
#[doc = "A security violation has been detected for the RAM memory space"]
pub mod events_ramaccerr;
#[doc = "A security violation has been detected for the flash memory space\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_flashaccerr](events_flashaccerr) module"]
pub type EVENTS_FLASHACCERR = crate::Reg<u32, _EVENTS_FLASHACCERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_FLASHACCERR;
#[doc = "`read()` method returns [events_flashaccerr::R](events_flashaccerr::R) reader structure"]
impl crate::Readable for EVENTS_FLASHACCERR {}
#[doc = "`write(|w| ..)` method takes [events_flashaccerr::W](events_flashaccerr::W) writer structure"]
impl crate::Writable for EVENTS_FLASHACCERR {}
#[doc = "A security violation has been detected for the flash memory space"]
pub mod events_flashaccerr;
#[doc = "A security violation has been detected on one or several peripherals\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_periphaccerr](events_periphaccerr) module"]
pub type EVENTS_PERIPHACCERR = crate::Reg<u32, _EVENTS_PERIPHACCERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_PERIPHACCERR;
#[doc = "`read()` method returns [events_periphaccerr::R](events_periphaccerr::R) reader structure"]
impl crate::Readable for EVENTS_PERIPHACCERR {}
#[doc = "`write(|w| ..)` method takes [events_periphaccerr::W](events_periphaccerr::W) writer structure"]
impl crate::Writable for EVENTS_PERIPHACCERR {}
#[doc = "A security violation has been detected on one or several peripherals"]
pub mod events_periphaccerr;
#[doc = "Publish configuration for event RAMACCERR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_ramaccerr](publish_ramaccerr) module"]
pub type PUBLISH_RAMACCERR = crate::Reg<u32, _PUBLISH_RAMACCERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_RAMACCERR;
#[doc = "`read()` method returns [publish_ramaccerr::R](publish_ramaccerr::R) reader structure"]
impl crate::Readable for PUBLISH_RAMACCERR {}
#[doc = "`write(|w| ..)` method takes [publish_ramaccerr::W](publish_ramaccerr::W) writer structure"]
impl crate::Writable for PUBLISH_RAMACCERR {}
#[doc = "Publish configuration for event RAMACCERR"]
pub mod publish_ramaccerr;
#[doc = "Publish configuration for event FLASHACCERR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_flashaccerr](publish_flashaccerr) module"]
pub type PUBLISH_FLASHACCERR = crate::Reg<u32, _PUBLISH_FLASHACCERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_FLASHACCERR;
#[doc = "`read()` method returns [publish_flashaccerr::R](publish_flashaccerr::R) reader structure"]
impl crate::Readable for PUBLISH_FLASHACCERR {}
#[doc = "`write(|w| ..)` method takes [publish_flashaccerr::W](publish_flashaccerr::W) writer structure"]
impl crate::Writable for PUBLISH_FLASHACCERR {}
#[doc = "Publish configuration for event FLASHACCERR"]
pub mod publish_flashaccerr;
#[doc = "Publish configuration for event PERIPHACCERR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_periphaccerr](publish_periphaccerr) module"]
pub type PUBLISH_PERIPHACCERR = crate::Reg<u32, _PUBLISH_PERIPHACCERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_PERIPHACCERR;
#[doc = "`read()` method returns [publish_periphaccerr::R](publish_periphaccerr::R) reader structure"]
impl crate::Readable for PUBLISH_PERIPHACCERR {}
#[doc = "`write(|w| ..)` method takes [publish_periphaccerr::W](publish_periphaccerr::W) writer structure"]
impl crate::Writable for PUBLISH_PERIPHACCERR {}
#[doc = "Publish configuration for event PERIPHACCERR"]
pub mod publish_periphaccerr;
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
#[doc = "Show implemented features for the current device\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap](cap) module"]
pub type CAP = crate::Reg<u32, _CAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP;
#[doc = "`read()` method returns [cap::R](cap::R) reader structure"]
impl crate::Readable for CAP {}
#[doc = "Show implemented features for the current device"]
pub mod cap;

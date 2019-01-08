#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - A security violation has been detected for the RAM memory space"]
    pub events_ramaccerr: EVENTS_RAMACCERR,
    #[doc = "0x104 - A security violation has been detected for the flash memory space"]
    pub events_flashaccerr: EVENTS_FLASHACCERR,
    #[doc = "0x108 - A security violation has been detected on one or several peripherals"]
    pub events_periphaccerr: EVENTS_PERIPHACCERR,
    _reserved1: [u8; 116usize],
    #[doc = "0x180 - Publish configuration for event RAMACCERR"]
    pub publish_ramaccerr: PUBLISH_RAMACCERR,
    #[doc = "0x184 - Publish configuration for event FLASHACCERR"]
    pub publish_flashaccerr: PUBLISH_FLASHACCERR,
    #[doc = "0x188 - Publish configuration for event PERIPHACCERR"]
    pub publish_periphaccerr: PUBLISH_PERIPHACCERR,
    _reserved2: [u8; 372usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved3: [u8; 244usize],
    #[doc = "0x400 - Show implemented features for the current device"]
    pub cap: CAP,
    _reserved4: [u8; 60usize],
    #[doc = "0x440 - Unspecified"]
    pub extdomain: [EXTDOMAIN; 1],
    _reserved5: [u8; 60usize],
    #[doc = "0x480 - Unspecified"]
    pub dppi: [DPPI; 1],
    _reserved6: [u8; 56usize],
    #[doc = "0x4c0 - Unspecified"]
    pub gpioport: [GPIOPORT; 1],
    _reserved7: [u8; 56usize],
    #[doc = "0x500 - Unspecified"]
    pub flashnsc: [FLASHNSC; 2],
    _reserved8: [u8; 48usize],
    #[doc = "0x540 - Unspecified"]
    pub ramnsc: [RAMNSC; 2],
    _reserved9: [u8; 176usize],
    #[doc = "0x600 - Unspecified"]
    pub flashregion: [FLASHREGION; 32],
    _reserved10: [u8; 128usize],
    #[doc = "0x700 - Unspecified"]
    pub ramregion: [RAMREGION; 32],
    _reserved11: [u8; 128usize],
    #[doc = "0x800 - Unspecified"]
    pub periphid: [PERIPHID; 67],
}
#[doc = r" Register block"]
#[repr(C)]
pub struct EXTDOMAIN {
    #[doc = "0x00 - Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n"]
    pub perm: self::extdomain::PERM,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod extdomain;
#[doc = r" Register block"]
#[repr(C)]
pub struct DPPI {
    #[doc = "0x00 - Description cluster: Select between secure and non-secure attribute for the DPPI channels."]
    pub perm: self::dppi::PERM,
    #[doc = "0x04 - Description cluster: Prevent further modification of the corresponding PERM register"]
    pub lock: self::dppi::LOCK,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod dppi;
#[doc = r" Register block"]
#[repr(C)]
pub struct GPIOPORT {
    #[doc = "0x00 - Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n."]
    pub perm: self::gpioport::PERM,
    #[doc = "0x04 - Description cluster: Prevent further modification of the corresponding PERM register"]
    pub lock: self::gpioport::LOCK,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod gpioport;
#[doc = r" Register block"]
#[repr(C)]
pub struct FLASHNSC {
    #[doc = "0x00 - Description cluster: Define which flash region can contain the non-secure callable (NSC) region n"]
    pub region: self::flashnsc::REGION,
    #[doc = "0x04 - Description cluster: Define the size of the non-secure callable (NSC) region n"]
    pub size: self::flashnsc::SIZE,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod flashnsc;
#[doc = r" Register block"]
#[repr(C)]
pub struct RAMNSC {
    #[doc = "0x00 - Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n"]
    pub region: self::ramnsc::REGION,
    #[doc = "0x04 - Description cluster: Define the size of the non-secure callable (NSC) region n"]
    pub size: self::ramnsc::SIZE,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod ramnsc;
#[doc = r" Register block"]
#[repr(C)]
pub struct FLASHREGION {
    #[doc = "0x00 - Description cluster: Access permissions for flash region n"]
    pub perm: self::flashregion::PERM,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod flashregion;
#[doc = r" Register block"]
#[repr(C)]
pub struct RAMREGION {
    #[doc = "0x00 - Description cluster: Access permissions for RAM region n"]
    pub perm: self::ramregion::PERM,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod ramregion;
#[doc = r" Register block"]
#[repr(C)]
pub struct PERIPHID {
    #[doc = "0x00 - Description cluster: List capabilities and access permissions for the peripheral with ID n"]
    pub perm: self::periphid::PERM,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod periphid;
#[doc = "A security violation has been detected for the RAM memory space"]
pub struct EVENTS_RAMACCERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A security violation has been detected for the RAM memory space"]
pub mod events_ramaccerr;
#[doc = "A security violation has been detected for the flash memory space"]
pub struct EVENTS_FLASHACCERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A security violation has been detected for the flash memory space"]
pub mod events_flashaccerr;
#[doc = "A security violation has been detected on one or several peripherals"]
pub struct EVENTS_PERIPHACCERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A security violation has been detected on one or several peripherals"]
pub mod events_periphaccerr;
#[doc = "Publish configuration for event RAMACCERR"]
pub struct PUBLISH_RAMACCERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event RAMACCERR"]
pub mod publish_ramaccerr;
#[doc = "Publish configuration for event FLASHACCERR"]
pub struct PUBLISH_FLASHACCERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event FLASHACCERR"]
pub mod publish_flashaccerr;
#[doc = "Publish configuration for event PERIPHACCERR"]
pub struct PUBLISH_PERIPHACCERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Publish configuration for event PERIPHACCERR"]
pub mod publish_periphaccerr;
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
#[doc = "Show implemented features for the current device"]
pub struct CAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Show implemented features for the current device"]
pub mod cap;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1536usize],
    #[doc = "0x600 - Unspecified"]
    pub ram0: RAM,
    _reserved1: [u8; 4usize],
    #[doc = "0x610 - Unspecified"]
    pub ram1: RAM,
    _reserved2: [u8; 4usize],
    #[doc = "0x620 - Unspecified"]
    pub ram2: RAM,
    _reserved3: [u8; 4usize],
    #[doc = "0x630 - Unspecified"]
    pub ram3: RAM,
    _reserved4: [u8; 4usize],
    #[doc = "0x640 - Unspecified"]
    pub ram4: RAM,
    _reserved5: [u8; 4usize],
    #[doc = "0x650 - Unspecified"]
    pub ram5: RAM,
    _reserved6: [u8; 4usize],
    #[doc = "0x660 - Unspecified"]
    pub ram6: RAM,
    _reserved7: [u8; 4usize],
    #[doc = "0x670 - Unspecified"]
    pub ram7: RAM,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct RAM {
    #[doc = "0x00 - Description cluster: RAMn power control register"]
    pub power: self::ram::POWER,
    #[doc = "0x04 - Description cluster: RAMn power control set register"]
    pub powerset: self::ram::POWERSET,
    #[doc = "0x08 - Description cluster: RAMn power control clear register"]
    pub powerclr: self::ram::POWERCLR,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod ram;

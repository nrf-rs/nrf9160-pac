#[doc = r" Register block"]
#[repr(C)]
pub struct CONFIG {
    #[doc = "0x00 - Description cluster: Destination address where content of the key value registers (KEYSLOT.KEYn.VALUE\\[0-3\\]) will be pushed by KMU. Note that this address MUST match that of a peripherals APB mapped write-only key registers, else the KMU can push this key value into an address range which the CPU can potentially read!"]
    pub dest: self::config::DEST,
    #[doc = "0x04 - Description cluster: Define permissions for the key slot with ID=n+1. Bits 0-15 and 16-31 can only be written once."]
    pub perm: self::config::PERM,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod config;
#[doc = r" Register block"]
#[repr(C)]
pub struct KEY {
    #[doc = "0x00 - Description collection: Define bits \\[31+o*32:0+o*32\\] of value assigned to KMU key slot ID=n+1"]
    pub value: [self::key::VALUE; 4],
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod key;

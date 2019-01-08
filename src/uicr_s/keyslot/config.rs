#[doc = "Description cluster: Destination address where content of the key value registers (KEYSLOT.KEYn.VALUE[0-3]) will be pushed by KMU. Note that this address MUST match that of a peripherals APB mapped write-only key registers, else the KMU can push this key value into an address range which the CPU can potentially read!"]
pub struct DEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster: Destination address where content of the key value registers (KEYSLOT.KEYn.VALUE[0-3]) will be pushed by KMU. Note that this address MUST match that of a peripherals APB mapped write-only key registers, else the KMU can push this key value into an address range which the CPU can potentially read!"]
pub mod dest;
#[doc = "Description cluster: Define permissions for the key slot with ID=n+1. Bits 0-15 and 16-31 can only be written once."]
pub struct PERM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster: Define permissions for the key slot with ID=n+1. Bits 0-15 and 16-31 can only be written once."]
pub mod perm;

#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Push a key slot over secure APB"]
    pub tasks_push_keyslot: TASKS_PUSH_KEYSLOT,
    _reserved0: [u8; 252usize],
    #[doc = "0x100 - Key successfully pushed over secure APB"]
    pub events_keyslot_pushed: EVENTS_KEYSLOT_PUSHED,
    #[doc = "0x104 - Key has been revoked and cannot be tasked for selection"]
    pub events_keyslot_revoked: EVENTS_KEYSLOT_REVOKED,
    #[doc = "0x108 - No key slot selected, no destination address defined, or error during push operation"]
    pub events_keyslot_error: EVENTS_KEYSLOT_ERROR,
    _reserved1: [u8; 500usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    #[doc = "0x30c - Pending interrupts"]
    pub intpend: INTPEND,
    _reserved2: [u8; 252usize],
    #[doc = "0x40c - Status bits for KMU operation"]
    pub status: STATUS,
    _reserved3: [u8; 240usize],
    #[doc = "0x500 - Select key slot ID to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started"]
    pub selectkeyslot: SELECTKEYSLOT,
}
#[doc = "Push a key slot over secure APB"]
pub struct TASKS_PUSH_KEYSLOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Push a key slot over secure APB"]
pub mod tasks_push_keyslot;
#[doc = "Key successfully pushed over secure APB"]
pub struct EVENTS_KEYSLOT_PUSHED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key successfully pushed over secure APB"]
pub mod events_keyslot_pushed;
#[doc = "Key has been revoked and cannot be tasked for selection"]
pub struct EVENTS_KEYSLOT_REVOKED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Key has been revoked and cannot be tasked for selection"]
pub mod events_keyslot_revoked;
#[doc = "No key slot selected, no destination address defined, or error during push operation"]
pub struct EVENTS_KEYSLOT_ERROR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "No key slot selected, no destination address defined, or error during push operation"]
pub mod events_keyslot_error;
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
#[doc = "Pending interrupts"]
pub struct INTPEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pending interrupts"]
pub mod intpend;
#[doc = "Status bits for KMU operation"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status bits for KMU operation"]
pub mod status;
#[doc = "Select key slot ID to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started"]
pub struct SELECTKEYSLOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Select key slot ID to be read over AHB or pushed over secure APB when TASKS_PUSH_KEYSLOT is started"]
pub mod selectkeyslot;

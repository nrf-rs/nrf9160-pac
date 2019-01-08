#[doc = "Lock ERASEALL mechanism"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock ERASEALL mechanism"]
pub mod lock;
#[doc = "Unlock ERASEPROTECT and perform ERASEALL"]
pub struct DISABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unlock ERASEPROTECT and perform ERASEALL"]
pub mod disable;

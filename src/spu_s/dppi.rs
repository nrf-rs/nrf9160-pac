#[doc = "Description cluster: Select between secure and non-secure attribute for the DPPI channels."]
pub struct PERM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster: Select between secure and non-secure attribute for the DPPI channels."]
pub mod perm;
#[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
pub struct LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
pub mod lock;

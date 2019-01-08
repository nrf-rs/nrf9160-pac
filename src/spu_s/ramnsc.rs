#[doc = "Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n"]
pub struct REGION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster: Define which RAM region can contain the non-secure callable (NSC) region n"]
pub mod region;
#[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
pub struct SIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
pub mod size;

#[doc = "Description cluster: Destination address where content of the key value registers (KEYSLOT.KEYn.VALUE\\[0-3\\]) will be pushed by KMU. Note that this address MUST match that of a peripherals APB mapped write-only key registers, else the KMU can push this key value into an address range which the CPU can potentially read!\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dest](dest) module"]
pub type DEST = crate::Reg<u32, _DEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEST;
#[doc = "`read()` method returns [dest::R](dest::R) reader structure"]
impl crate::Readable for DEST {}
#[doc = "`write(|w| ..)` method takes [dest::W](dest::W) writer structure"]
impl crate::Writable for DEST {}
#[doc = "Description cluster: Destination address where content of the key value registers (KEYSLOT.KEYn.VALUE\\[0-3\\]) will be pushed by KMU. Note that this address MUST match that of a peripherals APB mapped write-only key registers, else the KMU can push this key value into an address range which the CPU can potentially read!"]
pub mod dest;
#[doc = "Description cluster: Define permissions for the key slot with ID=n+1. Bits 0-15 and 16-31 can only be written once.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [perm](perm) module"]
pub type PERM = crate::Reg<u32, _PERM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERM;
#[doc = "`read()` method returns [perm::R](perm::R) reader structure"]
impl crate::Readable for PERM {}
#[doc = "`write(|w| ..)` method takes [perm::W](perm::W) writer structure"]
impl crate::Writable for PERM {}
#[doc = "Description cluster: Define permissions for the key slot with ID=n+1. Bits 0-15 and 16-31 can only be written once."]
pub mod perm;

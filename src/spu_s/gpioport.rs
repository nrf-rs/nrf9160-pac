#[doc = "Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [perm](perm) module"]
pub type PERM = crate::Reg<u32, _PERM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERM;
#[doc = "`read()` method returns [perm::R](perm::R) reader structure"]
impl crate::Readable for PERM {}
#[doc = "`write(|w| ..)` method takes [perm::W](perm::W) writer structure"]
impl crate::Writable for PERM {}
#[doc = "Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n."]
pub mod perm;
#[doc = "Description cluster: Prevent further modification of the corresponding PERM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
pub mod lock;

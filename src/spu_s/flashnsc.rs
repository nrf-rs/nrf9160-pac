#[doc = "Description cluster: Define which flash region can contain the non-secure callable (NSC) region n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [region](region) module"]
pub type REGION = crate::Reg<u32, _REGION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION;
#[doc = "`read()` method returns [region::R](region::R) reader structure"]
impl crate::Readable for REGION {}
#[doc = "`write(|w| ..)` method takes [region::W](region::W) writer structure"]
impl crate::Writable for REGION {}
#[doc = "Description cluster: Define which flash region can contain the non-secure callable (NSC) region n"]
pub mod region;
#[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [size](size) module"]
pub type SIZE = crate::Reg<u32, _SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIZE;
#[doc = "`read()` method returns [size::R](size::R) reader structure"]
impl crate::Readable for SIZE {}
#[doc = "`write(|w| ..)` method takes [size::W](size::W) writer structure"]
impl crate::Writable for SIZE {}
#[doc = "Description cluster: Define the size of the non-secure callable (NSC) region n"]
pub mod size;

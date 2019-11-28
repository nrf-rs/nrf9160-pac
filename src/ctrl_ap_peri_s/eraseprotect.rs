#[doc = "Lock ERASEALL mechanism\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Lock ERASEALL mechanism"]
pub mod lock;
#[doc = "Unlock ERASEPROTECT and perform ERASEALL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [disable](disable) module"]
pub type DISABLE = crate::Reg<u32, _DISABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DISABLE;
#[doc = "`read()` method returns [disable::R](disable::R) reader structure"]
impl crate::Readable for DISABLE {}
#[doc = "`write(|w| ..)` method takes [disable::W](disable::W) writer structure"]
impl crate::Writable for DISABLE {}
#[doc = "Unlock ERASEPROTECT and perform ERASEALL"]
pub mod disable;

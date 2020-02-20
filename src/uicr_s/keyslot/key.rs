#[doc = "Description collection: Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value](value) module"]
pub type VALUE = crate::Reg<u32, _VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VALUE;
#[doc = "`read()` method returns [value::R](value::R) reader structure"]
impl crate::Readable for VALUE {}
#[doc = "`write(|w| ..)` method takes [value::W](value::W) writer structure"]
impl crate::Writable for VALUE {}
#[doc = "Description collection: Define bits \\[31+o*32:0+o*32\\]
of value assigned to KMU key slot."]
pub mod value;

#[doc = "Data sent from the debugger to the CPU\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdata](rxdata) module"]
pub type RXDATA = crate::Reg<u32, _RXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDATA;
#[doc = "`read()` method returns [rxdata::R](rxdata::R) reader structure"]
impl crate::Readable for RXDATA {}
#[doc = "Data sent from the debugger to the CPU"]
pub mod rxdata;
#[doc = "Status to indicate if data sent from the debugger to the CPU has been read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxstatus](rxstatus) module"]
pub type RXSTATUS = crate::Reg<u32, _RXSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXSTATUS;
#[doc = "`read()` method returns [rxstatus::R](rxstatus::R) reader structure"]
impl crate::Readable for RXSTATUS {}
#[doc = "Status to indicate if data sent from the debugger to the CPU has been read"]
pub mod rxstatus;
#[doc = "Data sent from the CPU to the debugger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdata](txdata) module"]
pub type TXDATA = crate::Reg<u32, _TXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDATA;
#[doc = "`read()` method returns [txdata::R](txdata::R) reader structure"]
impl crate::Readable for TXDATA {}
#[doc = "`write(|w| ..)` method takes [txdata::W](txdata::W) writer structure"]
impl crate::Writable for TXDATA {}
#[doc = "Data sent from the CPU to the debugger"]
pub mod txdata;
#[doc = "Status to indicate if data sent from the CPU to the debugger has been read\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txstatus](txstatus) module"]
pub type TXSTATUS = crate::Reg<u32, _TXSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXSTATUS;
#[doc = "`read()` method returns [txstatus::R](txstatus::R) reader structure"]
impl crate::Readable for TXSTATUS {}
#[doc = "Status to indicate if data sent from the CPU to the debugger has been read"]
pub mod txstatus;

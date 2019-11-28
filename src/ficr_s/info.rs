#[doc = "Description collection: Device identifier\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deviceid](deviceid) module"]
pub type DEVICEID = crate::Reg<u32, _DEVICEID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICEID;
#[doc = "`read()` method returns [deviceid::R](deviceid::R) reader structure"]
impl crate::Readable for DEVICEID {}
#[doc = "Description collection: Device identifier"]
pub mod deviceid;
#[doc = "Part code\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [part](part) module"]
pub type PART = crate::Reg<u32, _PART>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PART;
#[doc = "`read()` method returns [part::R](part::R) reader structure"]
impl crate::Readable for PART {}
#[doc = "Part code"]
pub mod part;
#[doc = "Part Variant, Hardware version and Production configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [variant](variant) module"]
pub type VARIANT = crate::Reg<u32, _VARIANT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VARIANT;
#[doc = "`read()` method returns [variant::R](variant::R) reader structure"]
impl crate::Readable for VARIANT {}
#[doc = "Part Variant, Hardware version and Production configuration"]
pub mod variant;
#[doc = "Package option\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [package](package) module"]
pub type PACKAGE = crate::Reg<u32, _PACKAGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PACKAGE;
#[doc = "`read()` method returns [package::R](package::R) reader structure"]
impl crate::Readable for PACKAGE {}
#[doc = "Package option"]
pub mod package;
#[doc = "RAM variant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ram](ram) module"]
pub type RAM = crate::Reg<u32, _RAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAM;
#[doc = "`read()` method returns [ram::R](ram::R) reader structure"]
impl crate::Readable for RAM {}
#[doc = "RAM variant"]
pub mod ram;
#[doc = "Flash variant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [flash](flash) module"]
pub type FLASH = crate::Reg<u32, _FLASH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH;
#[doc = "`read()` method returns [flash::R](flash::R) reader structure"]
impl crate::Readable for FLASH {}
#[doc = "Flash variant"]
pub mod flash;
#[doc = "Code memory page size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [codepagesize](codepagesize) module"]
pub type CODEPAGESIZE = crate::Reg<u32, _CODEPAGESIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CODEPAGESIZE;
#[doc = "`read()` method returns [codepagesize::R](codepagesize::R) reader structure"]
impl crate::Readable for CODEPAGESIZE {}
#[doc = "Code memory page size"]
pub mod codepagesize;
#[doc = "Code memory size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [codesize](codesize) module"]
pub type CODESIZE = crate::Reg<u32, _CODESIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CODESIZE;
#[doc = "`read()` method returns [codesize::R](codesize::R) reader structure"]
impl crate::Readable for CODESIZE {}
#[doc = "Code memory size"]
pub mod codesize;
#[doc = "Device type\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devicetype](devicetype) module"]
pub type DEVICETYPE = crate::Reg<u32, _DEVICETYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICETYPE;
#[doc = "`read()` method returns [devicetype::R](devicetype::R) reader structure"]
impl crate::Readable for DEVICETYPE {}
#[doc = "Device type"]
pub mod devicetype;

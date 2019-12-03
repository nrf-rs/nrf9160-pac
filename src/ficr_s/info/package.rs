#[doc = "Reader of register PACKAGE"]
pub type R = crate::R<u32, super::PACKAGE>;
#[doc = "Package option\n\nValue on reset: 8192"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKAGE_A {
    #[doc = "8192: CCxx - 236 ball wlCSP"]
    CC,
}
impl From<PACKAGE_A> for u32 {
    #[inline(always)]
    fn from(variant: PACKAGE_A) -> Self {
        match variant {
            PACKAGE_A::CC => 8192,
        }
    }
}
#[doc = "Reader of field `PACKAGE`"]
pub type PACKAGE_R = crate::R<u32, PACKAGE_A>;
impl PACKAGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PACKAGE_A> {
        use crate::Variant::*;
        match self.bits {
            8192 => Val(PACKAGE_A::CC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CC`"]
    #[inline(always)]
    pub fn is_cc(&self) -> bool {
        *self == PACKAGE_A::CC
    }
}
impl R {
    #[doc = "Bits 0:31 - Package option"]
    #[inline(always)]
    pub fn package(&self) -> PACKAGE_R {
        PACKAGE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

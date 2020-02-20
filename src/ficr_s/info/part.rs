#[doc = "Reader of register PART"]
pub type R = crate::R<u32, super::PART>;
#[doc = "Part code\n\nValue on reset: 37216"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PART_A {
    #[doc = "37216: nRF9160"]
    N9160 = 37216,
}
impl From<PART_A> for u32 {
    #[inline(always)]
    fn from(variant: PART_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PART`"]
pub type PART_R = crate::R<u32, PART_A>;
impl PART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PART_A> {
        use crate::Variant::*;
        match self.bits {
            37216 => Val(PART_A::N9160),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `N9160`"]
    #[inline(always)]
    pub fn is_n9160(&self) -> bool {
        *self == PART_A::N9160
    }
}
impl R {
    #[doc = "Bits 0:31 - Part code"]
    #[inline(always)]
    pub fn part(&self) -> PART_R {
        PART_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

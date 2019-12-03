#[doc = "Reader of register FLASH"]
pub type R = crate::R<u32, super::FLASH>;
#[doc = "Flash variant\n\nValue on reset: 1024"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_A {
    #[doc = "1024: 1 MByte FLASH"]
    K1024,
}
impl From<FLASH_A> for u32 {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        match variant {
            FLASH_A::K1024 => 1024,
        }
    }
}
#[doc = "Reader of field `FLASH`"]
pub type FLASH_R = crate::R<u32, FLASH_A>;
impl FLASH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, FLASH_A> {
        use crate::Variant::*;
        match self.bits {
            1024 => Val(FLASH_A::K1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `K1024`"]
    #[inline(always)]
    pub fn is_k1024(&self) -> bool {
        *self == FLASH_A::K1024
    }
}
impl R {
    #[doc = "Bits 0:31 - Flash variant"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

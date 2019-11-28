#[doc = "Reader of register RAM"]
pub type R = crate::R<u32, super::RAM>;
#[doc = "RAM variant\n\nValue on reset: 256"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_A {
    #[doc = "256: 256  kByte RAM"]
    K256,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED,
}
impl From<RAM_A> for u32 {
    #[inline(always)]
    fn from(variant: RAM_A) -> Self {
        match variant {
            RAM_A::K256 => 256,
            RAM_A::UNSPECIFIED => 4294967295,
        }
    }
}
#[doc = "Reader of field `RAM`"]
pub type RAM_R = crate::R<u32, RAM_A>;
impl RAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, RAM_A> {
        use crate::Variant::*;
        match self.bits {
            256 => Val(RAM_A::K256),
            4294967295 => Val(RAM_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `K256`"]
    #[inline(always)]
    pub fn is_k256(&self) -> bool {
        *self == RAM_A::K256
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == RAM_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - RAM variant"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

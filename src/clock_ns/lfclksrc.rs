#[doc = "Reader of register LFCLKSRC"]
pub type R = crate::R<u32, super::LFCLKSRC>;
#[doc = "Writer for register LFCLKSRC"]
pub type W = crate::W<u32, super::LFCLKSRC>;
#[doc = "Register LFCLKSRC `reset()`'s with value 0x01"]
impl crate::ResetValue for super::LFCLKSRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Clock source\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: Reserved for future use (equals selecting LFRC)"]
    RFU = 0,
    #[doc = "1: 32.768 kHz RC oscillator"]
    LFRC = 1,
    #[doc = "2: 32.768 kHz crystal oscillator"]
    LFXO = 2,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_A::RFU),
            1 => Val(SRC_A::LFRC),
            2 => Val(SRC_A::LFXO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RFU`"]
    #[inline(always)]
    pub fn is_rfu(&self) -> bool {
        *self == SRC_A::RFU
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == SRC_A::LFRC
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == SRC_A::LFXO
    }
}
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reserved for future use (equals selecting LFRC)"]
    #[inline(always)]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SRC_A::RFU)
    }
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(SRC_A::LFRC)
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(SRC_A::LFXO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}

#[doc = "Reader of register XOSC32M"]
pub type R = crate::R<u32, super::XOSC32M>;
#[doc = "Writer for register XOSC32M"]
pub type W = crate::W<u32, super::XOSC32M>;
#[doc = "Register XOSC32M `reset()`'s with value 0xffff_ffcf"]
impl crate::ResetValue for super::XOSC32M {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffcf
    }
}
#[doc = "Reader of field `CTRL`"]
pub type CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTRL`"]
pub struct CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Pierce current DAC control signals"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pierce current DAC control signals"]
    #[inline(always)]
    pub fn ctrl(&mut self) -> CTRL_W {
        CTRL_W { w: self }
    }
}

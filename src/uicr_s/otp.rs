#[doc = "Reader of register OTP[%s]"]
pub type R = crate::R<u32, super::OTP>;
#[doc = "Writer for register OTP[%s]"]
pub type W = crate::W<u32, super::OTP>;
#[doc = "Register OTP[%s] `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::OTP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `OTP`"]
pub type OTP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OTP`"]
pub struct OTP_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bits \\[31+n*32:0+n*32\\] of OTP region"]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[31+n*32:0+n*32\\] of OTP region"]
    #[inline(always)]
    pub fn otp(&mut self) -> OTP_W {
        OTP_W { w: self }
    }
}

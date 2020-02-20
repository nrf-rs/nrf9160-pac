#[doc = "Writer for register CLOCKSTOP"]
pub type W = crate::W<u32, super::CLOCKSTOP>;
#[doc = "Register CLOCKSTOP `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCKSTOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_AW {
    #[doc = "1: Stop all trace and debug clocks."]
    STOP = 1,
}
impl From<STOP_AW> for bool {
    #[inline(always)]
    fn from(variant: STOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Stop all trace and debug clocks."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOP_AW::STOP)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
}

#[doc = "Writer for register ERASEALL"]
pub type W = crate::W<u32, super::ERASEALL>;
#[doc = "Register ERASEALL `reset()`'s with value 0"]
impl crate::ResetValue for super::ERASEALL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Erase all non-volatile memory including UICR registers. Note that erasing must be enabled by setting CONFIG.WEN = Een before the non-volatile memory can be erased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASEALL_AW {
    #[doc = "0: No operation"]
    NOOPERATION = 0,
    #[doc = "1: Start chip erase"]
    ERASE = 1,
}
impl From<ERASEALL_AW> for bool {
    #[inline(always)]
    fn from(variant: ERASEALL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ERASEALL`"]
pub struct ERASEALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERASEALL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(ERASEALL_AW::NOOPERATION)
    }
    #[doc = "Start chip erase"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASEALL_AW::ERASE)
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
    #[doc = "Bit 0 - Erase all non-volatile memory including UICR registers. Note that erasing must be enabled by setting CONFIG.WEN = Een before the non-volatile memory can be erased."]
    #[inline(always)]
    pub fn eraseall(&mut self) -> ERASEALL_W {
        ERASEALL_W { w: self }
    }
}

#[doc = "Writer for register TASKS_STARTRX"]
pub type W = crate::W<u32, super::TASKS_STARTRX>;
#[doc = "Register TASKS_STARTRX `reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_STARTRX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Start UART receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_STARTRX_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_STARTRX_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_STARTRX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TASKS_STARTRX`"]
pub struct TASKS_STARTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_STARTRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_STARTRX_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_STARTRX_AW::TRIGGER)
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
    #[doc = "Bit 0 - Start UART receiver"]
    #[inline(always)]
    pub fn tasks_startrx(&mut self) -> TASKS_STARTRX_W {
        TASKS_STARTRX_W { w: self }
    }
}

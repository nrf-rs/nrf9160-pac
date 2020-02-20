#[doc = "Writer for register TASKS_PREPARETX"]
pub type W = crate::W<u32, super::TASKS_PREPARETX>;
#[doc = "Register TASKS_PREPARETX `reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_PREPARETX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prepare the TWI slave to respond to a read command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_PREPARETX_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_PREPARETX_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_PREPARETX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TASKS_PREPARETX`"]
pub struct TASKS_PREPARETX_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_PREPARETX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_PREPARETX_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_PREPARETX_AW::TRIGGER)
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
    #[doc = "Bit 0 - Prepare the TWI slave to respond to a read command"]
    #[inline(always)]
    pub fn tasks_preparetx(&mut self) -> TASKS_PREPARETX_W {
        TASKS_PREPARETX_W { w: self }
    }
}

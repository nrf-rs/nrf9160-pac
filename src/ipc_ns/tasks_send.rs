#[doc = "Writer for register TASKS_SEND[%s]"]
pub type W = crate::W<u32, super::TASKS_SEND>;
#[doc = "Register TASKS_SEND[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_SEND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Trigger events on channel enabled in SEND_CNF\\[n\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_SEND_AW {
    #[doc = "1: Trigger task"]
    TRIGGER,
}
impl From<TASKS_SEND_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_SEND_AW) -> Self {
        match variant {
            TASKS_SEND_AW::TRIGGER => true,
        }
    }
}
#[doc = "Write proxy for field `TASKS_SEND`"]
pub struct TASKS_SEND_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_SEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_SEND_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_SEND_AW::TRIGGER)
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
    #[doc = "Bit 0 - Trigger events on channel enabled in SEND_CNF\\[n\\]."]
    #[inline(always)]
    pub fn tasks_send(&mut self) -> TASKS_SEND_W {
        TASKS_SEND_W { w: self }
    }
}

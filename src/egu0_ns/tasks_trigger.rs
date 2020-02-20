#[doc = "Writer for register TASKS_TRIGGER[%s]"]
pub type W = crate::W<u32, super::TASKS_TRIGGER>;
#[doc = "Register TASKS_TRIGGER[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_TRIGGER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Trigger n for triggering the corresponding TRIGGERED\\[n\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_TRIGGER_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_TRIGGER_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_TRIGGER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TASKS_TRIGGER`"]
pub struct TASKS_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_TRIGGER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_TRIGGER_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_TRIGGER_AW::TRIGGER)
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
    #[doc = "Bit 0 - Trigger n for triggering the corresponding TRIGGERED\\[n\\]
event"]
    #[inline(always)]
    pub fn tasks_trigger(&mut self) -> TASKS_TRIGGER_W {
        TASKS_TRIGGER_W { w: self }
    }
}

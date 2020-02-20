#[doc = "Writer for register TASKS_SEQSTART[%s]"]
pub type W = crate::W<u32, super::TASKS_SEQSTART>;
#[doc = "Register TASKS_SEQSTART[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_SEQSTART {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_SEQSTART_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_SEQSTART_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_SEQSTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TASKS_SEQSTART`"]
pub struct TASKS_SEQSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_SEQSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_SEQSTART_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_SEQSTART_AW::TRIGGER)
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
    #[doc = "Bit 0 - Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start if not running."]
    #[inline(always)]
    pub fn tasks_seqstart(&mut self) -> TASKS_SEQSTART_W {
        TASKS_SEQSTART_W { w: self }
    }
}

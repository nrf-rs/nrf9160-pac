#[doc = "Reader of register EVENTS_HFCLKSTARTED"]
pub type R = crate::R<u32, super::EVENTS_HFCLKSTARTED>;
#[doc = "Writer for register EVENTS_HFCLKSTARTED"]
pub type W = crate::W<u32, super::EVENTS_HFCLKSTARTED>;
#[doc = "Register EVENTS_HFCLKSTARTED `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_HFCLKSTARTED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "HFCLK oscillator started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_HFCLKSTARTED_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_HFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_HFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTS_HFCLKSTARTED`"]
pub type EVENTS_HFCLKSTARTED_R = crate::R<bool, EVENTS_HFCLKSTARTED_A>;
impl EVENTS_HFCLKSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_HFCLKSTARTED_A {
        match self.bits {
            false => EVENTS_HFCLKSTARTED_A::NOTGENERATED,
            true => EVENTS_HFCLKSTARTED_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_HFCLKSTARTED_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_HFCLKSTARTED_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_HFCLKSTARTED`"]
pub struct EVENTS_HFCLKSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_HFCLKSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_HFCLKSTARTED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_HFCLKSTARTED_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_HFCLKSTARTED_A::GENERATED)
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
impl R {
    #[doc = "Bit 0 - HFCLK oscillator started"]
    #[inline(always)]
    pub fn events_hfclkstarted(&self) -> EVENTS_HFCLKSTARTED_R {
        EVENTS_HFCLKSTARTED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFCLK oscillator started"]
    #[inline(always)]
    pub fn events_hfclkstarted(&mut self) -> EVENTS_HFCLKSTARTED_W {
        EVENTS_HFCLKSTARTED_W { w: self }
    }
}

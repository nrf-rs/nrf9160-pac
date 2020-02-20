#[doc = "Reader of register EVENTS_TXSTARTED"]
pub type R = crate::R<u32, super::EVENTS_TXSTARTED>;
#[doc = "Writer for register EVENTS_TXSTARTED"]
pub type W = crate::W<u32, super::EVENTS_TXSTARTED>;
#[doc = "Register EVENTS_TXSTARTED `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_TXSTARTED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit sequence started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_TXSTARTED_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_TXSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_TXSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTS_TXSTARTED`"]
pub type EVENTS_TXSTARTED_R = crate::R<bool, EVENTS_TXSTARTED_A>;
impl EVENTS_TXSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_TXSTARTED_A {
        match self.bits {
            false => EVENTS_TXSTARTED_A::NOTGENERATED,
            true => EVENTS_TXSTARTED_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_TXSTARTED_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_TXSTARTED_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_TXSTARTED`"]
pub struct EVENTS_TXSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_TXSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_TXSTARTED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_TXSTARTED_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_TXSTARTED_A::GENERATED)
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
    #[doc = "Bit 0 - Transmit sequence started"]
    #[inline(always)]
    pub fn events_txstarted(&self) -> EVENTS_TXSTARTED_R {
        EVENTS_TXSTARTED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit sequence started"]
    #[inline(always)]
    pub fn events_txstarted(&mut self) -> EVENTS_TXSTARTED_W {
        EVENTS_TXSTARTED_W { w: self }
    }
}

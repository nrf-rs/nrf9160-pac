#[doc = "Reader of register EVENTS_ACQUIRED"]
pub type R = crate::R<u32, super::EVENTS_ACQUIRED>;
#[doc = "Writer for register EVENTS_ACQUIRED"]
pub type W = crate::W<u32, super::EVENTS_ACQUIRED>;
#[doc = "Register EVENTS_ACQUIRED `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_ACQUIRED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Semaphore acquired\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_ACQUIRED_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_ACQUIRED_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_ACQUIRED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTS_ACQUIRED`"]
pub type EVENTS_ACQUIRED_R = crate::R<bool, EVENTS_ACQUIRED_A>;
impl EVENTS_ACQUIRED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_ACQUIRED_A {
        match self.bits {
            false => EVENTS_ACQUIRED_A::NOTGENERATED,
            true => EVENTS_ACQUIRED_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_ACQUIRED_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_ACQUIRED_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_ACQUIRED`"]
pub struct EVENTS_ACQUIRED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_ACQUIRED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_ACQUIRED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_ACQUIRED_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_ACQUIRED_A::GENERATED)
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
    #[doc = "Bit 0 - Semaphore acquired"]
    #[inline(always)]
    pub fn events_acquired(&self) -> EVENTS_ACQUIRED_R {
        EVENTS_ACQUIRED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Semaphore acquired"]
    #[inline(always)]
    pub fn events_acquired(&mut self) -> EVENTS_ACQUIRED_W {
        EVENTS_ACQUIRED_W { w: self }
    }
}

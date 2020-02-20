#[doc = "Reader of register EVENTS_CALIBRATEDONE"]
pub type R = crate::R<u32, super::EVENTS_CALIBRATEDONE>;
#[doc = "Writer for register EVENTS_CALIBRATEDONE"]
pub type W = crate::W<u32, super::EVENTS_CALIBRATEDONE>;
#[doc = "Register EVENTS_CALIBRATEDONE `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_CALIBRATEDONE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Calibration is complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_CALIBRATEDONE_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_CALIBRATEDONE_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_CALIBRATEDONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTS_CALIBRATEDONE`"]
pub type EVENTS_CALIBRATEDONE_R = crate::R<bool, EVENTS_CALIBRATEDONE_A>;
impl EVENTS_CALIBRATEDONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_CALIBRATEDONE_A {
        match self.bits {
            false => EVENTS_CALIBRATEDONE_A::NOTGENERATED,
            true => EVENTS_CALIBRATEDONE_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_CALIBRATEDONE_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_CALIBRATEDONE_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_CALIBRATEDONE`"]
pub struct EVENTS_CALIBRATEDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_CALIBRATEDONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_CALIBRATEDONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_CALIBRATEDONE_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_CALIBRATEDONE_A::GENERATED)
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
    #[doc = "Bit 0 - Calibration is complete"]
    #[inline(always)]
    pub fn events_calibratedone(&self) -> EVENTS_CALIBRATEDONE_R {
        EVENTS_CALIBRATEDONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration is complete"]
    #[inline(always)]
    pub fn events_calibratedone(&mut self) -> EVENTS_CALIBRATEDONE_W {
        EVENTS_CALIBRATEDONE_W { w: self }
    }
}

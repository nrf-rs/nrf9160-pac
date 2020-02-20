#[doc = "Reader of register EVENTS_END"]
pub type R = crate::R<u32, super::EVENTS_END>;
#[doc = "Writer for register EVENTS_END"]
pub type W = crate::W<u32, super::EVENTS_END>;
#[doc = "Register EVENTS_END `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_END {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The ADC has filled up the Result buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_END_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_END_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_END_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTS_END`"]
pub type EVENTS_END_R = crate::R<bool, EVENTS_END_A>;
impl EVENTS_END_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_END_A {
        match self.bits {
            false => EVENTS_END_A::NOTGENERATED,
            true => EVENTS_END_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_END_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_END_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_END`"]
pub struct EVENTS_END_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_END_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_END_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_END_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_END_A::GENERATED)
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
    #[doc = "Bit 0 - The ADC has filled up the Result buffer"]
    #[inline(always)]
    pub fn events_end(&self) -> EVENTS_END_R {
        EVENTS_END_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The ADC has filled up the Result buffer"]
    #[inline(always)]
    pub fn events_end(&mut self) -> EVENTS_END_W {
        EVENTS_END_W { w: self }
    }
}

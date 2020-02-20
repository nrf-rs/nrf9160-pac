#[doc = "Reader of register EVENTS_ERROR"]
pub type R = crate::R<u32, super::EVENTS_ERROR>;
#[doc = "Writer for register EVENTS_ERROR"]
pub type W = crate::W<u32, super::EVENTS_ERROR>;
#[doc = "Register EVENTS_ERROR `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_ERROR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Error detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_ERROR_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTS_ERROR`"]
pub type EVENTS_ERROR_R = crate::R<bool, EVENTS_ERROR_A>;
impl EVENTS_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_ERROR_A {
        match self.bits {
            false => EVENTS_ERROR_A::NOTGENERATED,
            true => EVENTS_ERROR_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_ERROR_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_ERROR_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_ERROR`"]
pub struct EVENTS_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_ERROR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_ERROR_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_ERROR_A::GENERATED)
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
    #[doc = "Bit 0 - Error detected"]
    #[inline(always)]
    pub fn events_error(&self) -> EVENTS_ERROR_R {
        EVENTS_ERROR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error detected"]
    #[inline(always)]
    pub fn events_error(&mut self) -> EVENTS_ERROR_W {
        EVENTS_ERROR_W { w: self }
    }
}

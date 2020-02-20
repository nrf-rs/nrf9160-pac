#[doc = "Reader of register EVENTS_NCTS"]
pub type R = crate::R<u32, super::EVENTS_NCTS>;
#[doc = "Writer for register EVENTS_NCTS"]
pub type W = crate::W<u32, super::EVENTS_NCTS>;
#[doc = "Register EVENTS_NCTS `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_NCTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CTS is deactivated (set high). Not Clear To Send.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_NCTS_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_NCTS_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_NCTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTS_NCTS`"]
pub type EVENTS_NCTS_R = crate::R<bool, EVENTS_NCTS_A>;
impl EVENTS_NCTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_NCTS_A {
        match self.bits {
            false => EVENTS_NCTS_A::NOTGENERATED,
            true => EVENTS_NCTS_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_NCTS_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_NCTS_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_NCTS`"]
pub struct EVENTS_NCTS_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_NCTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_NCTS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_NCTS_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_NCTS_A::GENERATED)
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
    #[doc = "Bit 0 - CTS is deactivated (set high). Not Clear To Send."]
    #[inline(always)]
    pub fn events_ncts(&self) -> EVENTS_NCTS_R {
        EVENTS_NCTS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTS is deactivated (set high). Not Clear To Send."]
    #[inline(always)]
    pub fn events_ncts(&mut self) -> EVENTS_NCTS_W {
        EVENTS_NCTS_W { w: self }
    }
}

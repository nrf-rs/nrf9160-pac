#[doc = "Reader of register EVENTS_PORT"]
pub type R = crate::R<u32, super::EVENTS_PORT>;
#[doc = "Writer for register EVENTS_PORT"]
pub type W = crate::W<u32, super::EVENTS_PORT>;
#[doc = "Register EVENTS_PORT `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_PORT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_PORT_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_PORT_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_PORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTS_PORT`"]
pub type EVENTS_PORT_R = crate::R<bool, EVENTS_PORT_A>;
impl EVENTS_PORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_PORT_A {
        match self.bits {
            false => EVENTS_PORT_A::NOTGENERATED,
            true => EVENTS_PORT_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_PORT_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_PORT_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_PORT`"]
pub struct EVENTS_PORT_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_PORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_PORT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_PORT_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_PORT_A::GENERATED)
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
    #[doc = "Bit 0 - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
    #[inline(always)]
    pub fn events_port(&self) -> EVENTS_PORT_R {
        EVENTS_PORT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
    #[inline(always)]
    pub fn events_port(&mut self) -> EVENTS_PORT_W {
        EVENTS_PORT_W { w: self }
    }
}

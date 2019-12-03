#[doc = "Reader of register EVENTS_PERIPHACCERR"]
pub type R = crate::R<u32, super::EVENTS_PERIPHACCERR>;
#[doc = "Writer for register EVENTS_PERIPHACCERR"]
pub type W = crate::W<u32, super::EVENTS_PERIPHACCERR>;
#[doc = "Register EVENTS_PERIPHACCERR `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_PERIPHACCERR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "A security violation has been detected on one or several peripherals\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_PERIPHACCERR_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_PERIPHACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_PERIPHACCERR_A) -> Self {
        match variant {
            EVENTS_PERIPHACCERR_A::NOTGENERATED => false,
            EVENTS_PERIPHACCERR_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_PERIPHACCERR`"]
pub type EVENTS_PERIPHACCERR_R = crate::R<bool, EVENTS_PERIPHACCERR_A>;
impl EVENTS_PERIPHACCERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_PERIPHACCERR_A {
        match self.bits {
            false => EVENTS_PERIPHACCERR_A::NOTGENERATED,
            true => EVENTS_PERIPHACCERR_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_PERIPHACCERR_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_PERIPHACCERR_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_PERIPHACCERR`"]
pub struct EVENTS_PERIPHACCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_PERIPHACCERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_PERIPHACCERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_PERIPHACCERR_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_PERIPHACCERR_A::GENERATED)
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
    #[doc = "Bit 0 - A security violation has been detected on one or several peripherals"]
    #[inline(always)]
    pub fn events_periphaccerr(&self) -> EVENTS_PERIPHACCERR_R {
        EVENTS_PERIPHACCERR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A security violation has been detected on one or several peripherals"]
    #[inline(always)]
    pub fn events_periphaccerr(&mut self) -> EVENTS_PERIPHACCERR_W {
        EVENTS_PERIPHACCERR_W { w: self }
    }
}

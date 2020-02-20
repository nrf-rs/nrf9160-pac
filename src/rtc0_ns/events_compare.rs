#[doc = "Reader of register EVENTS_COMPARE[%s]"]
pub type R = crate::R<u32, super::EVENTS_COMPARE>;
#[doc = "Writer for register EVENTS_COMPARE[%s]"]
pub type W = crate::W<u32, super::EVENTS_COMPARE>;
#[doc = "Register EVENTS_COMPARE[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_COMPARE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Compare event on CC\\[n\\]
match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_COMPARE_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_COMPARE_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_COMPARE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTS_COMPARE`"]
pub type EVENTS_COMPARE_R = crate::R<bool, EVENTS_COMPARE_A>;
impl EVENTS_COMPARE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_COMPARE_A {
        match self.bits {
            false => EVENTS_COMPARE_A::NOTGENERATED,
            true => EVENTS_COMPARE_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_COMPARE_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_COMPARE_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_COMPARE`"]
pub struct EVENTS_COMPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_COMPARE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_COMPARE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_COMPARE_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_COMPARE_A::GENERATED)
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
    #[doc = "Bit 0 - Compare event on CC\\[n\\]
match"]
    #[inline(always)]
    pub fn events_compare(&self) -> EVENTS_COMPARE_R {
        EVENTS_COMPARE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare event on CC\\[n\\]
match"]
    #[inline(always)]
    pub fn events_compare(&mut self) -> EVENTS_COMPARE_W {
        EVENTS_COMPARE_W { w: self }
    }
}

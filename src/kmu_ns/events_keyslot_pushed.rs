#[doc = "Reader of register EVENTS_KEYSLOT_PUSHED"]
pub type R = crate::R<u32, super::EVENTS_KEYSLOT_PUSHED>;
#[doc = "Writer for register EVENTS_KEYSLOT_PUSHED"]
pub type W = crate::W<u32, super::EVENTS_KEYSLOT_PUSHED>;
#[doc = "Register EVENTS_KEYSLOT_PUSHED `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_KEYSLOT_PUSHED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Key successfully pushed over secure APB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_KEYSLOT_PUSHED_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_KEYSLOT_PUSHED_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_KEYSLOT_PUSHED_A) -> Self {
        match variant {
            EVENTS_KEYSLOT_PUSHED_A::NOTGENERATED => false,
            EVENTS_KEYSLOT_PUSHED_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_KEYSLOT_PUSHED`"]
pub type EVENTS_KEYSLOT_PUSHED_R = crate::R<bool, EVENTS_KEYSLOT_PUSHED_A>;
impl EVENTS_KEYSLOT_PUSHED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_KEYSLOT_PUSHED_A {
        match self.bits {
            false => EVENTS_KEYSLOT_PUSHED_A::NOTGENERATED,
            true => EVENTS_KEYSLOT_PUSHED_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_KEYSLOT_PUSHED_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_KEYSLOT_PUSHED_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_KEYSLOT_PUSHED`"]
pub struct EVENTS_KEYSLOT_PUSHED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_KEYSLOT_PUSHED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_KEYSLOT_PUSHED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_KEYSLOT_PUSHED_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_KEYSLOT_PUSHED_A::GENERATED)
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
    #[doc = "Bit 0 - Key successfully pushed over secure APB"]
    #[inline(always)]
    pub fn events_keyslot_pushed(&self) -> EVENTS_KEYSLOT_PUSHED_R {
        EVENTS_KEYSLOT_PUSHED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key successfully pushed over secure APB"]
    #[inline(always)]
    pub fn events_keyslot_pushed(&mut self) -> EVENTS_KEYSLOT_PUSHED_W {
        EVENTS_KEYSLOT_PUSHED_W { w: self }
    }
}

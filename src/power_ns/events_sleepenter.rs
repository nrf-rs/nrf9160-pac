#[doc = "Reader of register EVENTS_SLEEPENTER"]
pub type R = crate::R<u32, super::EVENTS_SLEEPENTER>;
#[doc = "Writer for register EVENTS_SLEEPENTER"]
pub type W = crate::W<u32, super::EVENTS_SLEEPENTER>;
#[doc = "Register EVENTS_SLEEPENTER `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_SLEEPENTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CPU entered WFI/WFE sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_SLEEPENTER_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_SLEEPENTER_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_SLEEPENTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EVENTS_SLEEPENTER`"]
pub type EVENTS_SLEEPENTER_R = crate::R<bool, EVENTS_SLEEPENTER_A>;
impl EVENTS_SLEEPENTER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_SLEEPENTER_A {
        match self.bits {
            false => EVENTS_SLEEPENTER_A::NOTGENERATED,
            true => EVENTS_SLEEPENTER_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_SLEEPENTER_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_SLEEPENTER_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_SLEEPENTER`"]
pub struct EVENTS_SLEEPENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_SLEEPENTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_SLEEPENTER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_SLEEPENTER_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_SLEEPENTER_A::GENERATED)
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
    #[doc = "Bit 0 - CPU entered WFI/WFE sleep"]
    #[inline(always)]
    pub fn events_sleepenter(&self) -> EVENTS_SLEEPENTER_R {
        EVENTS_SLEEPENTER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU entered WFI/WFE sleep"]
    #[inline(always)]
    pub fn events_sleepenter(&mut self) -> EVENTS_SLEEPENTER_W {
        EVENTS_SLEEPENTER_W { w: self }
    }
}

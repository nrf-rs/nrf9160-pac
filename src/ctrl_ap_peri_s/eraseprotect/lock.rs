#[doc = "Reader of register LOCK"]
pub type R = crate::R<u32, super::LOCK>;
#[doc = "Writer for register LOCK"]
pub type W = crate::W<u32, super::LOCK>;
#[doc = "Register LOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable or disable the ERASEALL mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASEPROTECTLOCK_A {
    #[doc = "0: ERASEALL can be issued"]
    UNLOCKED,
    #[doc = "1: ERASEALL is locked"]
    LOCKED,
}
impl From<ERASEPROTECTLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: ERASEPROTECTLOCK_A) -> Self {
        match variant {
            ERASEPROTECTLOCK_A::UNLOCKED => false,
            ERASEPROTECTLOCK_A::LOCKED => true,
        }
    }
}
#[doc = "Reader of field `ERASEPROTECTLOCK`"]
pub type ERASEPROTECTLOCK_R = crate::R<bool, ERASEPROTECTLOCK_A>;
impl ERASEPROTECTLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERASEPROTECTLOCK_A {
        match self.bits {
            false => ERASEPROTECTLOCK_A::UNLOCKED,
            true => ERASEPROTECTLOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == ERASEPROTECTLOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == ERASEPROTECTLOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `ERASEPROTECTLOCK`"]
pub struct ERASEPROTECTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEPROTECTLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERASEPROTECTLOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ERASEALL can be issued"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(ERASEPROTECTLOCK_A::UNLOCKED)
    }
    #[doc = "ERASEALL is locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(ERASEPROTECTLOCK_A::LOCKED)
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
    #[doc = "Bit 0 - Enable or disable the ERASEALL mechanism"]
    #[inline(always)]
    pub fn eraseprotectlock(&self) -> ERASEPROTECTLOCK_R {
        ERASEPROTECTLOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable the ERASEALL mechanism"]
    #[inline(always)]
    pub fn eraseprotectlock(&mut self) -> ERASEPROTECTLOCK_W {
        ERASEPROTECTLOCK_W { w: self }
    }
}

#[doc = "Reader of register FORCEONNVM"]
pub type R = crate::R<u32, super::FORCEONNVM>;
#[doc = "Writer for register FORCEONNVM"]
pub type W = crate::W<u32, super::FORCEONNVM>;
#[doc = "Register FORCEONNVM `reset()`'s with value 0"]
impl crate::ResetValue for super::FORCEONNVM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Force on all NVM supplies. Also see the internal section in the NVMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEONNVM_A {
    #[doc = "0: Do not force on NVM supply"]
    DONOTFORCEON,
    #[doc = "1: Force on NVM supply"]
    FORCEON,
}
impl From<FORCEONNVM_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEONNVM_A) -> Self {
        match variant {
            FORCEONNVM_A::DONOTFORCEON => false,
            FORCEONNVM_A::FORCEON => true,
        }
    }
}
#[doc = "Reader of field `FORCEONNVM`"]
pub type FORCEONNVM_R = crate::R<bool, FORCEONNVM_A>;
impl FORCEONNVM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEONNVM_A {
        match self.bits {
            false => FORCEONNVM_A::DONOTFORCEON,
            true => FORCEONNVM_A::FORCEON,
        }
    }
    #[doc = "Checks if the value of the field is `DONOTFORCEON`"]
    #[inline(always)]
    pub fn is_do_not_force_on(&self) -> bool {
        *self == FORCEONNVM_A::DONOTFORCEON
    }
    #[doc = "Checks if the value of the field is `FORCEON`"]
    #[inline(always)]
    pub fn is_force_on(&self) -> bool {
        *self == FORCEONNVM_A::FORCEON
    }
}
#[doc = "Write proxy for field `FORCEONNVM`"]
pub struct FORCEONNVM_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEONNVM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEONNVM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not force on NVM supply"]
    #[inline(always)]
    pub fn do_not_force_on(self) -> &'a mut W {
        self.variant(FORCEONNVM_A::DONOTFORCEON)
    }
    #[doc = "Force on NVM supply"]
    #[inline(always)]
    pub fn force_on(self) -> &'a mut W {
        self.variant(FORCEONNVM_A::FORCEON)
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
    #[doc = "Bit 0 - Force on all NVM supplies. Also see the internal section in the NVMC chapter."]
    #[inline(always)]
    pub fn forceonnvm(&self) -> FORCEONNVM_R {
        FORCEONNVM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force on all NVM supplies. Also see the internal section in the NVMC chapter."]
    #[inline(always)]
    pub fn forceonnvm(&mut self) -> FORCEONNVM_W {
        FORCEONNVM_W { w: self }
    }
}

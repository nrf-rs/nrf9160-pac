#[doc = "Reader of register LIMITH"]
pub type R = crate::R<u32, super::LIMITH>;
#[doc = "Writer for register LIMITH"]
pub type W = crate::W<u32, super::LIMITH>;
#[doc = "Register LIMITH `reset()`'s with value 0"]
impl crate::ResetValue for super::LIMITH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Last results is equal or above CH\\[n\\].LIMIT.HIGH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIMITH_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LIMITH`"]
pub type LIMITH_R = crate::R<bool, LIMITH_A>;
impl LIMITH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMITH_A {
        match self.bits {
            false => LIMITH_A::NOTGENERATED,
            true => LIMITH_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == LIMITH_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == LIMITH_A::GENERATED
    }
}
#[doc = "Write proxy for field `LIMITH`"]
pub struct LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LIMITH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(LIMITH_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(LIMITH_A::GENERATED)
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
    #[doc = "Bit 0 - Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
    #[inline(always)]
    pub fn limith(&self) -> LIMITH_R {
        LIMITH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
    #[inline(always)]
    pub fn limith(&mut self) -> LIMITH_W {
        LIMITH_W { w: self }
    }
}

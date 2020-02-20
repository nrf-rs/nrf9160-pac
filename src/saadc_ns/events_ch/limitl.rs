#[doc = "Reader of register LIMITL"]
pub type R = crate::R<u32, super::LIMITL>;
#[doc = "Writer for register LIMITL"]
pub type W = crate::W<u32, super::LIMITL>;
#[doc = "Register LIMITL `reset()`'s with value 0"]
impl crate::ResetValue for super::LIMITL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Last results is equal or below CH\\[n\\].LIMIT.LOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIMITL_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LIMITL`"]
pub type LIMITL_R = crate::R<bool, LIMITL_A>;
impl LIMITL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMITL_A {
        match self.bits {
            false => LIMITL_A::NOTGENERATED,
            true => LIMITL_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == LIMITL_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == LIMITL_A::GENERATED
    }
}
#[doc = "Write proxy for field `LIMITL`"]
pub struct LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LIMITL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(LIMITL_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(LIMITL_A::GENERATED)
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
    #[doc = "Bit 0 - Last results is equal or below CH\\[n\\].LIMIT.LOW"]
    #[inline(always)]
    pub fn limitl(&self) -> LIMITL_R {
        LIMITL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Last results is equal or below CH\\[n\\].LIMIT.LOW"]
    #[inline(always)]
    pub fn limitl(&mut self) -> LIMITL_W {
        LIMITL_W { w: self }
    }
}

#[doc = "Reader of register ONESHOTEN[%s]"]
pub type R = crate::R<u32, super::ONESHOTEN>;
#[doc = "Writer for register ONESHOTEN[%s]"]
pub type W = crate::W<u32, super::ONESHOTEN>;
#[doc = "Register ONESHOTEN[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::ONESHOTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable one-shot operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONESHOTEN_A {
    #[doc = "0: Disable one-shot operation"]
    DISABLE = 0,
    #[doc = "1: Enable one-shot operation"]
    ENABLE = 1,
}
impl From<ONESHOTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ONESHOTEN`"]
pub type ONESHOTEN_R = crate::R<bool, ONESHOTEN_A>;
impl ONESHOTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONESHOTEN_A {
        match self.bits {
            false => ONESHOTEN_A::DISABLE,
            true => ONESHOTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ONESHOTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ONESHOTEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `ONESHOTEN`"]
pub struct ONESHOTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ONESHOTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONESHOTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable one-shot operation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ONESHOTEN_A::DISABLE)
    }
    #[doc = "Enable one-shot operation"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ONESHOTEN_A::ENABLE)
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
    #[doc = "Bit 0 - Enable one-shot operation"]
    #[inline(always)]
    pub fn oneshoten(&self) -> ONESHOTEN_R {
        ONESHOTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable one-shot operation"]
    #[inline(always)]
    pub fn oneshoten(&mut self) -> ONESHOTEN_W {
        ONESHOTEN_W { w: self }
    }
}

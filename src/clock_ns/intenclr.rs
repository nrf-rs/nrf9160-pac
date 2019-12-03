#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u32, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u32, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write '1' to disable interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<HFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: HFCLKSTARTED_A) -> Self {
        match variant {
            HFCLKSTARTED_A::DISABLED => false,
            HFCLKSTARTED_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `HFCLKSTARTED`"]
pub type HFCLKSTARTED_R = crate::R<bool, HFCLKSTARTED_A>;
impl HFCLKSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLKSTARTED_A {
        match self.bits {
            false => HFCLKSTARTED_A::DISABLED,
            true => HFCLKSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HFCLKSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HFCLKSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKSTARTED_AW {
    #[doc = "1: Disable"]
    CLEAR,
}
impl From<HFCLKSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: HFCLKSTARTED_AW) -> Self {
        match variant {
            HFCLKSTARTED_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `HFCLKSTARTED`"]
pub struct HFCLKSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> HFCLKSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFCLKSTARTED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(HFCLKSTARTED_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFCLKSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<LFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: LFCLKSTARTED_A) -> Self {
        match variant {
            LFCLKSTARTED_A::DISABLED => false,
            LFCLKSTARTED_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `LFCLKSTARTED`"]
pub type LFCLKSTARTED_R = crate::R<bool, LFCLKSTARTED_A>;
impl LFCLKSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFCLKSTARTED_A {
        match self.bits {
            false => LFCLKSTARTED_A::DISABLED,
            true => LFCLKSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFCLKSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LFCLKSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFCLKSTARTED_AW {
    #[doc = "1: Disable"]
    CLEAR,
}
impl From<LFCLKSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: LFCLKSTARTED_AW) -> Self {
        match variant {
            LFCLKSTARTED_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `LFCLKSTARTED`"]
pub struct LFCLKSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> LFCLKSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFCLKSTARTED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LFCLKSTARTED_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&self) -> HFCLKSTARTED_R {
        HFCLKSTARTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&self) -> LFCLKSTARTED_R {
        LFCLKSTARTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&mut self) -> HFCLKSTARTED_W {
        HFCLKSTARTED_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&mut self) -> LFCLKSTARTED_W {
        LFCLKSTARTED_W { w: self }
    }
}

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
#[doc = "Write '1' to disable interrupt for event RAMACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMACCERR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<RAMACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: RAMACCERR_A) -> Self {
        match variant {
            RAMACCERR_A::DISABLED => false,
            RAMACCERR_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `RAMACCERR`"]
pub type RAMACCERR_R = crate::R<bool, RAMACCERR_A>;
impl RAMACCERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMACCERR_A {
        match self.bits {
            false => RAMACCERR_A::DISABLED,
            true => RAMACCERR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RAMACCERR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RAMACCERR_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event RAMACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMACCERR_AW {
    #[doc = "1: Disable"]
    CLEAR,
}
impl From<RAMACCERR_AW> for bool {
    #[inline(always)]
    fn from(variant: RAMACCERR_AW) -> Self {
        match variant {
            RAMACCERR_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `RAMACCERR`"]
pub struct RAMACCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACCERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMACCERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RAMACCERR_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event FLASHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHACCERR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<FLASHACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHACCERR_A) -> Self {
        match variant {
            FLASHACCERR_A::DISABLED => false,
            FLASHACCERR_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `FLASHACCERR`"]
pub type FLASHACCERR_R = crate::R<bool, FLASHACCERR_A>;
impl FLASHACCERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHACCERR_A {
        match self.bits {
            false => FLASHACCERR_A::DISABLED,
            true => FLASHACCERR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHACCERR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHACCERR_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event FLASHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHACCERR_AW {
    #[doc = "1: Disable"]
    CLEAR,
}
impl From<FLASHACCERR_AW> for bool {
    #[inline(always)]
    fn from(variant: FLASHACCERR_AW) -> Self {
        match variant {
            FLASHACCERR_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `FLASHACCERR`"]
pub struct FLASHACCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHACCERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHACCERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FLASHACCERR_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event PERIPHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPHACCERR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<PERIPHACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: PERIPHACCERR_A) -> Self {
        match variant {
            PERIPHACCERR_A::DISABLED => false,
            PERIPHACCERR_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `PERIPHACCERR`"]
pub type PERIPHACCERR_R = crate::R<bool, PERIPHACCERR_A>;
impl PERIPHACCERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERIPHACCERR_A {
        match self.bits {
            false => PERIPHACCERR_A::DISABLED,
            true => PERIPHACCERR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PERIPHACCERR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PERIPHACCERR_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event PERIPHACCERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPHACCERR_AW {
    #[doc = "1: Disable"]
    CLEAR,
}
impl From<PERIPHACCERR_AW> for bool {
    #[inline(always)]
    fn from(variant: PERIPHACCERR_AW) -> Self {
        match variant {
            PERIPHACCERR_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `PERIPHACCERR`"]
pub struct PERIPHACCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPHACCERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERIPHACCERR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PERIPHACCERR_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event RAMACCERR"]
    #[inline(always)]
    pub fn ramaccerr(&self) -> RAMACCERR_R {
        RAMACCERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event FLASHACCERR"]
    #[inline(always)]
    pub fn flashaccerr(&self) -> FLASHACCERR_R {
        FLASHACCERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event PERIPHACCERR"]
    #[inline(always)]
    pub fn periphaccerr(&self) -> PERIPHACCERR_R {
        PERIPHACCERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event RAMACCERR"]
    #[inline(always)]
    pub fn ramaccerr(&mut self) -> RAMACCERR_W {
        RAMACCERR_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event FLASHACCERR"]
    #[inline(always)]
    pub fn flashaccerr(&mut self) -> FLASHACCERR_W {
        FLASHACCERR_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event PERIPHACCERR"]
    #[inline(always)]
    pub fn periphaccerr(&mut self) -> PERIPHACCERR_W {
        PERIPHACCERR_W { w: self }
    }
}

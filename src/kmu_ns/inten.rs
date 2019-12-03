#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable or disable interrupt for event KEYSLOT_PUSHED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_PUSHED_A {
    #[doc = "0: Disable"]
    DISABLED,
    #[doc = "1: Enable"]
    ENABLED,
}
impl From<KEYSLOT_PUSHED_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_PUSHED_A) -> Self {
        match variant {
            KEYSLOT_PUSHED_A::DISABLED => false,
            KEYSLOT_PUSHED_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `KEYSLOT_PUSHED`"]
pub type KEYSLOT_PUSHED_R = crate::R<bool, KEYSLOT_PUSHED_A>;
impl KEYSLOT_PUSHED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYSLOT_PUSHED_A {
        match self.bits {
            false => KEYSLOT_PUSHED_A::DISABLED,
            true => KEYSLOT_PUSHED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == KEYSLOT_PUSHED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == KEYSLOT_PUSHED_A::ENABLED
    }
}
#[doc = "Write proxy for field `KEYSLOT_PUSHED`"]
pub struct KEYSLOT_PUSHED_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSLOT_PUSHED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYSLOT_PUSHED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(KEYSLOT_PUSHED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(KEYSLOT_PUSHED_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event KEYSLOT_REVOKED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_REVOKED_A {
    #[doc = "0: Disable"]
    DISABLED,
    #[doc = "1: Enable"]
    ENABLED,
}
impl From<KEYSLOT_REVOKED_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_REVOKED_A) -> Self {
        match variant {
            KEYSLOT_REVOKED_A::DISABLED => false,
            KEYSLOT_REVOKED_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `KEYSLOT_REVOKED`"]
pub type KEYSLOT_REVOKED_R = crate::R<bool, KEYSLOT_REVOKED_A>;
impl KEYSLOT_REVOKED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYSLOT_REVOKED_A {
        match self.bits {
            false => KEYSLOT_REVOKED_A::DISABLED,
            true => KEYSLOT_REVOKED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == KEYSLOT_REVOKED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == KEYSLOT_REVOKED_A::ENABLED
    }
}
#[doc = "Write proxy for field `KEYSLOT_REVOKED`"]
pub struct KEYSLOT_REVOKED_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSLOT_REVOKED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYSLOT_REVOKED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(KEYSLOT_REVOKED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(KEYSLOT_REVOKED_A::ENABLED)
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
#[doc = "Enable or disable interrupt for event KEYSLOT_ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_ERROR_A {
    #[doc = "0: Disable"]
    DISABLED,
    #[doc = "1: Enable"]
    ENABLED,
}
impl From<KEYSLOT_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_ERROR_A) -> Self {
        match variant {
            KEYSLOT_ERROR_A::DISABLED => false,
            KEYSLOT_ERROR_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `KEYSLOT_ERROR`"]
pub type KEYSLOT_ERROR_R = crate::R<bool, KEYSLOT_ERROR_A>;
impl KEYSLOT_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYSLOT_ERROR_A {
        match self.bits {
            false => KEYSLOT_ERROR_A::DISABLED,
            true => KEYSLOT_ERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == KEYSLOT_ERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == KEYSLOT_ERROR_A::ENABLED
    }
}
#[doc = "Write proxy for field `KEYSLOT_ERROR`"]
pub struct KEYSLOT_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSLOT_ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYSLOT_ERROR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(KEYSLOT_ERROR_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(KEYSLOT_ERROR_A::ENABLED)
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
    #[doc = "Bit 0 - Enable or disable interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    pub fn keyslot_pushed(&self) -> KEYSLOT_PUSHED_R {
        KEYSLOT_PUSHED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    pub fn keyslot_revoked(&self) -> KEYSLOT_REVOKED_R {
        KEYSLOT_REVOKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    pub fn keyslot_error(&self) -> KEYSLOT_ERROR_R {
        KEYSLOT_ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    pub fn keyslot_pushed(&mut self) -> KEYSLOT_PUSHED_W {
        KEYSLOT_PUSHED_W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    pub fn keyslot_revoked(&mut self) -> KEYSLOT_REVOKED_W {
        KEYSLOT_REVOKED_W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    pub fn keyslot_error(&mut self) -> KEYSLOT_ERROR_W {
        KEYSLOT_ERROR_W { w: self }
    }
}

#[doc = "Reader of register PERM"]
pub type R = crate::R<u32, super::PERM>;
#[doc = "Writer for register PERM"]
pub type W = crate::W<u32, super::PERM>;
#[doc = "Register PERM `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PERM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Write permission for key slot\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_A {
    #[doc = "0: Disable write to the key value registers"]
    DISABLED,
    #[doc = "1: Enable write to the key value registers"]
    ENABLED,
}
impl From<WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_A) -> Self {
        match variant {
            WRITE_A::DISABLED => false,
            WRITE_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `WRITE`"]
pub type WRITE_R = crate::R<bool, WRITE_A>;
impl WRITE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_A {
        match self.bits {
            false => WRITE_A::DISABLED,
            true => WRITE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WRITE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WRITE_A::ENABLED
    }
}
#[doc = "Write proxy for field `WRITE`"]
pub struct WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRITE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable write to the key value registers"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WRITE_A::DISABLED)
    }
    #[doc = "Enable write to the key value registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WRITE_A::ENABLED)
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
#[doc = "Read permission for key slot\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_A {
    #[doc = "0: Disable read from key value registers"]
    DISABLED,
    #[doc = "1: Enable read from key value registers"]
    ENABLED,
}
impl From<READ_A> for bool {
    #[inline(always)]
    fn from(variant: READ_A) -> Self {
        match variant {
            READ_A::DISABLED => false,
            READ_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `READ`"]
pub type READ_R = crate::R<bool, READ_A>;
impl READ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_A {
        match self.bits {
            false => READ_A::DISABLED,
            true => READ_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READ_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READ_A::ENABLED
    }
}
#[doc = "Write proxy for field `READ`"]
pub struct READ_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable read from key value registers"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READ_A::DISABLED)
    }
    #[doc = "Enable read from key value registers"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READ_A::ENABLED)
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
#[doc = "Push permission for key slot\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUSH_A {
    #[doc = "0: Disable pushing of key value registers over secure APB, but can be read if field READ is Enabled"]
    DISABLED,
    #[doc = "1: Enable pushing of key value registers over secure APB. Register KEYSLOT.CONFIGn.DEST must contain a valid destination address!"]
    ENABLED,
}
impl From<PUSH_A> for bool {
    #[inline(always)]
    fn from(variant: PUSH_A) -> Self {
        match variant {
            PUSH_A::DISABLED => false,
            PUSH_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `PUSH`"]
pub type PUSH_R = crate::R<bool, PUSH_A>;
impl PUSH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PUSH_A {
        match self.bits {
            false => PUSH_A::DISABLED,
            true => PUSH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PUSH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PUSH_A::ENABLED
    }
}
#[doc = "Write proxy for field `PUSH`"]
pub struct PUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> PUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUSH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable pushing of key value registers over secure APB, but can be read if field READ is Enabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PUSH_A::DISABLED)
    }
    #[doc = "Enable pushing of key value registers over secure APB. Register KEYSLOT.CONFIGn.DEST must contain a valid destination address!"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PUSH_A::ENABLED)
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
#[doc = "Revocation state for the key slot\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: Key value registers can no longer be read or pushed"]
    REVOKED,
    #[doc = "1: Key value registers are readable (if enabled) and can be pushed (if enabled)"]
    ACTIVE,
}
impl From<STATE_A> for bool {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        match variant {
            STATE_A::REVOKED => false,
            STATE_A::ACTIVE => true,
        }
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<bool, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            false => STATE_A::REVOKED,
            true => STATE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `REVOKED`"]
    #[inline(always)]
    pub fn is_revoked(&self) -> bool {
        *self == STATE_A::REVOKED
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == STATE_A::ACTIVE
    }
}
#[doc = "Write proxy for field `STATE`"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Key value registers can no longer be read or pushed"]
    #[inline(always)]
    pub fn revoked(self) -> &'a mut W {
        self.variant(STATE_A::REVOKED)
    }
    #[doc = "Key value registers are readable (if enabled) and can be pushed (if enabled)"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(STATE_A::ACTIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write permission for key slot"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read permission for key slot"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Push permission for key slot"]
    #[inline(always)]
    pub fn push(&self) -> PUSH_R {
        PUSH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Revocation state for the key slot"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write permission for key slot"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W { w: self }
    }
    #[doc = "Bit 1 - Read permission for key slot"]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W {
        READ_W { w: self }
    }
    #[doc = "Bit 2 - Push permission for key slot"]
    #[inline(always)]
    pub fn push(&mut self) -> PUSH_W {
        PUSH_W { w: self }
    }
    #[doc = "Bit 16 - Revocation state for the key slot"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
}

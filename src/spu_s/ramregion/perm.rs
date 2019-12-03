#[doc = "Reader of register PERM"]
pub type R = crate::R<u32, super::PERM>;
#[doc = "Writer for register PERM"]
pub type W = crate::W<u32, super::PERM>;
#[doc = "Register PERM `reset()`'s with value 0x17"]
impl crate::ResetValue for super::PERM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x17
    }
}
#[doc = "Configure instruction fetch permissions from RAM region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXECUTE_A {
    #[doc = "1: Allow instruction fetches from RAM region n"]
    ENABLE,
    #[doc = "0: Block instruction fetches from RAM region n"]
    DISABLE,
}
impl From<EXECUTE_A> for bool {
    #[inline(always)]
    fn from(variant: EXECUTE_A) -> Self {
        match variant {
            EXECUTE_A::ENABLE => true,
            EXECUTE_A::DISABLE => false,
        }
    }
}
#[doc = "Reader of field `EXECUTE`"]
pub type EXECUTE_R = crate::R<bool, EXECUTE_A>;
impl EXECUTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXECUTE_A {
        match self.bits {
            true => EXECUTE_A::ENABLE,
            false => EXECUTE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EXECUTE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EXECUTE_A::DISABLE
    }
}
#[doc = "Write proxy for field `EXECUTE`"]
pub struct EXECUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXECUTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXECUTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow instruction fetches from RAM region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EXECUTE_A::ENABLE)
    }
    #[doc = "Block instruction fetches from RAM region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXECUTE_A::DISABLE)
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
#[doc = "Configure write permission for RAM region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_A {
    #[doc = "1: Allow write operation to RAM region n"]
    ENABLE,
    #[doc = "0: Block write operation to RAM region n"]
    DISABLE,
}
impl From<WRITE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_A) -> Self {
        match variant {
            WRITE_A::ENABLE => true,
            WRITE_A::DISABLE => false,
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
            true => WRITE_A::ENABLE,
            false => WRITE_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WRITE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WRITE_A::DISABLE
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
    #[doc = "Allow write operation to RAM region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WRITE_A::ENABLE)
    }
    #[doc = "Block write operation to RAM region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WRITE_A::DISABLE)
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
#[doc = "Configure read permissions for RAM region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_A {
    #[doc = "1: Allow read operation from RAM region n"]
    ENABLE,
    #[doc = "0: Block read operation from RAM region n"]
    DISABLE,
}
impl From<READ_A> for bool {
    #[inline(always)]
    fn from(variant: READ_A) -> Self {
        match variant {
            READ_A::ENABLE => true,
            READ_A::DISABLE => false,
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
            true => READ_A::ENABLE,
            false => READ_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == READ_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == READ_A::DISABLE
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
    #[doc = "Allow read operation from RAM region n"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(READ_A::ENABLE)
    }
    #[doc = "Block read operation from RAM region n"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(READ_A::DISABLE)
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
#[doc = "Security attribute for RAM region n\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECATTR_A {
    #[doc = "0: RAM region n security attribute is non-secure"]
    NON_SECURE,
    #[doc = "1: RAM region n security attribute is secure"]
    SECURE,
}
impl From<SECATTR_A> for bool {
    #[inline(always)]
    fn from(variant: SECATTR_A) -> Self {
        match variant {
            SECATTR_A::NON_SECURE => false,
            SECATTR_A::SECURE => true,
        }
    }
}
#[doc = "Reader of field `SECATTR`"]
pub type SECATTR_R = crate::R<bool, SECATTR_A>;
impl SECATTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECATTR_A {
        match self.bits {
            false => SECATTR_A::NON_SECURE,
            true => SECATTR_A::SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SECATTR_A::NON_SECURE
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SECATTR_A::SECURE
    }
}
#[doc = "Write proxy for field `SECATTR`"]
pub struct SECATTR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECATTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SECATTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RAM region n security attribute is non-secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(SECATTR_A::NON_SECURE)
    }
    #[doc = "RAM region n security attribute is secure"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(SECATTR_A::SECURE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: This register can be updated"]
    UNLOCKED,
    #[doc = "1: The content of this register can't be changed until the next reset"]
    LOCKED,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        match variant {
            LOCK_A::UNLOCKED => false,
            LOCK_A::LOCKED => true,
        }
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This register can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "The content of this register can't be changed until the next reset"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Configure instruction fetch permissions from RAM region n"]
    #[inline(always)]
    pub fn execute(&self) -> EXECUTE_R {
        EXECUTE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configure write permission for RAM region n"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Configure read permissions for RAM region n"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Security attribute for RAM region n"]
    #[inline(always)]
    pub fn secattr(&self) -> SECATTR_R {
        SECATTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure instruction fetch permissions from RAM region n"]
    #[inline(always)]
    pub fn execute(&mut self) -> EXECUTE_W {
        EXECUTE_W { w: self }
    }
    #[doc = "Bit 1 - Configure write permission for RAM region n"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W { w: self }
    }
    #[doc = "Bit 2 - Configure read permissions for RAM region n"]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W {
        READ_W { w: self }
    }
    #[doc = "Bit 4 - Security attribute for RAM region n"]
    #[inline(always)]
    pub fn secattr(&mut self) -> SECATTR_W {
        SECATTR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}

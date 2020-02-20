#[doc = "Reader of register PERM"]
pub type R = crate::R<u32, super::PERM>;
#[doc = "Writer for register PERM"]
pub type W = crate::W<u32, super::PERM>;
#[doc = "Register PERM `reset()`'s with value 0"]
impl crate::ResetValue for super::PERM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Define configuration capabilities for TrustZone Cortex-M secure attribute\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SECUREMAPPING_A {
    #[doc = "0: The bus access from this external domain always have the non-secure attribute set"]
    NONSECURE = 0,
    #[doc = "1: The bus access from this external domain always have the secure attribute set"]
    SECURE = 1,
    #[doc = "2: Non-secure or secure attribute for bus access from this domain is defined by the EXTDOMAIN\\[n\\].PERM register"]
    USERSELECTABLE = 2,
}
impl From<SECUREMAPPING_A> for u8 {
    #[inline(always)]
    fn from(variant: SECUREMAPPING_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SECUREMAPPING`"]
pub type SECUREMAPPING_R = crate::R<u8, SECUREMAPPING_A>;
impl SECUREMAPPING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SECUREMAPPING_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SECUREMAPPING_A::NONSECURE),
            1 => Val(SECUREMAPPING_A::SECURE),
            2 => Val(SECUREMAPPING_A::USERSELECTABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SECUREMAPPING_A::NONSECURE
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SECUREMAPPING_A::SECURE
    }
    #[doc = "Checks if the value of the field is `USERSELECTABLE`"]
    #[inline(always)]
    pub fn is_user_selectable(&self) -> bool {
        *self == SECUREMAPPING_A::USERSELECTABLE
    }
}
#[doc = "Write proxy for field `SECUREMAPPING`"]
pub struct SECUREMAPPING_W<'a> {
    w: &'a mut W,
}
impl<'a> SECUREMAPPING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SECUREMAPPING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The bus access from this external domain always have the non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(SECUREMAPPING_A::NONSECURE)
    }
    #[doc = "The bus access from this external domain always have the secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(SECUREMAPPING_A::SECURE)
    }
    #[doc = "Non-secure or secure attribute for bus access from this domain is defined by the EXTDOMAIN\\[n\\].PERM register"]
    #[inline(always)]
    pub fn user_selectable(self) -> &'a mut W {
        self.variant(SECUREMAPPING_A::USERSELECTABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Peripheral security mapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECATTR_A {
    #[doc = "0: Bus accesses from this domain have the non-secure attribute set"]
    NONSECURE = 0,
    #[doc = "1: Bus accesses from this domain have secure attribute set"]
    SECURE = 1,
}
impl From<SECATTR_A> for bool {
    #[inline(always)]
    fn from(variant: SECATTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SECATTR`"]
pub type SECATTR_R = crate::R<bool, SECATTR_A>;
impl SECATTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECATTR_A {
        match self.bits {
            false => SECATTR_A::NONSECURE,
            true => SECATTR_A::SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SECATTR_A::NONSECURE
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
    #[doc = "Bus accesses from this domain have the non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(SECATTR_A::NONSECURE)
    }
    #[doc = "Bus accesses from this domain have secure attribute set"]
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
    UNLOCKED = 0,
    #[doc = "1: The content of this register can't be changed until the next reset"]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
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
    #[doc = "Bits 0:1 - Define configuration capabilities for TrustZone Cortex-M secure attribute"]
    #[inline(always)]
    pub fn securemapping(&self) -> SECUREMAPPING_R {
        SECUREMAPPING_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - Peripheral security mapping"]
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
    #[doc = "Bits 0:1 - Define configuration capabilities for TrustZone Cortex-M secure attribute"]
    #[inline(always)]
    pub fn securemapping(&mut self) -> SECUREMAPPING_W {
        SECUREMAPPING_W { w: self }
    }
    #[doc = "Bit 4 - Peripheral security mapping"]
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

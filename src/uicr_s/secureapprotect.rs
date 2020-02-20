#[doc = "Reader of register SECUREAPPROTECT"]
pub type R = crate::R<u32, super::SECUREAPPROTECT>;
#[doc = "Writer for register SECUREAPPROTECT"]
pub type W = crate::W<u32, super::SECUREAPPROTECT>;
#[doc = "Register SECUREAPPROTECT `reset()`'s with value 0"]
impl crate::ResetValue for super::SECUREAPPROTECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PALL_A {
    #[doc = "4294967295: Unprotected"]
    UNPROTECTED = 4294967295,
    #[doc = "0: Protected"]
    PROTECTED = 0,
}
impl From<PALL_A> for u32 {
    #[inline(always)]
    fn from(variant: PALL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PALL`"]
pub type PALL_R = crate::R<u32, PALL_A>;
impl PALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PALL_A> {
        use crate::Variant::*;
        match self.bits {
            4294967295 => Val(PALL_A::UNPROTECTED),
            0 => Val(PALL_A::PROTECTED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNPROTECTED`"]
    #[inline(always)]
    pub fn is_unprotected(&self) -> bool {
        *self == PALL_A::UNPROTECTED
    }
    #[doc = "Checks if the value of the field is `PROTECTED`"]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == PALL_A::PROTECTED
    }
}
#[doc = "Write proxy for field `PALL`"]
pub struct PALL_W<'a> {
    w: &'a mut W,
}
impl<'a> PALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PALL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Unprotected"]
    #[inline(always)]
    pub fn unprotected(self) -> &'a mut W {
        self.variant(PALL_A::UNPROTECTED)
    }
    #[doc = "Protected"]
    #[inline(always)]
    pub fn protected(self) -> &'a mut W {
        self.variant(PALL_A::PROTECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses"]
    #[inline(always)]
    pub fn pall(&self) -> PALL_R {
        PALL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Blocks debugger read/write access to all secure CPU registers and secure memory mapped addresses"]
    #[inline(always)]
    pub fn pall(&mut self) -> PALL_W {
        PALL_W { w: self }
    }
}

#[doc = "Reader of register PERM"]
pub type R = crate::R<u32, super::PERM>;
#[doc = "Writer for register PERM"]
pub type W = crate::W<u32, super::PERM>;
#[doc = "Register PERM `reset()`'s with value 0x12"]
impl crate::ResetValue for super::PERM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x12
    }
}
#[doc = "Define configuration capabilities for TrustZone Cortex-M secure attribute\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECUREMAPPING_A {
    #[doc = "0: This peripheral is always accessible as a non-secure peripheral"]
    NONSECURE,
    #[doc = "1: This peripheral is always accessible as a secure peripheral"]
    SECURE,
    #[doc = "2: Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register"]
    USERSELECTABLE,
    #[doc = "3: This peripheral implements the split security mechanism. Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register."]
    SPLIT,
}
impl From<SECUREMAPPING_A> for u8 {
    #[inline(always)]
    fn from(variant: SECUREMAPPING_A) -> Self {
        match variant {
            SECUREMAPPING_A::NONSECURE => 0,
            SECUREMAPPING_A::SECURE => 1,
            SECUREMAPPING_A::USERSELECTABLE => 2,
            SECUREMAPPING_A::SPLIT => 3,
        }
    }
}
#[doc = "Reader of field `SECUREMAPPING`"]
pub type SECUREMAPPING_R = crate::R<u8, SECUREMAPPING_A>;
impl SECUREMAPPING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECUREMAPPING_A {
        match self.bits {
            0 => SECUREMAPPING_A::NONSECURE,
            1 => SECUREMAPPING_A::SECURE,
            2 => SECUREMAPPING_A::USERSELECTABLE,
            3 => SECUREMAPPING_A::SPLIT,
            _ => unreachable!(),
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
    #[doc = "Checks if the value of the field is `SPLIT`"]
    #[inline(always)]
    pub fn is_split(&self) -> bool {
        *self == SECUREMAPPING_A::SPLIT
    }
}
#[doc = "Indicate if the peripheral has DMA capabilities and if DMA transfer can be assigned to a different security attribute than the peripheral itself\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: Peripheral has no DMA capability"]
    NODMA,
    #[doc = "1: Peripheral has DMA and DMA transfers always have the same security attribute as assigned to the peripheral"]
    NOSEPARATEATTRIBUTE,
    #[doc = "2: Peripheral has DMA and DMA transfers can have a different security attribute than the one assigned to the peripheral"]
    SEPARATEATTRIBUTE,
}
impl From<DMA_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        match variant {
            DMA_A::NODMA => 0,
            DMA_A::NOSEPARATEATTRIBUTE => 1,
            DMA_A::SEPARATEATTRIBUTE => 2,
        }
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<u8, DMA_A>;
impl DMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMA_A::NODMA),
            1 => Val(DMA_A::NOSEPARATEATTRIBUTE),
            2 => Val(DMA_A::SEPARATEATTRIBUTE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODMA`"]
    #[inline(always)]
    pub fn is_no_dma(&self) -> bool {
        *self == DMA_A::NODMA
    }
    #[doc = "Checks if the value of the field is `NOSEPARATEATTRIBUTE`"]
    #[inline(always)]
    pub fn is_no_separate_attribute(&self) -> bool {
        *self == DMA_A::NOSEPARATEATTRIBUTE
    }
    #[doc = "Checks if the value of the field is `SEPARATEATTRIBUTE`"]
    #[inline(always)]
    pub fn is_separate_attribute(&self) -> bool {
        *self == DMA_A::SEPARATEATTRIBUTE
    }
}
#[doc = "Peripheral security mapping\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECATTR_A {
    #[doc = "1: Peripheral is mapped in secure peripheral address space"]
    SECURE,
    #[doc = "0: If SECUREMAPPING == UserSelectable: Peripheral is mapped in non-secure peripheral address space. If SECUREMAPPING == Split: Peripheral is mapped in non-secure and secure peripheral address space."]
    NONSECURE,
}
impl From<SECATTR_A> for bool {
    #[inline(always)]
    fn from(variant: SECATTR_A) -> Self {
        match variant {
            SECATTR_A::SECURE => true,
            SECATTR_A::NONSECURE => false,
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
            true => SECATTR_A::SECURE,
            false => SECATTR_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == SECATTR_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == SECATTR_A::NONSECURE
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
    #[doc = "Peripheral is mapped in secure peripheral address space"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(SECATTR_A::SECURE)
    }
    #[doc = "If SECUREMAPPING == UserSelectable: Peripheral is mapped in non-secure peripheral address space. If SECUREMAPPING == Split: Peripheral is mapped in non-secure and secure peripheral address space."]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(SECATTR_A::NONSECURE)
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
#[doc = "Security attribution for the DMA transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMASEC_A {
    #[doc = "1: DMA transfers initiated by this peripheral have the secure attribute set"]
    SECURE,
    #[doc = "0: DMA transfers initiated by this peripheral have the non-secure attribute set"]
    NONSECURE,
}
impl From<DMASEC_A> for bool {
    #[inline(always)]
    fn from(variant: DMASEC_A) -> Self {
        match variant {
            DMASEC_A::SECURE => true,
            DMASEC_A::NONSECURE => false,
        }
    }
}
#[doc = "Reader of field `DMASEC`"]
pub type DMASEC_R = crate::R<bool, DMASEC_A>;
impl DMASEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMASEC_A {
        match self.bits {
            true => DMASEC_A::SECURE,
            false => DMASEC_A::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == DMASEC_A::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == DMASEC_A::NONSECURE
    }
}
#[doc = "Write proxy for field `DMASEC`"]
pub struct DMASEC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMASEC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA transfers initiated by this peripheral have the secure attribute set"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut W {
        self.variant(DMASEC_A::SECURE)
    }
    #[doc = "DMA transfers initiated by this peripheral have the non-secure attribute set"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(DMASEC_A::NONSECURE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
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
#[doc = "Indicate if a peripheral is present with ID n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESENT_A {
    #[doc = "0: Peripheral is not present"]
    NOTPRESENT,
    #[doc = "1: Peripheral is present"]
    ISPRESENT,
}
impl From<PRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: PRESENT_A) -> Self {
        match variant {
            PRESENT_A::NOTPRESENT => false,
            PRESENT_A::ISPRESENT => true,
        }
    }
}
#[doc = "Reader of field `PRESENT`"]
pub type PRESENT_R = crate::R<bool, PRESENT_A>;
impl PRESENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESENT_A {
        match self.bits {
            false => PRESENT_A::NOTPRESENT,
            true => PRESENT_A::ISPRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == PRESENT_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `ISPRESENT`"]
    #[inline(always)]
    pub fn is_is_present(&self) -> bool {
        *self == PRESENT_A::ISPRESENT
    }
}
impl R {
    #[doc = "Bits 0:1 - Define configuration capabilities for TrustZone Cortex-M secure attribute"]
    #[inline(always)]
    pub fn securemapping(&self) -> SECUREMAPPING_R {
        SECUREMAPPING_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Indicate if the peripheral has DMA capabilities and if DMA transfer can be assigned to a different security attribute than the peripheral itself"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline(always)]
    pub fn secattr(&self) -> SECATTR_R {
        SECATTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Security attribution for the DMA transfer"]
    #[inline(always)]
    pub fn dmasec(&self) -> DMASEC_R {
        DMASEC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Indicate if a peripheral is present with ID n"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline(always)]
    pub fn secattr(&mut self) -> SECATTR_W {
        SECATTR_W { w: self }
    }
    #[doc = "Bit 5 - Security attribution for the DMA transfer"]
    #[inline(always)]
    pub fn dmasec(&mut self) -> DMASEC_W {
        DMASEC_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}

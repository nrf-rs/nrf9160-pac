#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PERM {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SECUREMAPPING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECUREMAPPINGR {
    #[doc = "This peripheral is always accessible as a non-secure peripheral"]
    NONSECURE,
    #[doc = "This peripheral is always accessible as a secure peripheral"]
    SECURE,
    #[doc = "Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register"]
    USERSELECTABLE,
    #[doc = "This peripheral implements the split security mechanism. Non-secure or secure attribute for this peripheral is defined by the PERIPHID\\[n\\].PERM register."]
    SPLIT,
}
impl SECUREMAPPINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SECUREMAPPINGR::NONSECURE => 0,
            SECUREMAPPINGR::SECURE => 1,
            SECUREMAPPINGR::USERSELECTABLE => 2,
            SECUREMAPPINGR::SPLIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SECUREMAPPINGR {
        match value {
            0 => SECUREMAPPINGR::NONSECURE,
            1 => SECUREMAPPINGR::SECURE,
            2 => SECUREMAPPINGR::USERSELECTABLE,
            3 => SECUREMAPPINGR::SPLIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == SECUREMAPPINGR::NONSECURE
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == SECUREMAPPINGR::SECURE
    }
    #[doc = "Checks if the value of the field is `USERSELECTABLE`"]
    #[inline]
    pub fn is_user_selectable(&self) -> bool {
        *self == SECUREMAPPINGR::USERSELECTABLE
    }
    #[doc = "Checks if the value of the field is `SPLIT`"]
    #[inline]
    pub fn is_split(&self) -> bool {
        *self == SECUREMAPPINGR::SPLIT
    }
}
#[doc = "Possible values of the field `DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR {
    #[doc = "Peripheral has no DMA capability"]
    NODMA,
    #[doc = "Peripheral has DMA and DMA transfers always have the same security attribute as assigned to the peripheral"]
    NOSEPARATEATTRIBUTE,
    #[doc = "Peripheral has DMA and DMA transfers can have a different security attribute than the one assigned to the peripheral"]
    SEPARATEATTRIBUTE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DMAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DMAR::NODMA => 0,
            DMAR::NOSEPARATEATTRIBUTE => 1,
            DMAR::SEPARATEATTRIBUTE => 2,
            DMAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DMAR {
        match value {
            0 => DMAR::NODMA,
            1 => DMAR::NOSEPARATEATTRIBUTE,
            2 => DMAR::SEPARATEATTRIBUTE,
            i => DMAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODMA`"]
    #[inline]
    pub fn is_no_dma(&self) -> bool {
        *self == DMAR::NODMA
    }
    #[doc = "Checks if the value of the field is `NOSEPARATEATTRIBUTE`"]
    #[inline]
    pub fn is_no_separate_attribute(&self) -> bool {
        *self == DMAR::NOSEPARATEATTRIBUTE
    }
    #[doc = "Checks if the value of the field is `SEPARATEATTRIBUTE`"]
    #[inline]
    pub fn is_separate_attribute(&self) -> bool {
        *self == DMAR::SEPARATEATTRIBUTE
    }
}
#[doc = "Possible values of the field `SECATTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECATTRR {
    #[doc = "Peripheral is mapped in secure peripheral address space"]
    SECURE,
    #[doc = "If SECUREMAPPING == UserSelectable: Peripheral is mapped in non-secure peripheral address space. If SECUREMAPPING == Split: Peripheral is mapped in non-secure and secure peripheral address space."]
    NONSECURE,
}
impl SECATTRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SECATTRR::SECURE => true,
            SECATTRR::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SECATTRR {
        match value {
            true => SECATTRR::SECURE,
            false => SECATTRR::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == SECATTRR::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == SECATTRR::NONSECURE
    }
}
#[doc = "Possible values of the field `DMASEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMASECR {
    #[doc = "DMA transfers initiated by this peripheral have the secure attribute set"]
    SECURE,
    #[doc = "DMA transfers initiated by this peripheral have the non-secure attribute set"]
    NONSECURE,
}
impl DMASECR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DMASECR::SECURE => true,
            DMASECR::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMASECR {
        match value {
            true => DMASECR::SECURE,
            false => DMASECR::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == DMASECR::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == DMASECR::NONSECURE
    }
}
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKR {
    #[doc = "This register can be updated"]
    UNLOCKED,
    #[doc = "The content of this register can't be changed until the next reset"]
    LOCKED,
}
impl LOCKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LOCKR::UNLOCKED => false,
            LOCKR::LOCKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKR {
        match value {
            false => LOCKR::UNLOCKED,
            true => LOCKR::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKR::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline]
    pub fn is_locked(&self) -> bool {
        *self == LOCKR::LOCKED
    }
}
#[doc = "Possible values of the field `PRESENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESENTR {
    #[doc = "Peripheral is not present"]
    NOTPRESENT,
    #[doc = "Peripheral is present"]
    ISPRESENT,
}
impl PRESENTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PRESENTR::NOTPRESENT => false,
            PRESENTR::ISPRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRESENTR {
        match value {
            false => PRESENTR::NOTPRESENT,
            true => PRESENTR::ISPRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == PRESENTR::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `ISPRESENT`"]
    #[inline]
    pub fn is_is_present(&self) -> bool {
        *self == PRESENTR::ISPRESENT
    }
}
#[doc = "Values that can be written to the field `SECATTR`"]
pub enum SECATTRW {
    #[doc = "Peripheral is mapped in secure peripheral address space"]
    SECURE,
    #[doc = "If SECUREMAPPING == UserSelectable: Peripheral is mapped in non-secure peripheral address space. If SECUREMAPPING == Split: Peripheral is mapped in non-secure and secure peripheral address space."]
    NONSECURE,
}
impl SECATTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SECATTRW::SECURE => true,
            SECATTRW::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SECATTRW<'a> {
    w: &'a mut W,
}
impl<'a> _SECATTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SECATTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripheral is mapped in secure peripheral address space"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(SECATTRW::SECURE)
    }
    #[doc = "If SECUREMAPPING == UserSelectable: Peripheral is mapped in non-secure peripheral address space. If SECUREMAPPING == Split: Peripheral is mapped in non-secure and secure peripheral address space."]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(SECATTRW::NONSECURE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMASEC`"]
pub enum DMASECW {
    #[doc = "DMA transfers initiated by this peripheral have the secure attribute set"]
    SECURE,
    #[doc = "DMA transfers initiated by this peripheral have the non-secure attribute set"]
    NONSECURE,
}
impl DMASECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMASECW::SECURE => true,
            DMASECW::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMASECW<'a> {
    w: &'a mut W,
}
impl<'a> _DMASECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMASECW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA transfers initiated by this peripheral have the secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(DMASECW::SECURE)
    }
    #[doc = "DMA transfers initiated by this peripheral have the non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(DMASECW::NONSECURE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOCK`"]
pub enum LOCKW {
    #[doc = "This register can be updated"]
    UNLOCKED,
    #[doc = "The content of this register can't be changed until the next reset"]
    LOCKED,
}
impl LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKW::UNLOCKED => false,
            LOCKW::LOCKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This register can be updated"]
    #[inline]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKW::UNLOCKED)
    }
    #[doc = "The content of this register can't be changed until the next reset"]
    #[inline]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKW::LOCKED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Define configuration capabilities for TrustZone Cortex-M secure attribute"]
    #[inline]
    pub fn securemapping(&self) -> SECUREMAPPINGR {
        SECUREMAPPINGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Indicate if the peripheral has DMA capabilities and if DMA transfer can be assigned to a different security attribute than the peripheral itself"]
    #[inline]
    pub fn dma(&self) -> DMAR {
        DMAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline]
    pub fn secattr(&self) -> SECATTRR {
        SECATTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Security attribution for the DMA transfer"]
    #[inline]
    pub fn dmasec(&self) -> DMASECR {
        DMASECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Indicate if a peripheral is present with ID n"]
    #[inline]
    pub fn present(&self) -> PRESENTR {
        PRESENTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 18 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline]
    pub fn secattr(&mut self) -> _SECATTRW {
        _SECATTRW { w: self }
    }
    #[doc = "Bit 5 - Security attribution for the DMA transfer"]
    #[inline]
    pub fn dmasec(&mut self) -> _DMASECW {
        _DMASECW { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
}

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
    #[doc = "The bus access from this external domain always have the non-secure attribute set"]
    NONSECURE,
    #[doc = "The bus access from this external domain always have the secure attribute set"]
    SECURE,
    #[doc = "Non-secure or secure attribute for bus access from this domain is defined by the EXTDOMAIN[n].PERM register"]
    USERSELECTABLE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SECUREMAPPINGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SECUREMAPPINGR::NONSECURE => 0,
            SECUREMAPPINGR::SECURE => 1,
            SECUREMAPPINGR::USERSELECTABLE => 2,
            SECUREMAPPINGR::_Reserved(bits) => bits,
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
            i => SECUREMAPPINGR::_Reserved(i),
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
}
#[doc = "Possible values of the field `SECATTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECATTRR {
    #[doc = "Bus accesses from this domain have the non-secure attribute set"]
    NONSECURE,
    #[doc = "Bus accesses from this domain have secure attribute set"]
    SECURE,
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
            SECATTRR::NONSECURE => false,
            SECATTRR::SECURE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SECATTRR {
        match value {
            false => SECATTRR::NONSECURE,
            true => SECATTRR::SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == SECATTRR::NONSECURE
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == SECATTRR::SECURE
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
#[doc = "Values that can be written to the field `SECUREMAPPING`"]
pub enum SECUREMAPPINGW {
    #[doc = "The bus access from this external domain always have the non-secure attribute set"]
    NONSECURE,
    #[doc = "The bus access from this external domain always have the secure attribute set"]
    SECURE,
    #[doc = "Non-secure or secure attribute for bus access from this domain is defined by the EXTDOMAIN[n].PERM register"]
    USERSELECTABLE,
}
impl SECUREMAPPINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SECUREMAPPINGW::NONSECURE => 0,
            SECUREMAPPINGW::SECURE => 1,
            SECUREMAPPINGW::USERSELECTABLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SECUREMAPPINGW<'a> {
    w: &'a mut W,
}
impl<'a> _SECUREMAPPINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SECUREMAPPINGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The bus access from this external domain always have the non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(SECUREMAPPINGW::NONSECURE)
    }
    #[doc = "The bus access from this external domain always have the secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(SECUREMAPPINGW::SECURE)
    }
    #[doc = "Non-secure or secure attribute for bus access from this domain is defined by the EXTDOMAIN[n].PERM register"]
    #[inline]
    pub fn user_selectable(self) -> &'a mut W {
        self.variant(SECUREMAPPINGW::USERSELECTABLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SECATTR`"]
pub enum SECATTRW {
    #[doc = "Bus accesses from this domain have the non-secure attribute set"]
    NONSECURE,
    #[doc = "Bus accesses from this domain have secure attribute set"]
    SECURE,
}
impl SECATTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SECATTRW::NONSECURE => false,
            SECATTRW::SECURE => true,
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
    #[doc = "Bus accesses from this domain have the non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(SECATTRW::NONSECURE)
    }
    #[doc = "Bus accesses from this domain have secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(SECATTRW::SECURE)
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
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline]
    pub fn secattr(&self) -> SECATTRR {
        SECATTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Define configuration capabilities for TrustZone Cortex-M secure attribute"]
    #[inline]
    pub fn securemapping(&mut self) -> _SECUREMAPPINGW {
        _SECUREMAPPINGW { w: self }
    }
    #[doc = "Bit 4 - Peripheral security mapping"]
    #[inline]
    pub fn secattr(&mut self) -> _SECATTRW {
        _SECATTRW { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
}

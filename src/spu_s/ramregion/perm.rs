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
#[doc = "Possible values of the field `EXECUTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXECUTER {
    #[doc = "Allow instruction fetches from RAM region n"]
    ENABLE,
    #[doc = "Block instruction fetches from RAM region n"]
    DISABLE,
}
impl EXECUTER {
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
            EXECUTER::ENABLE => true,
            EXECUTER::DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXECUTER {
        match value {
            true => EXECUTER::ENABLE,
            false => EXECUTER::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == EXECUTER::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == EXECUTER::DISABLE
    }
}
#[doc = "Possible values of the field `WRITE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITER {
    #[doc = "Allow write operation to RAM region n"]
    ENABLE,
    #[doc = "Block write operation to RAM region n"]
    DISABLE,
}
impl WRITER {
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
            WRITER::ENABLE => true,
            WRITER::DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRITER {
        match value {
            true => WRITER::ENABLE,
            false => WRITER::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WRITER::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WRITER::DISABLE
    }
}
#[doc = "Possible values of the field `READ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READR {
    #[doc = "Allow read operation from RAM region n"]
    ENABLE,
    #[doc = "Block read operation from RAM region n"]
    DISABLE,
}
impl READR {
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
            READR::ENABLE => true,
            READR::DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READR {
        match value {
            true => READR::ENABLE,
            false => READR::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == READR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == READR::DISABLE
    }
}
#[doc = "Possible values of the field `SECATTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECATTRR {
    #[doc = "RAM region n security attribute is non-secure"]
    NON_SECURE,
    #[doc = "RAM region n security attribute is secure"]
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
            SECATTRR::NON_SECURE => false,
            SECATTRR::SECURE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SECATTRR {
        match value {
            false => SECATTRR::NON_SECURE,
            true => SECATTRR::SECURE,
        }
    }
    #[doc = "Checks if the value of the field is `NON_SECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == SECATTRR::NON_SECURE
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
#[doc = "Values that can be written to the field `EXECUTE`"]
pub enum EXECUTEW {
    #[doc = "Allow instruction fetches from RAM region n"]
    ENABLE,
    #[doc = "Block instruction fetches from RAM region n"]
    DISABLE,
}
impl EXECUTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXECUTEW::ENABLE => true,
            EXECUTEW::DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXECUTEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXECUTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXECUTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow instruction fetches from RAM region n"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(EXECUTEW::ENABLE)
    }
    #[doc = "Block instruction fetches from RAM region n"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXECUTEW::DISABLE)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WRITE`"]
pub enum WRITEW {
    #[doc = "Allow write operation to RAM region n"]
    ENABLE,
    #[doc = "Block write operation to RAM region n"]
    DISABLE,
}
impl WRITEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WRITEW::ENABLE => true,
            WRITEW::DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRITEW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRITEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow write operation to RAM region n"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WRITEW::ENABLE)
    }
    #[doc = "Block write operation to RAM region n"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WRITEW::DISABLE)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `READ`"]
pub enum READW {
    #[doc = "Allow read operation from RAM region n"]
    ENABLE,
    #[doc = "Block read operation from RAM region n"]
    DISABLE,
}
impl READW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READW::ENABLE => true,
            READW::DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READW<'a> {
    w: &'a mut W,
}
impl<'a> _READW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow read operation from RAM region n"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(READW::ENABLE)
    }
    #[doc = "Block read operation from RAM region n"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(READW::DISABLE)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SECATTR`"]
pub enum SECATTRW {
    #[doc = "RAM region n security attribute is non-secure"]
    NON_SECURE,
    #[doc = "RAM region n security attribute is secure"]
    SECURE,
}
impl SECATTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SECATTRW::NON_SECURE => false,
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
    #[doc = "RAM region n security attribute is non-secure"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(SECATTRW::NON_SECURE)
    }
    #[doc = "RAM region n security attribute is secure"]
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
    #[doc = "Bit 0 - Configure instruction fetch permissions from RAM region n"]
    #[inline]
    pub fn execute(&self) -> EXECUTER {
        EXECUTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Configure write permission for RAM region n"]
    #[inline]
    pub fn write(&self) -> WRITER {
        WRITER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Configure read permissions for RAM region n"]
    #[inline]
    pub fn read(&self) -> READR {
        READR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Security attribute for RAM region n"]
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
        W { bits: 23 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Configure instruction fetch permissions from RAM region n"]
    #[inline]
    pub fn execute(&mut self) -> _EXECUTEW {
        _EXECUTEW { w: self }
    }
    #[doc = "Bit 1 - Configure write permission for RAM region n"]
    #[inline]
    pub fn write(&mut self) -> _WRITEW {
        _WRITEW { w: self }
    }
    #[doc = "Bit 2 - Configure read permissions for RAM region n"]
    #[inline]
    pub fn read(&mut self) -> _READW {
        _READW { w: self }
    }
    #[doc = "Bit 4 - Security attribute for RAM region n"]
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

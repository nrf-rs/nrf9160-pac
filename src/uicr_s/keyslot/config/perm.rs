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
#[doc = "Possible values of the field `WRITE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITER {
    #[doc = "Disable write to the key value registers"]
    DISABLED,
    #[doc = "Enable write to the key value registers"]
    ENABLED,
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
            WRITER::DISABLED => false,
            WRITER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRITER {
        match value {
            false => WRITER::DISABLED,
            true => WRITER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WRITER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WRITER::ENABLED
    }
}
#[doc = "Possible values of the field `READ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READR {
    #[doc = "Disable read from key value registers"]
    DISABLED,
    #[doc = "Enable read from key value registers"]
    ENABLED,
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
            READR::DISABLED => false,
            READR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READR {
        match value {
            false => READR::DISABLED,
            true => READR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == READR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == READR::ENABLED
    }
}
#[doc = "Possible values of the field `PUSH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PUSHR {
    #[doc = "Disable pushing of key value registers over secure APB, but can be read if field READ is Enabled"]
    DISABLED,
    #[doc = "Enable pushing of key value registers over secure APB. Register KEYSLOT.CONFIGn.DEST must contain a valid destination address!"]
    ENABLED,
}
impl PUSHR {
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
            PUSHR::DISABLED => false,
            PUSHR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PUSHR {
        match value {
            false => PUSHR::DISABLED,
            true => PUSHR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PUSHR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PUSHR::ENABLED
    }
}
#[doc = "Possible values of the field `STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATER {
    #[doc = "Key value registers can no longer be read or pushed"]
    REVOKED,
    #[doc = "Key value registers are readable (if enabled) and can be pushed (if enabled)"]
    ACTIVE,
}
impl STATER {
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
            STATER::REVOKED => false,
            STATER::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STATER {
        match value {
            false => STATER::REVOKED,
            true => STATER::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `REVOKED`"]
    #[inline]
    pub fn is_revoked(&self) -> bool {
        *self == STATER::REVOKED
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == STATER::ACTIVE
    }
}
#[doc = "Values that can be written to the field `WRITE`"]
pub enum WRITEW {
    #[doc = "Disable write to the key value registers"]
    DISABLED,
    #[doc = "Enable write to the key value registers"]
    ENABLED,
}
impl WRITEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WRITEW::DISABLED => false,
            WRITEW::ENABLED => true,
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
    #[doc = "Disable write to the key value registers"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WRITEW::DISABLED)
    }
    #[doc = "Enable write to the key value registers"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WRITEW::ENABLED)
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
#[doc = "Values that can be written to the field `READ`"]
pub enum READW {
    #[doc = "Disable read from key value registers"]
    DISABLED,
    #[doc = "Enable read from key value registers"]
    ENABLED,
}
impl READW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READW::DISABLED => false,
            READW::ENABLED => true,
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
    #[doc = "Disable read from key value registers"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READW::DISABLED)
    }
    #[doc = "Enable read from key value registers"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READW::ENABLED)
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
#[doc = "Values that can be written to the field `PUSH`"]
pub enum PUSHW {
    #[doc = "Disable pushing of key value registers over secure APB, but can be read if field READ is Enabled"]
    DISABLED,
    #[doc = "Enable pushing of key value registers over secure APB. Register KEYSLOT.CONFIGn.DEST must contain a valid destination address!"]
    ENABLED,
}
impl PUSHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PUSHW::DISABLED => false,
            PUSHW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PUSHW<'a> {
    w: &'a mut W,
}
impl<'a> _PUSHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PUSHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable pushing of key value registers over secure APB, but can be read if field READ is Enabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PUSHW::DISABLED)
    }
    #[doc = "Enable pushing of key value registers over secure APB. Register KEYSLOT.CONFIGn.DEST must contain a valid destination address!"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PUSHW::ENABLED)
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
#[doc = "Values that can be written to the field `STATE`"]
pub enum STATEW {
    #[doc = "Key value registers can no longer be read or pushed"]
    REVOKED,
    #[doc = "Key value registers are readable (if enabled) and can be pushed (if enabled)"]
    ACTIVE,
}
impl STATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STATEW::REVOKED => false,
            STATEW::ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATEW<'a> {
    w: &'a mut W,
}
impl<'a> _STATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Key value registers can no longer be read or pushed"]
    #[inline]
    pub fn revoked(self) -> &'a mut W {
        self.variant(STATEW::REVOKED)
    }
    #[doc = "Key value registers are readable (if enabled) and can be pushed (if enabled)"]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(STATEW::ACTIVE)
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Write permission for key slot"]
    #[inline]
    pub fn write(&self) -> WRITER {
        WRITER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Read permission for key slot"]
    #[inline]
    pub fn read(&self) -> READR {
        READR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Push permission for key slot"]
    #[inline]
    pub fn push(&self) -> PUSHR {
        PUSHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Revocation state for the key slot"]
    #[inline]
    pub fn state(&self) -> STATER {
        STATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Write permission for key slot"]
    #[inline]
    pub fn write(&mut self) -> _WRITEW {
        _WRITEW { w: self }
    }
    #[doc = "Bit 1 - Read permission for key slot"]
    #[inline]
    pub fn read(&mut self) -> _READW {
        _READW { w: self }
    }
    #[doc = "Bit 2 - Push permission for key slot"]
    #[inline]
    pub fn push(&mut self) -> _PUSHW {
        _PUSHW { w: self }
    }
    #[doc = "Bit 16 - Revocation state for the key slot"]
    #[inline]
    pub fn state(&mut self) -> _STATEW {
        _STATEW { w: self }
    }
}

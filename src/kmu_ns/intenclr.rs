#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENCLR {
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
#[doc = "Possible values of the field `KEYSLOT_PUSHED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_PUSHEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl KEYSLOT_PUSHEDR {
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
            KEYSLOT_PUSHEDR::DISABLED => false,
            KEYSLOT_PUSHEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KEYSLOT_PUSHEDR {
        match value {
            false => KEYSLOT_PUSHEDR::DISABLED,
            true => KEYSLOT_PUSHEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == KEYSLOT_PUSHEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == KEYSLOT_PUSHEDR::ENABLED
    }
}
#[doc = "Possible values of the field `KEYSLOT_REVOKED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_REVOKEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl KEYSLOT_REVOKEDR {
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
            KEYSLOT_REVOKEDR::DISABLED => false,
            KEYSLOT_REVOKEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KEYSLOT_REVOKEDR {
        match value {
            false => KEYSLOT_REVOKEDR::DISABLED,
            true => KEYSLOT_REVOKEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == KEYSLOT_REVOKEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == KEYSLOT_REVOKEDR::ENABLED
    }
}
#[doc = "Possible values of the field `KEYSLOT_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_ERRORR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl KEYSLOT_ERRORR {
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
            KEYSLOT_ERRORR::DISABLED => false,
            KEYSLOT_ERRORR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KEYSLOT_ERRORR {
        match value {
            false => KEYSLOT_ERRORR::DISABLED,
            true => KEYSLOT_ERRORR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == KEYSLOT_ERRORR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == KEYSLOT_ERRORR::ENABLED
    }
}
#[doc = "Values that can be written to the field `KEYSLOT_PUSHED`"]
pub enum KEYSLOT_PUSHEDW {
    #[doc = "Disable"]
    CLEAR,
}
impl KEYSLOT_PUSHEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KEYSLOT_PUSHEDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEYSLOT_PUSHEDW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYSLOT_PUSHEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEYSLOT_PUSHEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(KEYSLOT_PUSHEDW::CLEAR)
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
#[doc = "Values that can be written to the field `KEYSLOT_REVOKED`"]
pub enum KEYSLOT_REVOKEDW {
    #[doc = "Disable"]
    CLEAR,
}
impl KEYSLOT_REVOKEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KEYSLOT_REVOKEDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEYSLOT_REVOKEDW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYSLOT_REVOKEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEYSLOT_REVOKEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(KEYSLOT_REVOKEDW::CLEAR)
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
#[doc = "Values that can be written to the field `KEYSLOT_ERROR`"]
pub enum KEYSLOT_ERRORW {
    #[doc = "Disable"]
    CLEAR,
}
impl KEYSLOT_ERRORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            KEYSLOT_ERRORW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEYSLOT_ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYSLOT_ERRORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEYSLOT_ERRORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(KEYSLOT_ERRORW::CLEAR)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Write '1' to disable interrupt for event KEYSLOT_PUSHED"]
    #[inline]
    pub fn keyslot_pushed(&self) -> KEYSLOT_PUSHEDR {
        KEYSLOT_PUSHEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event KEYSLOT_REVOKED"]
    #[inline]
    pub fn keyslot_revoked(&self) -> KEYSLOT_REVOKEDR {
        KEYSLOT_REVOKEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event KEYSLOT_ERROR"]
    #[inline]
    pub fn keyslot_error(&self) -> KEYSLOT_ERRORR {
        KEYSLOT_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Write '1' to disable interrupt for event KEYSLOT_PUSHED"]
    #[inline]
    pub fn keyslot_pushed(&mut self) -> _KEYSLOT_PUSHEDW {
        _KEYSLOT_PUSHEDW { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event KEYSLOT_REVOKED"]
    #[inline]
    pub fn keyslot_revoked(&mut self) -> _KEYSLOT_REVOKEDW {
        _KEYSLOT_REVOKEDW { w: self }
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event KEYSLOT_ERROR"]
    #[inline]
    pub fn keyslot_error(&mut self) -> _KEYSLOT_ERRORW {
        _KEYSLOT_ERRORW { w: self }
    }
}

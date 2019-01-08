#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN {
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
#[doc = "Possible values of the field `RAMACCERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMACCERRR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl RAMACCERRR {
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
            RAMACCERRR::DISABLED => false,
            RAMACCERRR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAMACCERRR {
        match value {
            false => RAMACCERRR::DISABLED,
            true => RAMACCERRR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RAMACCERRR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RAMACCERRR::ENABLED
    }
}
#[doc = "Possible values of the field `FLASHACCERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHACCERRR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl FLASHACCERRR {
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
            FLASHACCERRR::DISABLED => false,
            FLASHACCERRR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASHACCERRR {
        match value {
            false => FLASHACCERRR::DISABLED,
            true => FLASHACCERRR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHACCERRR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHACCERRR::ENABLED
    }
}
#[doc = "Possible values of the field `PERIPHACCERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIPHACCERRR {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl PERIPHACCERRR {
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
            PERIPHACCERRR::DISABLED => false,
            PERIPHACCERRR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PERIPHACCERRR {
        match value {
            false => PERIPHACCERRR::DISABLED,
            true => PERIPHACCERRR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PERIPHACCERRR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PERIPHACCERRR::ENABLED
    }
}
#[doc = "Values that can be written to the field `RAMACCERR`"]
pub enum RAMACCERRW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl RAMACCERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RAMACCERRW::DISABLED => false,
            RAMACCERRW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAMACCERRW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMACCERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAMACCERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RAMACCERRW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RAMACCERRW::ENABLED)
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
#[doc = "Values that can be written to the field `FLASHACCERR`"]
pub enum FLASHACCERRW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl FLASHACCERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASHACCERRW::DISABLED => false,
            FLASHACCERRW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHACCERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHACCERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHACCERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHACCERRW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHACCERRW::ENABLED)
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
#[doc = "Values that can be written to the field `PERIPHACCERR`"]
pub enum PERIPHACCERRW {
    #[doc = "Disable"]
    DISABLED,
    #[doc = "Enable"]
    ENABLED,
}
impl PERIPHACCERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PERIPHACCERRW::DISABLED => false,
            PERIPHACCERRW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PERIPHACCERRW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIPHACCERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PERIPHACCERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PERIPHACCERRW::DISABLED)
    }
    #[doc = "Enable"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PERIPHACCERRW::ENABLED)
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
    #[doc = "Bit 0 - Enable or disable interrupt for event RAMACCERR"]
    #[inline]
    pub fn ramaccerr(&self) -> RAMACCERRR {
        RAMACCERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event FLASHACCERR"]
    #[inline]
    pub fn flashaccerr(&self) -> FLASHACCERRR {
        FLASHACCERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event PERIPHACCERR"]
    #[inline]
    pub fn periphaccerr(&self) -> PERIPHACCERRR {
        PERIPHACCERRR::_from({
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
    #[doc = "Bit 0 - Enable or disable interrupt for event RAMACCERR"]
    #[inline]
    pub fn ramaccerr(&mut self) -> _RAMACCERRW {
        _RAMACCERRW { w: self }
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event FLASHACCERR"]
    #[inline]
    pub fn flashaccerr(&mut self) -> _FLASHACCERRW {
        _FLASHACCERRW { w: self }
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event PERIPHACCERR"]
    #[inline]
    pub fn periphaccerr(&mut self) -> _PERIPHACCERRW {
        _PERIPHACCERRW { w: self }
    }
}

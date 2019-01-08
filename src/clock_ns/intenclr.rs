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
#[doc = "Possible values of the field `HFCLKSTARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKSTARTEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl HFCLKSTARTEDR {
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
            HFCLKSTARTEDR::DISABLED => false,
            HFCLKSTARTEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HFCLKSTARTEDR {
        match value {
            false => HFCLKSTARTEDR::DISABLED,
            true => HFCLKSTARTEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HFCLKSTARTEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HFCLKSTARTEDR::ENABLED
    }
}
#[doc = "Possible values of the field `LFCLKSTARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFCLKSTARTEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl LFCLKSTARTEDR {
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
            LFCLKSTARTEDR::DISABLED => false,
            LFCLKSTARTEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LFCLKSTARTEDR {
        match value {
            false => LFCLKSTARTEDR::DISABLED,
            true => LFCLKSTARTEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LFCLKSTARTEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LFCLKSTARTEDR::ENABLED
    }
}
#[doc = "Values that can be written to the field `HFCLKSTARTED`"]
pub enum HFCLKSTARTEDW {
    #[doc = "Disable"]
    CLEAR,
}
impl HFCLKSTARTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HFCLKSTARTEDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFCLKSTARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _HFCLKSTARTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFCLKSTARTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(HFCLKSTARTEDW::CLEAR)
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
#[doc = "Values that can be written to the field `LFCLKSTARTED`"]
pub enum LFCLKSTARTEDW {
    #[doc = "Disable"]
    CLEAR,
}
impl LFCLKSTARTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LFCLKSTARTEDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFCLKSTARTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _LFCLKSTARTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFCLKSTARTEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(LFCLKSTARTEDW::CLEAR)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Write '1' to disable interrupt for event HFCLKSTARTED"]
    #[inline]
    pub fn hfclkstarted(&self) -> HFCLKSTARTEDR {
        HFCLKSTARTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event LFCLKSTARTED"]
    #[inline]
    pub fn lfclkstarted(&self) -> LFCLKSTARTEDR {
        LFCLKSTARTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Write '1' to disable interrupt for event HFCLKSTARTED"]
    #[inline]
    pub fn hfclkstarted(&mut self) -> _HFCLKSTARTEDW {
        _HFCLKSTARTEDW { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event LFCLKSTARTED"]
    #[inline]
    pub fn lfclkstarted(&mut self) -> _LFCLKSTARTEDW {
        _LFCLKSTARTEDW { w: self }
    }
}

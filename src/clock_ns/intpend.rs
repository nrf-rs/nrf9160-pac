#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTPEND {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `HFCLKSTARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKSTARTEDR {
    #[doc = "Read: Not pending"]
    NOTPENDING,
    #[doc = "Read: Pending"]
    PENDING,
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
            HFCLKSTARTEDR::NOTPENDING => false,
            HFCLKSTARTEDR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HFCLKSTARTEDR {
        match value {
            false => HFCLKSTARTEDR::NOTPENDING,
            true => HFCLKSTARTEDR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == HFCLKSTARTEDR::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == HFCLKSTARTEDR::PENDING
    }
}
#[doc = "Possible values of the field `LFCLKSTARTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFCLKSTARTEDR {
    #[doc = "Read: Not pending"]
    NOTPENDING,
    #[doc = "Read: Pending"]
    PENDING,
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
            LFCLKSTARTEDR::NOTPENDING => false,
            LFCLKSTARTEDR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LFCLKSTARTEDR {
        match value {
            false => LFCLKSTARTEDR::NOTPENDING,
            true => LFCLKSTARTEDR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == LFCLKSTARTEDR::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == LFCLKSTARTEDR::PENDING
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Read pending status of interrupt for event HFCLKSTARTED"]
    #[inline]
    pub fn hfclkstarted(&self) -> HFCLKSTARTEDR {
        HFCLKSTARTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event LFCLKSTARTED"]
    #[inline]
    pub fn lfclkstarted(&self) -> LFCLKSTARTEDR {
        LFCLKSTARTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}

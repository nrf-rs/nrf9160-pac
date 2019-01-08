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
#[doc = "Possible values of the field `KEYSLOT_PUSHED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_PUSHEDR {
    #[doc = "Read: Not pending"]
    NOTPENDING,
    #[doc = "Read: Pending"]
    PENDING,
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
            KEYSLOT_PUSHEDR::NOTPENDING => false,
            KEYSLOT_PUSHEDR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KEYSLOT_PUSHEDR {
        match value {
            false => KEYSLOT_PUSHEDR::NOTPENDING,
            true => KEYSLOT_PUSHEDR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == KEYSLOT_PUSHEDR::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == KEYSLOT_PUSHEDR::PENDING
    }
}
#[doc = "Possible values of the field `KEYSLOT_REVOKED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_REVOKEDR {
    #[doc = "Read: Not pending"]
    NOTPENDING,
    #[doc = "Read: Pending"]
    PENDING,
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
            KEYSLOT_REVOKEDR::NOTPENDING => false,
            KEYSLOT_REVOKEDR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KEYSLOT_REVOKEDR {
        match value {
            false => KEYSLOT_REVOKEDR::NOTPENDING,
            true => KEYSLOT_REVOKEDR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == KEYSLOT_REVOKEDR::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == KEYSLOT_REVOKEDR::PENDING
    }
}
#[doc = "Possible values of the field `KEYSLOT_ERROR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_ERRORR {
    #[doc = "Read: Not pending"]
    NOTPENDING,
    #[doc = "Read: Pending"]
    PENDING,
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
            KEYSLOT_ERRORR::NOTPENDING => false,
            KEYSLOT_ERRORR::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> KEYSLOT_ERRORR {
        match value {
            false => KEYSLOT_ERRORR::NOTPENDING,
            true => KEYSLOT_ERRORR::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == KEYSLOT_ERRORR::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == KEYSLOT_ERRORR::PENDING
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Read pending status of interrupt for event KEYSLOT_PUSHED"]
    #[inline]
    pub fn keyslot_pushed(&self) -> KEYSLOT_PUSHEDR {
        KEYSLOT_PUSHEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event KEYSLOT_REVOKED"]
    #[inline]
    pub fn keyslot_revoked(&self) -> KEYSLOT_REVOKEDR {
        KEYSLOT_REVOKEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event KEYSLOT_ERROR"]
    #[inline]
    pub fn keyslot_error(&self) -> KEYSLOT_ERRORR {
        KEYSLOT_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}

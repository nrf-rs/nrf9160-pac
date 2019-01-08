#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CAP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TZM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZMR {
    #[doc = "ARM TrustZone support not available"]
    NOTAVAILABLE,
    #[doc = "ARM TrustZone support is available"]
    ENABLED,
}
impl TZMR {
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
            TZMR::NOTAVAILABLE => false,
            TZMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TZMR {
        match value {
            false => TZMR::NOTAVAILABLE,
            true => TZMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTAVAILABLE`"]
    #[inline]
    pub fn is_not_available(&self) -> bool {
        *self == TZMR::NOTAVAILABLE
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TZMR::ENABLED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Show ARM TrustZone status"]
    #[inline]
    pub fn tzm(&self) -> TZMR {
        TZMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}

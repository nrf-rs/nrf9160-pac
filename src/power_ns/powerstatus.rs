#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::POWERSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `LTEMODEM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTEMODEMR {
    #[doc = "LTE modem domain is powered off"]
    OFF,
    #[doc = "LTE modem domain is powered on"]
    ON,
}
impl LTEMODEMR {
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
            LTEMODEMR::OFF => false,
            LTEMODEMR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LTEMODEMR {
        match value {
            false => LTEMODEMR::OFF,
            true => LTEMODEMR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == LTEMODEMR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == LTEMODEMR::ON
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - LTE modem domain status"]
    #[inline]
    pub fn ltemodem(&self) -> LTEMODEMR {
        LTEMODEMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}

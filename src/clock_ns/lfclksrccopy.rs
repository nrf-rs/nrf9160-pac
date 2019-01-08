#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::LFCLKSRCCOPY {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCR {
    #[doc = "Reserved for future use"]
    RFU,
    #[doc = "32.768 kHz RC oscillator"]
    LFRC,
    #[doc = "32.768 kHz crystal oscillator"]
    LFXO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCR::RFU => 0,
            SRCR::LFRC => 1,
            SRCR::LFXO => 2,
            SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCR {
        match value {
            0 => SRCR::RFU,
            1 => SRCR::LFRC,
            2 => SRCR::LFXO,
            i => SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RFU`"]
    #[inline]
    pub fn is_rfu(&self) -> bool {
        *self == SRCR::RFU
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == SRCR::LFRC
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == SRCR::LFXO
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Clock source"]
    #[inline]
    pub fn src(&self) -> SRCR {
        SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}

#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PART {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARTR {
    #[doc = "nRF9160"]
    N9160,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl PARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            PARTR::N9160 => 37216,
            PARTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> PARTR {
        match value {
            37216 => PARTR::N9160,
            i => PARTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `N9160`"]
    #[inline]
    pub fn is_n9160(&self) -> bool {
        *self == PARTR::N9160
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Part code"]
    #[inline]
    pub fn part(&self) -> PARTR {
        PARTR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}

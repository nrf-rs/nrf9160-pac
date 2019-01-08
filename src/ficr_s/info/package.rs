#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PACKAGE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PACKAGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKAGER {
    #[doc = "CCxx - 236 ball wlCSP"]
    CC,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl PACKAGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            PACKAGER::CC => 8192,
            PACKAGER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> PACKAGER {
        match value {
            8192 => PACKAGER::CC,
            i => PACKAGER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CC`"]
    #[inline]
    pub fn is_cc(&self) -> bool {
        *self == PACKAGER::CC
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Package option"]
    #[inline]
    pub fn package(&self) -> PACKAGER {
        PACKAGER::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}

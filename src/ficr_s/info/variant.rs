#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::VARIANT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `VARIANT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VARIANTR {
    #[doc = "AAAA"]
    AAAA,
    #[doc = "AAA0"]
    AAA0,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl VARIANTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            VARIANTR::AAAA => 1094795585,
            VARIANTR::AAA0 => 1094795568,
            VARIANTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> VARIANTR {
        match value {
            1094795585 => VARIANTR::AAAA,
            1094795568 => VARIANTR::AAA0,
            i => VARIANTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AAAA`"]
    #[inline]
    pub fn is_aaaa(&self) -> bool {
        *self == VARIANTR::AAAA
    }
    #[doc = "Checks if the value of the field is `AAA0`"]
    #[inline]
    pub fn is_aaa0(&self) -> bool {
        *self == VARIANTR::AAA0
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Part Variant, Hardware version and Production configuration, encoded as ASCII"]
    #[inline]
    pub fn variant(&self) -> VARIANTR {
        VARIANTR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}

#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DEVICETYPE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DEVICETYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVICETYPER {
    #[doc = "Device is an physical DIE"]
    DIE,
    #[doc = "Device is an FPGA"]
    FPGA,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl DEVICETYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            DEVICETYPER::DIE => 0,
            DEVICETYPER::FPGA => 4294967295,
            DEVICETYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> DEVICETYPER {
        match value {
            0 => DEVICETYPER::DIE,
            4294967295 => DEVICETYPER::FPGA,
            i => DEVICETYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIE`"]
    #[inline]
    pub fn is_die(&self) -> bool {
        *self == DEVICETYPER::DIE
    }
    #[doc = "Checks if the value of the field is `FPGA`"]
    #[inline]
    pub fn is_fpga(&self) -> bool {
        *self == DEVICETYPER::FPGA
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:31 - Device type"]
    #[inline]
    pub fn devicetype(&self) -> DEVICETYPER {
        DEVICETYPER::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}

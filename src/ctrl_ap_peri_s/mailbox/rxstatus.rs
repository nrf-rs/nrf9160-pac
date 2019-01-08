#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RXSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RXSTATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTATUSR {
    #[doc = "No data pending in register RXDATA"]
    NODATAPENDING,
    #[doc = "Data pending in register RXDATA"]
    DATAPENDING,
}
impl RXSTATUSR {
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
            RXSTATUSR::NODATAPENDING => false,
            RXSTATUSR::DATAPENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXSTATUSR {
        match value {
            false => RXSTATUSR::NODATAPENDING,
            true => RXSTATUSR::DATAPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NODATAPENDING`"]
    #[inline]
    pub fn is_no_data_pending(&self) -> bool {
        *self == RXSTATUSR::NODATAPENDING
    }
    #[doc = "Checks if the value of the field is `DATAPENDING`"]
    #[inline]
    pub fn is_data_pending(&self) -> bool {
        *self == RXSTATUSR::DATAPENDING
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Status of data in register RXDATA"]
    #[inline]
    pub fn rxstatus(&self) -> RXSTATUSR {
        RXSTATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}

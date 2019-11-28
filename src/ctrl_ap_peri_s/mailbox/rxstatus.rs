#[doc = "Reader of register RXSTATUS"]
pub type R = crate::R<u32, super::RXSTATUS>;
#[doc = "Status of data in register RXDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTATUS_A {
    #[doc = "0: No data pending in register RXDATA"]
    NODATAPENDING,
    #[doc = "1: Data pending in register RXDATA"]
    DATAPENDING,
}
impl From<RXSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: RXSTATUS_A) -> Self {
        match variant {
            RXSTATUS_A::NODATAPENDING => false,
            RXSTATUS_A::DATAPENDING => true,
        }
    }
}
#[doc = "Reader of field `RXSTATUS`"]
pub type RXSTATUS_R = crate::R<bool, RXSTATUS_A>;
impl RXSTATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSTATUS_A {
        match self.bits {
            false => RXSTATUS_A::NODATAPENDING,
            true => RXSTATUS_A::DATAPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NODATAPENDING`"]
    #[inline(always)]
    pub fn is_no_data_pending(&self) -> bool {
        *self == RXSTATUS_A::NODATAPENDING
    }
    #[doc = "Checks if the value of the field is `DATAPENDING`"]
    #[inline(always)]
    pub fn is_data_pending(&self) -> bool {
        *self == RXSTATUS_A::DATAPENDING
    }
}
impl R {
    #[doc = "Bit 0 - Status of data in register RXDATA"]
    #[inline(always)]
    pub fn rxstatus(&self) -> RXSTATUS_R {
        RXSTATUS_R::new((self.bits & 0x01) != 0)
    }
}

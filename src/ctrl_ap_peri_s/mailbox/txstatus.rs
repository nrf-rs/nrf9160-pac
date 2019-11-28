#[doc = "Reader of register TXSTATUS"]
pub type R = crate::R<u32, super::TXSTATUS>;
#[doc = "Status of data in register TXDATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTATUS_A {
    #[doc = "0: No data pending in register TXDATA"]
    NODATAPENDING,
    #[doc = "1: Data pending in register TXDATA"]
    DATAPENDING,
}
impl From<TXSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: TXSTATUS_A) -> Self {
        match variant {
            TXSTATUS_A::NODATAPENDING => false,
            TXSTATUS_A::DATAPENDING => true,
        }
    }
}
#[doc = "Reader of field `TXSTATUS`"]
pub type TXSTATUS_R = crate::R<bool, TXSTATUS_A>;
impl TXSTATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSTATUS_A {
        match self.bits {
            false => TXSTATUS_A::NODATAPENDING,
            true => TXSTATUS_A::DATAPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NODATAPENDING`"]
    #[inline(always)]
    pub fn is_no_data_pending(&self) -> bool {
        *self == TXSTATUS_A::NODATAPENDING
    }
    #[doc = "Checks if the value of the field is `DATAPENDING`"]
    #[inline(always)]
    pub fn is_data_pending(&self) -> bool {
        *self == TXSTATUS_A::DATAPENDING
    }
}
impl R {
    #[doc = "Bit 0 - Status of data in register TXDATA"]
    #[inline(always)]
    pub fn txstatus(&self) -> TXSTATUS_R {
        TXSTATUS_R::new((self.bits & 0x01) != 0)
    }
}

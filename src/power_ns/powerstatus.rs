#[doc = "Reader of register POWERSTATUS"]
pub type R = crate::R<u32, super::POWERSTATUS>;
#[doc = "LTE modem domain status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTEMODEM_A {
    #[doc = "0: LTE modem domain is powered off"]
    OFF,
    #[doc = "1: LTE modem domain is powered on"]
    ON,
}
impl From<LTEMODEM_A> for bool {
    #[inline(always)]
    fn from(variant: LTEMODEM_A) -> Self {
        match variant {
            LTEMODEM_A::OFF => false,
            LTEMODEM_A::ON => true,
        }
    }
}
#[doc = "Reader of field `LTEMODEM`"]
pub type LTEMODEM_R = crate::R<bool, LTEMODEM_A>;
impl LTEMODEM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTEMODEM_A {
        match self.bits {
            false => LTEMODEM_A::OFF,
            true => LTEMODEM_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LTEMODEM_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LTEMODEM_A::ON
    }
}
impl R {
    #[doc = "Bit 0 - LTE modem domain status"]
    #[inline(always)]
    pub fn ltemodem(&self) -> LTEMODEM_R {
        LTEMODEM_R::new((self.bits & 0x01) != 0)
    }
}

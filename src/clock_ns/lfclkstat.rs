#[doc = "Reader of register LFCLKSTAT"]
pub type R = crate::R<u32, super::LFCLKSTAT>;
#[doc = "Active clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: Reserved for future use"]
    RFU = 0,
    #[doc = "1: 32.768 kHz RC oscillator"]
    LFRC = 1,
    #[doc = "2: 32.768 kHz crystal oscillator"]
    LFXO = 2,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_A::RFU),
            1 => Val(SRC_A::LFRC),
            2 => Val(SRC_A::LFXO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RFU`"]
    #[inline(always)]
    pub fn is_rfu(&self) -> bool {
        *self == SRC_A::RFU
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == SRC_A::LFRC
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == SRC_A::LFXO
    }
}
#[doc = "LFCLK state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: Requested LFCLK source has not been started or LFCLKSTOP task has been triggered"]
    NOTRUNNING = 0,
    #[doc = "1: Requested LFCLK source has been started (LFCLKSTARTED event has been generated)"]
    RUNNING = 1,
}
impl From<STATE_A> for bool {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<bool, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            false => STATE_A::NOTRUNNING,
            true => STATE_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == STATE_A::NOTRUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == STATE_A::RUNNING
    }
}
impl R {
    #[doc = "Bits 0:1 - Active clock source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 16 - LFCLK state"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}

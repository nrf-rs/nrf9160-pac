#[doc = "Reader of register RUNSTATUS"]
pub type R = crate::R<u32, super::RUNSTATUS>;
#[doc = "Indicates whether or not the watchdog is running\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNSTATUSWDT_A {
    #[doc = "0: Watchdog not running"]
    NOTRUNNING,
    #[doc = "1: Watchdog is running"]
    RUNNING,
}
impl From<RUNSTATUSWDT_A> for bool {
    #[inline(always)]
    fn from(variant: RUNSTATUSWDT_A) -> Self {
        match variant {
            RUNSTATUSWDT_A::NOTRUNNING => false,
            RUNSTATUSWDT_A::RUNNING => true,
        }
    }
}
#[doc = "Reader of field `RUNSTATUSWDT`"]
pub type RUNSTATUSWDT_R = crate::R<bool, RUNSTATUSWDT_A>;
impl RUNSTATUSWDT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUNSTATUSWDT_A {
        match self.bits {
            false => RUNSTATUSWDT_A::NOTRUNNING,
            true => RUNSTATUSWDT_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == RUNSTATUSWDT_A::NOTRUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == RUNSTATUSWDT_A::RUNNING
    }
}
impl R {
    #[doc = "Bit 0 - Indicates whether or not the watchdog is running"]
    #[inline(always)]
    pub fn runstatuswdt(&self) -> RUNSTATUSWDT_R {
        RUNSTATUSWDT_R::new((self.bits & 0x01) != 0)
    }
}

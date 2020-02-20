#[doc = "Reader of register CAP"]
pub type R = crate::R<u32, super::CAP>;
#[doc = "Show ARM TrustZone status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZM_A {
    #[doc = "0: ARM TrustZone support not available"]
    NOTAVAILABLE = 0,
    #[doc = "1: ARM TrustZone support is available"]
    ENABLED = 1,
}
impl From<TZM_A> for bool {
    #[inline(always)]
    fn from(variant: TZM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TZM`"]
pub type TZM_R = crate::R<bool, TZM_A>;
impl TZM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZM_A {
        match self.bits {
            false => TZM_A::NOTAVAILABLE,
            true => TZM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTAVAILABLE`"]
    #[inline(always)]
    pub fn is_not_available(&self) -> bool {
        *self == TZM_A::NOTAVAILABLE
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TZM_A::ENABLED
    }
}
impl R {
    #[doc = "Bit 0 - Show ARM TrustZone status"]
    #[inline(always)]
    pub fn tzm(&self) -> TZM_R {
        TZM_R::new((self.bits & 0x01) != 0)
    }
}

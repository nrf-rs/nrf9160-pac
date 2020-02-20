#[doc = "Reader of register INTPEND"]
pub type R = crate::R<u32, super::INTPEND>;
#[doc = "Read pending status of interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFCLKSTARTED_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<HFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: HFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFCLKSTARTED`"]
pub type HFCLKSTARTED_R = crate::R<bool, HFCLKSTARTED_A>;
impl HFCLKSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLKSTARTED_A {
        match self.bits {
            false => HFCLKSTARTED_A::NOTPENDING,
            true => HFCLKSTARTED_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == HFCLKSTARTED_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == HFCLKSTARTED_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFCLKSTARTED_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING = 0,
    #[doc = "1: Read: Pending"]
    PENDING = 1,
}
impl From<LFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: LFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LFCLKSTARTED`"]
pub type LFCLKSTARTED_R = crate::R<bool, LFCLKSTARTED_A>;
impl LFCLKSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFCLKSTARTED_A {
        match self.bits {
            false => LFCLKSTARTED_A::NOTPENDING,
            true => LFCLKSTARTED_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == LFCLKSTARTED_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == LFCLKSTARTED_A::PENDING
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&self) -> HFCLKSTARTED_R {
        HFCLKSTARTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&self) -> LFCLKSTARTED_R {
        LFCLKSTARTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}

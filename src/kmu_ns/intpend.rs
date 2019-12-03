#[doc = "Reader of register INTPEND"]
pub type R = crate::R<u32, super::INTPEND>;
#[doc = "Read pending status of interrupt for event KEYSLOT_PUSHED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_PUSHED_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<KEYSLOT_PUSHED_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_PUSHED_A) -> Self {
        match variant {
            KEYSLOT_PUSHED_A::NOTPENDING => false,
            KEYSLOT_PUSHED_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `KEYSLOT_PUSHED`"]
pub type KEYSLOT_PUSHED_R = crate::R<bool, KEYSLOT_PUSHED_A>;
impl KEYSLOT_PUSHED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYSLOT_PUSHED_A {
        match self.bits {
            false => KEYSLOT_PUSHED_A::NOTPENDING,
            true => KEYSLOT_PUSHED_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == KEYSLOT_PUSHED_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == KEYSLOT_PUSHED_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event KEYSLOT_REVOKED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_REVOKED_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<KEYSLOT_REVOKED_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_REVOKED_A) -> Self {
        match variant {
            KEYSLOT_REVOKED_A::NOTPENDING => false,
            KEYSLOT_REVOKED_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `KEYSLOT_REVOKED`"]
pub type KEYSLOT_REVOKED_R = crate::R<bool, KEYSLOT_REVOKED_A>;
impl KEYSLOT_REVOKED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYSLOT_REVOKED_A {
        match self.bits {
            false => KEYSLOT_REVOKED_A::NOTPENDING,
            true => KEYSLOT_REVOKED_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == KEYSLOT_REVOKED_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == KEYSLOT_REVOKED_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event KEYSLOT_ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSLOT_ERROR_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<KEYSLOT_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: KEYSLOT_ERROR_A) -> Self {
        match variant {
            KEYSLOT_ERROR_A::NOTPENDING => false,
            KEYSLOT_ERROR_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `KEYSLOT_ERROR`"]
pub type KEYSLOT_ERROR_R = crate::R<bool, KEYSLOT_ERROR_A>;
impl KEYSLOT_ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYSLOT_ERROR_A {
        match self.bits {
            false => KEYSLOT_ERROR_A::NOTPENDING,
            true => KEYSLOT_ERROR_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == KEYSLOT_ERROR_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == KEYSLOT_ERROR_A::PENDING
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event KEYSLOT_PUSHED"]
    #[inline(always)]
    pub fn keyslot_pushed(&self) -> KEYSLOT_PUSHED_R {
        KEYSLOT_PUSHED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event KEYSLOT_REVOKED"]
    #[inline(always)]
    pub fn keyslot_revoked(&self) -> KEYSLOT_REVOKED_R {
        KEYSLOT_REVOKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event KEYSLOT_ERROR"]
    #[inline(always)]
    pub fn keyslot_error(&self) -> KEYSLOT_ERROR_R {
        KEYSLOT_ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}

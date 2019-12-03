#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Key slot ID successfully selected by the KMU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECTED_A {
    #[doc = "0: No key slot ID selected by KMU"]
    DISABLED,
    #[doc = "1: Key slot ID successfully selected by KMU"]
    ENABLED,
}
impl From<SELECTED_A> for bool {
    #[inline(always)]
    fn from(variant: SELECTED_A) -> Self {
        match variant {
            SELECTED_A::DISABLED => false,
            SELECTED_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `SELECTED`"]
pub type SELECTED_R = crate::R<bool, SELECTED_A>;
impl SELECTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELECTED_A {
        match self.bits {
            false => SELECTED_A::DISABLED,
            true => SELECTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SELECTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SELECTED_A::ENABLED
    }
}
#[doc = "Violation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCKED_A {
    #[doc = "0: No access violation detected"]
    DISABLED,
    #[doc = "1: Access violation detected and blocked"]
    ENABLED,
}
impl From<BLOCKED_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCKED_A) -> Self {
        match variant {
            BLOCKED_A::DISABLED => false,
            BLOCKED_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `BLOCKED`"]
pub type BLOCKED_R = crate::R<bool, BLOCKED_A>;
impl BLOCKED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLOCKED_A {
        match self.bits {
            false => BLOCKED_A::DISABLED,
            true => BLOCKED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BLOCKED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BLOCKED_A::ENABLED
    }
}
impl R {
    #[doc = "Bit 0 - Key slot ID successfully selected by the KMU"]
    #[inline(always)]
    pub fn selected(&self) -> SELECTED_R {
        SELECTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Violation status"]
    #[inline(always)]
    pub fn blocked(&self) -> BLOCKED_R {
        BLOCKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}

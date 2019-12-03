#[doc = "Reader of register FORCEOFFNVM"]
pub type R = crate::R<u32, super::FORCEOFFNVM>;
#[doc = "Writer for register FORCEOFFNVM"]
pub type W = crate::W<u32, super::FORCEOFFNVM>;
#[doc = "Register FORCEOFFNVM `reset()`'s with value 0"]
impl crate::ResetValue for super::FORCEOFFNVM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Force off NVM supply 0. Also see the internal section in the NVMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEOFFNVM0_A {
    #[doc = "0: Do not force off supply"]
    DONOTFORCEOFF,
    #[doc = "1: Force off supply"]
    FORCEOFF,
}
impl From<FORCEOFFNVM0_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEOFFNVM0_A) -> Self {
        match variant {
            FORCEOFFNVM0_A::DONOTFORCEOFF => false,
            FORCEOFFNVM0_A::FORCEOFF => true,
        }
    }
}
#[doc = "Reader of field `FORCEOFFNVM0`"]
pub type FORCEOFFNVM0_R = crate::R<bool, FORCEOFFNVM0_A>;
impl FORCEOFFNVM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEOFFNVM0_A {
        match self.bits {
            false => FORCEOFFNVM0_A::DONOTFORCEOFF,
            true => FORCEOFFNVM0_A::FORCEOFF,
        }
    }
    #[doc = "Checks if the value of the field is `DONOTFORCEOFF`"]
    #[inline(always)]
    pub fn is_do_not_force_off(&self) -> bool {
        *self == FORCEOFFNVM0_A::DONOTFORCEOFF
    }
    #[doc = "Checks if the value of the field is `FORCEOFF`"]
    #[inline(always)]
    pub fn is_force_off(&self) -> bool {
        *self == FORCEOFFNVM0_A::FORCEOFF
    }
}
#[doc = "Write proxy for field `FORCEOFFNVM0`"]
pub struct FORCEOFFNVM0_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEOFFNVM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEOFFNVM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not force off supply"]
    #[inline(always)]
    pub fn do_not_force_off(self) -> &'a mut W {
        self.variant(FORCEOFFNVM0_A::DONOTFORCEOFF)
    }
    #[doc = "Force off supply"]
    #[inline(always)]
    pub fn force_off(self) -> &'a mut W {
        self.variant(FORCEOFFNVM0_A::FORCEOFF)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Force off NVM supply 1. Also see the internal section in the NVMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEOFFNVM1_A {
    #[doc = "0: Do not force off supply"]
    DONOTFORCEOFF,
    #[doc = "1: Force off supply"]
    FORCEOFF,
}
impl From<FORCEOFFNVM1_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEOFFNVM1_A) -> Self {
        match variant {
            FORCEOFFNVM1_A::DONOTFORCEOFF => false,
            FORCEOFFNVM1_A::FORCEOFF => true,
        }
    }
}
#[doc = "Reader of field `FORCEOFFNVM1`"]
pub type FORCEOFFNVM1_R = crate::R<bool, FORCEOFFNVM1_A>;
impl FORCEOFFNVM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEOFFNVM1_A {
        match self.bits {
            false => FORCEOFFNVM1_A::DONOTFORCEOFF,
            true => FORCEOFFNVM1_A::FORCEOFF,
        }
    }
    #[doc = "Checks if the value of the field is `DONOTFORCEOFF`"]
    #[inline(always)]
    pub fn is_do_not_force_off(&self) -> bool {
        *self == FORCEOFFNVM1_A::DONOTFORCEOFF
    }
    #[doc = "Checks if the value of the field is `FORCEOFF`"]
    #[inline(always)]
    pub fn is_force_off(&self) -> bool {
        *self == FORCEOFFNVM1_A::FORCEOFF
    }
}
#[doc = "Write proxy for field `FORCEOFFNVM1`"]
pub struct FORCEOFFNVM1_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEOFFNVM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEOFFNVM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not force off supply"]
    #[inline(always)]
    pub fn do_not_force_off(self) -> &'a mut W {
        self.variant(FORCEOFFNVM1_A::DONOTFORCEOFF)
    }
    #[doc = "Force off supply"]
    #[inline(always)]
    pub fn force_off(self) -> &'a mut W {
        self.variant(FORCEOFFNVM1_A::FORCEOFF)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "KEY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_A {
    #[doc = "11325013: Must be written in order to write to bits 0-7. Any other value will ignore writes to this register. Read as zero."]
    ENABLEWRITE,
}
impl From<KEY_A> for u32 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        match variant {
            KEY_A::ENABLEWRITE => 11325013,
        }
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u32, KEY_A>;
impl KEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, KEY_A> {
        use crate::Variant::*;
        match self.bits {
            11325013 => Val(KEY_A::ENABLEWRITE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLEWRITE`"]
    #[inline(always)]
    pub fn is_enable_write(&self) -> bool {
        *self == KEY_A::ENABLEWRITE
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Must be written in order to write to bits 0-7. Any other value will ignore writes to this register. Read as zero."]
    #[inline(always)]
    pub fn enable_write(self) -> &'a mut W {
        self.variant(KEY_A::ENABLEWRITE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Force off NVM supply 0. Also see the internal section in the NVMC chapter."]
    #[inline(always)]
    pub fn forceoffnvm0(&self) -> FORCEOFFNVM0_R {
        FORCEOFFNVM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force off NVM supply 1. Also see the internal section in the NVMC chapter."]
    #[inline(always)]
    pub fn forceoffnvm1(&self) -> FORCEOFFNVM1_R {
        FORCEOFFNVM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - KEY"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Force off NVM supply 0. Also see the internal section in the NVMC chapter."]
    #[inline(always)]
    pub fn forceoffnvm0(&mut self) -> FORCEOFFNVM0_W {
        FORCEOFFNVM0_W { w: self }
    }
    #[doc = "Bit 1 - Force off NVM supply 1. Also see the internal section in the NVMC chapter."]
    #[inline(always)]
    pub fn forceoffnvm1(&mut self) -> FORCEOFFNVM1_W {
        FORCEOFFNVM1_W { w: self }
    }
    #[doc = "Bits 8:31 - KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}

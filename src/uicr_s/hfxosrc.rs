#[doc = "Reader of register HFXOSRC"]
pub type R = crate::R<u32, super::HFXOSRC>;
#[doc = "Writer for register HFXOSRC"]
pub type W = crate::W<u32, super::HFXOSRC>;
#[doc = "Register HFXOSRC `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::HFXOSRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "HFXO clock source selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXOSRC_A {
    #[doc = "1: 32 MHz crystal oscillator"]
    XTAL = 1,
    #[doc = "0: 32 MHz temperature compensated crystal oscillator (TCXO)"]
    TCXO = 0,
}
impl From<HFXOSRC_A> for bool {
    #[inline(always)]
    fn from(variant: HFXOSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFXOSRC`"]
pub type HFXOSRC_R = crate::R<bool, HFXOSRC_A>;
impl HFXOSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXOSRC_A {
        match self.bits {
            true => HFXOSRC_A::XTAL,
            false => HFXOSRC_A::TCXO,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == HFXOSRC_A::XTAL
    }
    #[doc = "Checks if the value of the field is `TCXO`"]
    #[inline(always)]
    pub fn is_tcxo(&self) -> bool {
        *self == HFXOSRC_A::TCXO
    }
}
#[doc = "Write proxy for field `HFXOSRC`"]
pub struct HFXOSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXOSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "32 MHz crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(HFXOSRC_A::XTAL)
    }
    #[doc = "32 MHz temperature compensated crystal oscillator (TCXO)"]
    #[inline(always)]
    pub fn tcxo(self) -> &'a mut W {
        self.variant(HFXOSRC_A::TCXO)
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
impl R {
    #[doc = "Bit 0 - HFXO clock source selection"]
    #[inline(always)]
    pub fn hfxosrc(&self) -> HFXOSRC_R {
        HFXOSRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFXO clock source selection"]
    #[inline(always)]
    pub fn hfxosrc(&mut self) -> HFXOSRC_W {
        HFXOSRC_W { w: self }
    }
}

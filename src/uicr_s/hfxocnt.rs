#[doc = "Reader of register HFXOCNT"]
pub type R = crate::R<u32, super::HFXOCNT>;
#[doc = "Writer for register HFXOCNT"]
pub type W = crate::W<u32, super::HFXOCNT>;
#[doc = "Register HFXOCNT `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::HFXOCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFXOCNT_A {
    #[doc = "0: Min debounce time = (0*64 us + 0.5 us)"]
    MINDEBOUNCETIME = 0,
    #[doc = "255: Max debounce time = (255*64 us + 0.5 us)"]
    MAXDEBOUNCETIME = 255,
}
impl From<HFXOCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXOCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HFXOCNT`"]
pub type HFXOCNT_R = crate::R<u8, HFXOCNT_A>;
impl HFXOCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HFXOCNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HFXOCNT_A::MINDEBOUNCETIME),
            255 => Val(HFXOCNT_A::MAXDEBOUNCETIME),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MINDEBOUNCETIME`"]
    #[inline(always)]
    pub fn is_min_debounce_time(&self) -> bool {
        *self == HFXOCNT_A::MINDEBOUNCETIME
    }
    #[doc = "Checks if the value of the field is `MAXDEBOUNCETIME`"]
    #[inline(always)]
    pub fn is_max_debounce_time(&self) -> bool {
        *self == HFXOCNT_A::MAXDEBOUNCETIME
    }
}
#[doc = "Write proxy for field `HFXOCNT`"]
pub struct HFXOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFXOCNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Min debounce time = (0*64 us + 0.5 us)"]
    #[inline(always)]
    pub fn min_debounce_time(self) -> &'a mut W {
        self.variant(HFXOCNT_A::MINDEBOUNCETIME)
    }
    #[doc = "Max debounce time = (255*64 us + 0.5 us)"]
    #[inline(always)]
    pub fn max_debounce_time(self) -> &'a mut W {
        self.variant(HFXOCNT_A::MAXDEBOUNCETIME)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
    #[inline(always)]
    pub fn hfxocnt(&self) -> HFXOCNT_R {
        HFXOCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
    #[inline(always)]
    pub fn hfxocnt(&mut self) -> HFXOCNT_W {
        HFXOCNT_W { w: self }
    }
}

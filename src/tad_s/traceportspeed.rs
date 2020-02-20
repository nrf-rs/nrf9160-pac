#[doc = "Reader of register TRACEPORTSPEED"]
pub type R = crate::R<u32, super::TRACEPORTSPEED>;
#[doc = "Writer for register TRACEPORTSPEED"]
pub type W = crate::W<u32, super::TRACEPORTSPEED>;
#[doc = "Register TRACEPORTSPEED `reset()`'s with value 0"]
impl crate::ResetValue for super::TRACEPORTSPEED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRACEPORTSPEED_A {
    #[doc = "0: 32 MHz Trace Port clock (TRACECLK = 16 MHz)"]
    _32MHZ = 0,
    #[doc = "1: 16 MHz Trace Port clock (TRACECLK = 8 MHz)"]
    _16MHZ = 1,
    #[doc = "2: 8 MHz Trace Port clock (TRACECLK = 4 MHz)"]
    _8MHZ = 2,
    #[doc = "3: 4 MHz Trace Port clock (TRACECLK = 2 MHz)"]
    _4MHZ = 3,
}
impl From<TRACEPORTSPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACEPORTSPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRACEPORTSPEED`"]
pub type TRACEPORTSPEED_R = crate::R<u8, TRACEPORTSPEED_A>;
impl TRACEPORTSPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACEPORTSPEED_A {
        match self.bits {
            0 => TRACEPORTSPEED_A::_32MHZ,
            1 => TRACEPORTSPEED_A::_16MHZ,
            2 => TRACEPORTSPEED_A::_8MHZ,
            3 => TRACEPORTSPEED_A::_4MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32MHZ`"]
    #[inline(always)]
    pub fn is_32mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_32MHZ
    }
    #[doc = "Checks if the value of the field is `_16MHZ`"]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_16MHZ
    }
    #[doc = "Checks if the value of the field is `_8MHZ`"]
    #[inline(always)]
    pub fn is_8mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_8MHZ
    }
    #[doc = "Checks if the value of the field is `_4MHZ`"]
    #[inline(always)]
    pub fn is_4mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_4MHZ
    }
}
#[doc = "Write proxy for field `TRACEPORTSPEED`"]
pub struct TRACEPORTSPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEPORTSPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACEPORTSPEED_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32 MHz Trace Port clock (TRACECLK = 16 MHz)"]
    #[inline(always)]
    pub fn _32mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_32MHZ)
    }
    #[doc = "16 MHz Trace Port clock (TRACECLK = 8 MHz)"]
    #[inline(always)]
    pub fn _16mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_16MHZ)
    }
    #[doc = "8 MHz Trace Port clock (TRACECLK = 4 MHz)"]
    #[inline(always)]
    pub fn _8mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_8MHZ)
    }
    #[doc = "4 MHz Trace Port clock (TRACECLK = 2 MHz)"]
    #[inline(always)]
    pub fn _4mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_4MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline(always)]
    pub fn traceportspeed(&self) -> TRACEPORTSPEED_R {
        TRACEPORTSPEED_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline(always)]
    pub fn traceportspeed(&mut self) -> TRACEPORTSPEED_W {
        TRACEPORTSPEED_W { w: self }
    }
}

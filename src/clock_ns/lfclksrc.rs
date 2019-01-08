#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFCLKSRC {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCR {
    #[doc = "Reserved for future use (equals selecting LFRC)"]
    RFU,
    #[doc = "32.768 kHz RC oscillator"]
    LFRC,
    #[doc = "32.768 kHz crystal oscillator"]
    LFXO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCR::RFU => 0,
            SRCR::LFRC => 1,
            SRCR::LFXO => 2,
            SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCR {
        match value {
            0 => SRCR::RFU,
            1 => SRCR::LFRC,
            2 => SRCR::LFXO,
            i => SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RFU`"]
    #[inline]
    pub fn is_rfu(&self) -> bool {
        *self == SRCR::RFU
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline]
    pub fn is_lfrc(&self) -> bool {
        *self == SRCR::LFRC
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == SRCR::LFXO
    }
}
#[doc = "Values that can be written to the field `SRC`"]
pub enum SRCW {
    #[doc = "Reserved for future use (equals selecting LFRC)"]
    RFU,
    #[doc = "32.768 kHz RC oscillator"]
    LFRC,
    #[doc = "32.768 kHz crystal oscillator"]
    LFXO,
}
impl SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCW::RFU => 0,
            SRCW::LFRC => 1,
            SRCW::LFXO => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reserved for future use (equals selecting LFRC)"]
    #[inline]
    pub fn rfu(self) -> &'a mut W {
        self.variant(SRCW::RFU)
    }
    #[doc = "32.768 kHz RC oscillator"]
    #[inline]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(SRCW::LFRC)
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(SRCW::LFXO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Clock source"]
    #[inline]
    pub fn src(&self) -> SRCR {
        SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Clock source"]
    #[inline]
    pub fn src(&mut self) -> _SRCW {
        _SRCW { w: self }
    }
}

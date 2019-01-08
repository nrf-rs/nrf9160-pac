#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFXOCNT {
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
#[doc = "Possible values of the field `HFXOCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXOCNTR {
    #[doc = "Min debounce time = (0*64 us + 0.5 us)"]
    MINDEBOUNCETIME,
    #[doc = "Max debounce time = (255*64 us + 0.5 us)"]
    MAXDEBOUNCETIME,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HFXOCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HFXOCNTR::MINDEBOUNCETIME => 0,
            HFXOCNTR::MAXDEBOUNCETIME => 255,
            HFXOCNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HFXOCNTR {
        match value {
            0 => HFXOCNTR::MINDEBOUNCETIME,
            255 => HFXOCNTR::MAXDEBOUNCETIME,
            i => HFXOCNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MINDEBOUNCETIME`"]
    #[inline]
    pub fn is_min_debounce_time(&self) -> bool {
        *self == HFXOCNTR::MINDEBOUNCETIME
    }
    #[doc = "Checks if the value of the field is `MAXDEBOUNCETIME`"]
    #[inline]
    pub fn is_max_debounce_time(&self) -> bool {
        *self == HFXOCNTR::MAXDEBOUNCETIME
    }
}
#[doc = "Values that can be written to the field `HFXOCNT`"]
pub enum HFXOCNTW {
    #[doc = "Min debounce time = (0*64 us + 0.5 us)"]
    MINDEBOUNCETIME,
    #[doc = "Max debounce time = (255*64 us + 0.5 us)"]
    MAXDEBOUNCETIME,
}
impl HFXOCNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HFXOCNTW::MINDEBOUNCETIME => 0,
            HFXOCNTW::MAXDEBOUNCETIME => 255,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFXOCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _HFXOCNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFXOCNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Min debounce time = (0*64 us + 0.5 us)"]
    #[inline]
    pub fn min_debounce_time(self) -> &'a mut W {
        self.variant(HFXOCNTW::MINDEBOUNCETIME)
    }
    #[doc = "Max debounce time = (255*64 us + 0.5 us)"]
    #[inline]
    pub fn max_debounce_time(self) -> &'a mut W {
        self.variant(HFXOCNTW::MAXDEBOUNCETIME)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
    #[inline]
    pub fn hfxocnt(&self) -> HFXOCNTR {
        HFXOCNTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - HFXO startup counter. Total debounce time = HFXOCNT*64 us + 0.5 us"]
    #[inline]
    pub fn hfxocnt(&mut self) -> _HFXOCNTW {
        _HFXOCNTW { w: self }
    }
}

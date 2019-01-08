#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFXOSRC {
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
#[doc = "Possible values of the field `HFXOSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXOSRCR {
    #[doc = "32 MHz crystal oscillator"]
    XTAL,
    #[doc = "32 MHz temperature compensated crystal oscillator (TCXO)"]
    TCXO,
}
impl HFXOSRCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HFXOSRCR::XTAL => true,
            HFXOSRCR::TCXO => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HFXOSRCR {
        match value {
            true => HFXOSRCR::XTAL,
            false => HFXOSRCR::TCXO,
        }
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline]
    pub fn is_xtal(&self) -> bool {
        *self == HFXOSRCR::XTAL
    }
    #[doc = "Checks if the value of the field is `TCXO`"]
    #[inline]
    pub fn is_tcxo(&self) -> bool {
        *self == HFXOSRCR::TCXO
    }
}
#[doc = "Values that can be written to the field `HFXOSRC`"]
pub enum HFXOSRCW {
    #[doc = "32 MHz crystal oscillator"]
    XTAL,
    #[doc = "32 MHz temperature compensated crystal oscillator (TCXO)"]
    TCXO,
}
impl HFXOSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HFXOSRCW::XTAL => true,
            HFXOSRCW::TCXO => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFXOSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _HFXOSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFXOSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "32 MHz crystal oscillator"]
    #[inline]
    pub fn xtal(self) -> &'a mut W {
        self.variant(HFXOSRCW::XTAL)
    }
    #[doc = "32 MHz temperature compensated crystal oscillator (TCXO)"]
    #[inline]
    pub fn tcxo(self) -> &'a mut W {
        self.variant(HFXOSRCW::TCXO)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
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
    #[doc = "Bit 0 - HFXO clock source selection"]
    #[inline]
    pub fn hfxosrc(&self) -> HFXOSRCR {
        HFXOSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - HFXO clock source selection"]
    #[inline]
    pub fn hfxosrc(&mut self) -> _HFXOSRCW {
        _HFXOSRCW { w: self }
    }
}

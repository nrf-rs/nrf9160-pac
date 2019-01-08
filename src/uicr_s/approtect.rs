#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APPROTECT {
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
#[doc = "Possible values of the field `PALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PALLR {
    #[doc = "Unprotected"]
    UNPROTECTED,
    #[doc = "Protected"]
    PROTECTED,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl PALLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            PALLR::UNPROTECTED => 4294967295,
            PALLR::PROTECTED => 0,
            PALLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> PALLR {
        match value {
            4294967295 => PALLR::UNPROTECTED,
            0 => PALLR::PROTECTED,
            i => PALLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNPROTECTED`"]
    #[inline]
    pub fn is_unprotected(&self) -> bool {
        *self == PALLR::UNPROTECTED
    }
    #[doc = "Checks if the value of the field is `PROTECTED`"]
    #[inline]
    pub fn is_protected(&self) -> bool {
        *self == PALLR::PROTECTED
    }
}
#[doc = "Values that can be written to the field `PALL`"]
pub enum PALLW {
    #[doc = "Unprotected"]
    UNPROTECTED,
    #[doc = "Protected"]
    PROTECTED,
}
impl PALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            PALLW::UNPROTECTED => 4294967295,
            PALLW::PROTECTED => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PALLW<'a> {
    w: &'a mut W,
}
impl<'a> _PALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PALLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Unprotected"]
    #[inline]
    pub fn unprotected(self) -> &'a mut W {
        self.variant(PALLW::UNPROTECTED)
    }
    #[doc = "Protected"]
    #[inline]
    pub fn protected(self) -> &'a mut W {
        self.variant(PALLW::PROTECTED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - Blocks debugger read/write access to all CPU registers and memory mapped addresses"]
    #[inline]
    pub fn pall(&self) -> PALLR {
        PALLR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - Blocks debugger read/write access to all CPU registers and memory mapped addresses"]
    #[inline]
    pub fn pall(&mut self) -> _PALLW {
        _PALLW { w: self }
    }
}

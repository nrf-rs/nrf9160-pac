#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WRITEUICRNS {
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
}
#[doc = "Values that can be written to the field `SET`"]
pub enum SETW {
    #[doc = "Set value"]
    SET,
}
impl SETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SETW::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETW<'a> {
    w: &'a mut W,
}
impl<'a> _SETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Set value"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(SETW::SET)
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
#[doc = "Values that can be written to the field `KEY`"]
pub enum KEYW {
    #[doc = "Key value"]
    KEYVALID,
}
impl KEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            KEYW::KEYVALID => 184280487,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Key value"]
    #[inline]
    pub fn keyvalid(self) -> &'a mut W {
        self.variant(KEYW::KEYVALID)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 0 - Allow non-secure code to set APPROTECT"]
    #[inline]
    pub fn set(&mut self) -> _SETW {
        _SETW { w: self }
    }
    #[doc = "Bits 4:31 - Key to write in order to validate the write operation"]
    #[inline]
    pub fn key(&mut self) -> _KEYW {
        _KEYW { w: self }
    }
}

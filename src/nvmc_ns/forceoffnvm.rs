#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FORCEOFFNVM {
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
#[doc = "Possible values of the field `FORCEOFFNVM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEOFFNVM0R {
    #[doc = "Do not force off supply"]
    DONOTFORCEOFF,
    #[doc = "Force off supply"]
    FORCEOFF,
}
impl FORCEOFFNVM0R {
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
            FORCEOFFNVM0R::DONOTFORCEOFF => false,
            FORCEOFFNVM0R::FORCEOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCEOFFNVM0R {
        match value {
            false => FORCEOFFNVM0R::DONOTFORCEOFF,
            true => FORCEOFFNVM0R::FORCEOFF,
        }
    }
    #[doc = "Checks if the value of the field is `DONOTFORCEOFF`"]
    #[inline]
    pub fn is_do_not_force_off(&self) -> bool {
        *self == FORCEOFFNVM0R::DONOTFORCEOFF
    }
    #[doc = "Checks if the value of the field is `FORCEOFF`"]
    #[inline]
    pub fn is_force_off(&self) -> bool {
        *self == FORCEOFFNVM0R::FORCEOFF
    }
}
#[doc = "Possible values of the field `FORCEOFFNVM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEOFFNVM1R {
    #[doc = "Do not force off supply"]
    DONOTFORCEOFF,
    #[doc = "Force off supply"]
    FORCEOFF,
}
impl FORCEOFFNVM1R {
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
            FORCEOFFNVM1R::DONOTFORCEOFF => false,
            FORCEOFFNVM1R::FORCEOFF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCEOFFNVM1R {
        match value {
            false => FORCEOFFNVM1R::DONOTFORCEOFF,
            true => FORCEOFFNVM1R::FORCEOFF,
        }
    }
    #[doc = "Checks if the value of the field is `DONOTFORCEOFF`"]
    #[inline]
    pub fn is_do_not_force_off(&self) -> bool {
        *self == FORCEOFFNVM1R::DONOTFORCEOFF
    }
    #[doc = "Checks if the value of the field is `FORCEOFF`"]
    #[inline]
    pub fn is_force_off(&self) -> bool {
        *self == FORCEOFFNVM1R::FORCEOFF
    }
}
#[doc = "Possible values of the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYR {
    #[doc = "Must be written in order to write to bits 0-7. Any other value will ignore writes to this register. Read as zero."]
    ENABLEWRITE,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl KEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            KEYR::ENABLEWRITE => 11325013,
            KEYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> KEYR {
        match value {
            11325013 => KEYR::ENABLEWRITE,
            i => KEYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLEWRITE`"]
    #[inline]
    pub fn is_enable_write(&self) -> bool {
        *self == KEYR::ENABLEWRITE
    }
}
#[doc = "Values that can be written to the field `FORCEOFFNVM0`"]
pub enum FORCEOFFNVM0W {
    #[doc = "Do not force off supply"]
    DONOTFORCEOFF,
    #[doc = "Force off supply"]
    FORCEOFF,
}
impl FORCEOFFNVM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FORCEOFFNVM0W::DONOTFORCEOFF => false,
            FORCEOFFNVM0W::FORCEOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORCEOFFNVM0W<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEOFFNVM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCEOFFNVM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not force off supply"]
    #[inline]
    pub fn do_not_force_off(self) -> &'a mut W {
        self.variant(FORCEOFFNVM0W::DONOTFORCEOFF)
    }
    #[doc = "Force off supply"]
    #[inline]
    pub fn force_off(self) -> &'a mut W {
        self.variant(FORCEOFFNVM0W::FORCEOFF)
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
#[doc = "Values that can be written to the field `FORCEOFFNVM1`"]
pub enum FORCEOFFNVM1W {
    #[doc = "Do not force off supply"]
    DONOTFORCEOFF,
    #[doc = "Force off supply"]
    FORCEOFF,
}
impl FORCEOFFNVM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FORCEOFFNVM1W::DONOTFORCEOFF => false,
            FORCEOFFNVM1W::FORCEOFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FORCEOFFNVM1W<'a> {
    w: &'a mut W,
}
impl<'a> _FORCEOFFNVM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCEOFFNVM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not force off supply"]
    #[inline]
    pub fn do_not_force_off(self) -> &'a mut W {
        self.variant(FORCEOFFNVM1W::DONOTFORCEOFF)
    }
    #[doc = "Force off supply"]
    #[inline]
    pub fn force_off(self) -> &'a mut W {
        self.variant(FORCEOFFNVM1W::FORCEOFF)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `KEY`"]
pub enum KEYW {
    #[doc = "Must be written in order to write to bits 0-7. Any other value will ignore writes to this register. Read as zero."]
    ENABLEWRITE,
}
impl KEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            KEYW::ENABLEWRITE => 11325013,
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
    #[doc = "Must be written in order to write to bits 0-7. Any other value will ignore writes to this register. Read as zero."]
    #[inline]
    pub fn enable_write(self) -> &'a mut W {
        self.variant(KEYW::ENABLEWRITE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Force off NVM supply 0. Also see the internal section in the NVMC chapter."]
    #[inline]
    pub fn forceoffnvm0(&self) -> FORCEOFFNVM0R {
        FORCEOFFNVM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Force off NVM supply 1. Also see the internal section in the NVMC chapter."]
    #[inline]
    pub fn forceoffnvm1(&self) -> FORCEOFFNVM1R {
        FORCEOFFNVM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:31 - KEY"]
    #[inline]
    pub fn key(&self) -> KEYR {
        KEYR::_from({
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Force off NVM supply 0. Also see the internal section in the NVMC chapter."]
    #[inline]
    pub fn forceoffnvm0(&mut self) -> _FORCEOFFNVM0W {
        _FORCEOFFNVM0W { w: self }
    }
    #[doc = "Bit 1 - Force off NVM supply 1. Also see the internal section in the NVMC chapter."]
    #[inline]
    pub fn forceoffnvm1(&mut self) -> _FORCEOFFNVM1W {
        _FORCEOFFNVM1W { w: self }
    }
    #[doc = "Bits 8:31 - KEY"]
    #[inline]
    pub fn key(&mut self) -> _KEYW {
        _KEYW { w: self }
    }
}

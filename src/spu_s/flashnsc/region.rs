#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REGION {
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
#[doc = r" Value of the field"]
pub struct REGIONR {
    bits: u8,
}
impl REGIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKR {
    #[doc = "This register can be updated"]
    UNLOCKED,
    #[doc = "The content of this register can't be changed until the next reset"]
    LOCKED,
}
impl LOCKR {
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
            LOCKR::UNLOCKED => false,
            LOCKR::LOCKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKR {
        match value {
            false => LOCKR::UNLOCKED,
            true => LOCKR::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKR::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline]
    pub fn is_locked(&self) -> bool {
        *self == LOCKR::LOCKED
    }
}
#[doc = r" Proxy"]
pub struct _REGIONW<'a> {
    w: &'a mut W,
}
impl<'a> _REGIONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOCK`"]
pub enum LOCKW {
    #[doc = "This register can be updated"]
    UNLOCKED,
    #[doc = "The content of this register can't be changed until the next reset"]
    LOCKED,
}
impl LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKW::UNLOCKED => false,
            LOCKW::LOCKED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This register can be updated"]
    #[inline]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKW::UNLOCKED)
    }
    #[doc = "The content of this register can't be changed until the next reset"]
    #[inline]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKW::LOCKED)
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
    #[doc = "Bits 0:4 - Region number"]
    #[inline]
    pub fn region(&self) -> REGIONR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REGIONR { bits }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn lock(&self) -> LOCKR {
        LOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:4 - Region number"]
    #[inline]
    pub fn region(&mut self) -> _REGIONW {
        _REGIONW { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
}

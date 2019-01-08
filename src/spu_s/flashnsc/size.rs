#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SIZE {
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
#[doc = "Possible values of the field `SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZER {
    #[doc = "The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
    DISABLED,
    #[doc = "The region n is defined as non-secure callable with a 32-byte size"]
    _32,
    #[doc = "The region n is defined as non-secure callable with a 64-byte size"]
    _64,
    #[doc = "The region n is defined as non-secure callable with a 128-byte size"]
    _128,
    #[doc = "The region n is defined as non-secure callable with a 256-byte size"]
    _256,
    #[doc = "The region n is defined as non-secure callable with a 512-byte size"]
    _512,
    #[doc = "The region n is defined as non-secure callable with a 1024-byte size"]
    _1024,
    #[doc = "The region n is defined as non-secure callable with a 2048-byte size"]
    _2048,
    #[doc = "The region n is defined as non-secure callable with a 4096-byte size"]
    _4096,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SIZER::DISABLED => 0,
            SIZER::_32 => 1,
            SIZER::_64 => 2,
            SIZER::_128 => 3,
            SIZER::_256 => 4,
            SIZER::_512 => 5,
            SIZER::_1024 => 6,
            SIZER::_2048 => 7,
            SIZER::_4096 => 8,
            SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SIZER {
        match value {
            0 => SIZER::DISABLED,
            1 => SIZER::_32,
            2 => SIZER::_64,
            3 => SIZER::_128,
            4 => SIZER::_256,
            5 => SIZER::_512,
            6 => SIZER::_1024,
            7 => SIZER::_2048,
            8 => SIZER::_4096,
            i => SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SIZER::DISABLED
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == SIZER::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == SIZER::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline]
    pub fn is_128(&self) -> bool {
        *self == SIZER::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline]
    pub fn is_256(&self) -> bool {
        *self == SIZER::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == SIZER::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline]
    pub fn is_1024(&self) -> bool {
        *self == SIZER::_1024
    }
    #[doc = "Checks if the value of the field is `_2048`"]
    #[inline]
    pub fn is_2048(&self) -> bool {
        *self == SIZER::_2048
    }
    #[doc = "Checks if the value of the field is `_4096`"]
    #[inline]
    pub fn is_4096(&self) -> bool {
        *self == SIZER::_4096
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
#[doc = "Values that can be written to the field `SIZE`"]
pub enum SIZEW {
    #[doc = "The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
    DISABLED,
    #[doc = "The region n is defined as non-secure callable with a 32-byte size"]
    _32,
    #[doc = "The region n is defined as non-secure callable with a 64-byte size"]
    _64,
    #[doc = "The region n is defined as non-secure callable with a 128-byte size"]
    _128,
    #[doc = "The region n is defined as non-secure callable with a 256-byte size"]
    _256,
    #[doc = "The region n is defined as non-secure callable with a 512-byte size"]
    _512,
    #[doc = "The region n is defined as non-secure callable with a 1024-byte size"]
    _1024,
    #[doc = "The region n is defined as non-secure callable with a 2048-byte size"]
    _2048,
    #[doc = "The region n is defined as non-secure callable with a 4096-byte size"]
    _4096,
}
impl SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SIZEW::DISABLED => 0,
            SIZEW::_32 => 1,
            SIZEW::_64 => 2,
            SIZEW::_128 => 3,
            SIZEW::_256 => 4,
            SIZEW::_512 => 5,
            SIZEW::_1024 => 6,
            SIZEW::_2048 => 7,
            SIZEW::_4096 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SIZEW::DISABLED)
    }
    #[doc = "The region n is defined as non-secure callable with a 32-byte size"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(SIZEW::_32)
    }
    #[doc = "The region n is defined as non-secure callable with a 64-byte size"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(SIZEW::_64)
    }
    #[doc = "The region n is defined as non-secure callable with a 128-byte size"]
    #[inline]
    pub fn _128(self) -> &'a mut W {
        self.variant(SIZEW::_128)
    }
    #[doc = "The region n is defined as non-secure callable with a 256-byte size"]
    #[inline]
    pub fn _256(self) -> &'a mut W {
        self.variant(SIZEW::_256)
    }
    #[doc = "The region n is defined as non-secure callable with a 512-byte size"]
    #[inline]
    pub fn _512(self) -> &'a mut W {
        self.variant(SIZEW::_512)
    }
    #[doc = "The region n is defined as non-secure callable with a 1024-byte size"]
    #[inline]
    pub fn _1024(self) -> &'a mut W {
        self.variant(SIZEW::_1024)
    }
    #[doc = "The region n is defined as non-secure callable with a 2048-byte size"]
    #[inline]
    pub fn _2048(self) -> &'a mut W {
        self.variant(SIZEW::_2048)
    }
    #[doc = "The region n is defined as non-secure callable with a 4096-byte size"]
    #[inline]
    pub fn _4096(self) -> &'a mut W {
        self.variant(SIZEW::_4096)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Size of the non-secure callable (NSC) region n"]
    #[inline]
    pub fn size(&self) -> SIZER {
        SIZER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:3 - Size of the non-secure callable (NSC) region n"]
    #[inline]
    pub fn size(&mut self) -> _SIZEW {
        _SIZEW { w: self }
    }
    #[doc = "Bit 8"]
    #[inline]
    pub fn lock(&mut self) -> _LOCKW {
        _LOCKW { w: self }
    }
}

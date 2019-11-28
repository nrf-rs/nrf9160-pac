#[doc = "Reader of register SIZE"]
pub type R = crate::R<u32, super::SIZE>;
#[doc = "Writer for register SIZE"]
pub type W = crate::W<u32, super::SIZE>;
#[doc = "Register SIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::SIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Size of the non-secure callable (NSC) region n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZE_A {
    #[doc = "0: The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
    DISABLED,
    #[doc = "1: The region n is defined as non-secure callable with a 32-byte size"]
    _32,
    #[doc = "2: The region n is defined as non-secure callable with a 64-byte size"]
    _64,
    #[doc = "3: The region n is defined as non-secure callable with a 128-byte size"]
    _128,
    #[doc = "4: The region n is defined as non-secure callable with a 256-byte size"]
    _256,
    #[doc = "5: The region n is defined as non-secure callable with a 512-byte size"]
    _512,
    #[doc = "6: The region n is defined as non-secure callable with a 1024-byte size"]
    _1024,
    #[doc = "7: The region n is defined as non-secure callable with a 2048-byte size"]
    _2048,
    #[doc = "8: The region n is defined as non-secure callable with a 4096-byte size"]
    _4096,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        match variant {
            SIZE_A::DISABLED => 0,
            SIZE_A::_32 => 1,
            SIZE_A::_64 => 2,
            SIZE_A::_128 => 3,
            SIZE_A::_256 => 4,
            SIZE_A::_512 => 5,
            SIZE_A::_1024 => 6,
            SIZE_A::_2048 => 7,
            SIZE_A::_4096 => 8,
        }
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, SIZE_A>;
impl SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SIZE_A::DISABLED),
            1 => Val(SIZE_A::_32),
            2 => Val(SIZE_A::_64),
            3 => Val(SIZE_A::_128),
            4 => Val(SIZE_A::_256),
            5 => Val(SIZE_A::_512),
            6 => Val(SIZE_A::_1024),
            7 => Val(SIZE_A::_2048),
            8 => Val(SIZE_A::_4096),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SIZE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == SIZE_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == SIZE_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == SIZE_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == SIZE_A::_256
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == SIZE_A::_512
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == SIZE_A::_1024
    }
    #[doc = "Checks if the value of the field is `_2048`"]
    #[inline(always)]
    pub fn is_2048(&self) -> bool {
        *self == SIZE_A::_2048
    }
    #[doc = "Checks if the value of the field is `_4096`"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == SIZE_A::_4096
    }
}
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The region n is not defined as a non-secure callable region. Normal security attributes (secure or non-secure) are enforced."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SIZE_A::DISABLED)
    }
    #[doc = "The region n is defined as non-secure callable with a 32-byte size"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(SIZE_A::_32)
    }
    #[doc = "The region n is defined as non-secure callable with a 64-byte size"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(SIZE_A::_64)
    }
    #[doc = "The region n is defined as non-secure callable with a 128-byte size"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(SIZE_A::_128)
    }
    #[doc = "The region n is defined as non-secure callable with a 256-byte size"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(SIZE_A::_256)
    }
    #[doc = "The region n is defined as non-secure callable with a 512-byte size"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(SIZE_A::_512)
    }
    #[doc = "The region n is defined as non-secure callable with a 1024-byte size"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(SIZE_A::_1024)
    }
    #[doc = "The region n is defined as non-secure callable with a 2048-byte size"]
    #[inline(always)]
    pub fn _2048(self) -> &'a mut W {
        self.variant(SIZE_A::_2048)
    }
    #[doc = "The region n is defined as non-secure callable with a 4096-byte size"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut W {
        self.variant(SIZE_A::_4096)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: This register can be updated"]
    UNLOCKED,
    #[doc = "1: The content of this register can't be changed until the next reset"]
    LOCKED,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        match variant {
            LOCK_A::UNLOCKED => false,
            LOCK_A::LOCKED => true,
        }
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This register can be updated"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "The content of this register can't be changed until the next reset"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Size of the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Size of the non-secure callable (NSC) region n"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}

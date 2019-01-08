#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PERM {
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
#[doc = "Possible values of the field `CHANNEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL0R {
    #[doc = "Channel0 has its secure attribute set"]
    SECURE,
    #[doc = "Channel0 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL0R {
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
            CHANNEL0R::SECURE => true,
            CHANNEL0R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL0R {
        match value {
            true => CHANNEL0R::SECURE,
            false => CHANNEL0R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL0R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL0R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL1R {
    #[doc = "Channel1 has its secure attribute set"]
    SECURE,
    #[doc = "Channel1 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL1R {
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
            CHANNEL1R::SECURE => true,
            CHANNEL1R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL1R {
        match value {
            true => CHANNEL1R::SECURE,
            false => CHANNEL1R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL1R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL1R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL2R {
    #[doc = "Channel2 has its secure attribute set"]
    SECURE,
    #[doc = "Channel2 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL2R {
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
            CHANNEL2R::SECURE => true,
            CHANNEL2R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL2R {
        match value {
            true => CHANNEL2R::SECURE,
            false => CHANNEL2R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL2R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL2R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL3R {
    #[doc = "Channel3 has its secure attribute set"]
    SECURE,
    #[doc = "Channel3 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL3R {
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
            CHANNEL3R::SECURE => true,
            CHANNEL3R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL3R {
        match value {
            true => CHANNEL3R::SECURE,
            false => CHANNEL3R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL3R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL3R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL4R {
    #[doc = "Channel4 has its secure attribute set"]
    SECURE,
    #[doc = "Channel4 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL4R {
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
            CHANNEL4R::SECURE => true,
            CHANNEL4R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL4R {
        match value {
            true => CHANNEL4R::SECURE,
            false => CHANNEL4R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL4R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL4R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL5R {
    #[doc = "Channel5 has its secure attribute set"]
    SECURE,
    #[doc = "Channel5 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL5R {
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
            CHANNEL5R::SECURE => true,
            CHANNEL5R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL5R {
        match value {
            true => CHANNEL5R::SECURE,
            false => CHANNEL5R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL5R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL5R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL6R {
    #[doc = "Channel6 has its secure attribute set"]
    SECURE,
    #[doc = "Channel6 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL6R {
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
            CHANNEL6R::SECURE => true,
            CHANNEL6R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL6R {
        match value {
            true => CHANNEL6R::SECURE,
            false => CHANNEL6R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL6R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL6R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL7R {
    #[doc = "Channel7 has its secure attribute set"]
    SECURE,
    #[doc = "Channel7 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL7R {
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
            CHANNEL7R::SECURE => true,
            CHANNEL7R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL7R {
        match value {
            true => CHANNEL7R::SECURE,
            false => CHANNEL7R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL7R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL7R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL8R {
    #[doc = "Channel8 has its secure attribute set"]
    SECURE,
    #[doc = "Channel8 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL8R {
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
            CHANNEL8R::SECURE => true,
            CHANNEL8R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL8R {
        match value {
            true => CHANNEL8R::SECURE,
            false => CHANNEL8R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL8R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL8R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL9R {
    #[doc = "Channel9 has its secure attribute set"]
    SECURE,
    #[doc = "Channel9 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL9R {
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
            CHANNEL9R::SECURE => true,
            CHANNEL9R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL9R {
        match value {
            true => CHANNEL9R::SECURE,
            false => CHANNEL9R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL9R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL9R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL10R {
    #[doc = "Channel10 has its secure attribute set"]
    SECURE,
    #[doc = "Channel10 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL10R {
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
            CHANNEL10R::SECURE => true,
            CHANNEL10R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL10R {
        match value {
            true => CHANNEL10R::SECURE,
            false => CHANNEL10R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL10R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL10R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL11R {
    #[doc = "Channel11 has its secure attribute set"]
    SECURE,
    #[doc = "Channel11 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL11R {
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
            CHANNEL11R::SECURE => true,
            CHANNEL11R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL11R {
        match value {
            true => CHANNEL11R::SECURE,
            false => CHANNEL11R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL11R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL11R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL12R {
    #[doc = "Channel12 has its secure attribute set"]
    SECURE,
    #[doc = "Channel12 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL12R {
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
            CHANNEL12R::SECURE => true,
            CHANNEL12R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL12R {
        match value {
            true => CHANNEL12R::SECURE,
            false => CHANNEL12R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL12R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL12R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL13R {
    #[doc = "Channel13 has its secure attribute set"]
    SECURE,
    #[doc = "Channel13 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL13R {
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
            CHANNEL13R::SECURE => true,
            CHANNEL13R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL13R {
        match value {
            true => CHANNEL13R::SECURE,
            false => CHANNEL13R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL13R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL13R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL14R {
    #[doc = "Channel14 has its secure attribute set"]
    SECURE,
    #[doc = "Channel14 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL14R {
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
            CHANNEL14R::SECURE => true,
            CHANNEL14R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL14R {
        match value {
            true => CHANNEL14R::SECURE,
            false => CHANNEL14R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL14R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL14R::NONSECURE
    }
}
#[doc = "Possible values of the field `CHANNEL15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHANNEL15R {
    #[doc = "Channel15 has its secure attribute set"]
    SECURE,
    #[doc = "Channel15 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL15R {
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
            CHANNEL15R::SECURE => true,
            CHANNEL15R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHANNEL15R {
        match value {
            true => CHANNEL15R::SECURE,
            false => CHANNEL15R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == CHANNEL15R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == CHANNEL15R::NONSECURE
    }
}
#[doc = "Values that can be written to the field `CHANNEL0`"]
pub enum CHANNEL0W {
    #[doc = "Channel0 has its secure attribute set"]
    SECURE,
    #[doc = "Channel0 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL0W::SECURE => true,
            CHANNEL0W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel0 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL0W::SECURE)
    }
    #[doc = "Channel0 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL0W::NONSECURE)
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
#[doc = "Values that can be written to the field `CHANNEL1`"]
pub enum CHANNEL1W {
    #[doc = "Channel1 has its secure attribute set"]
    SECURE,
    #[doc = "Channel1 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL1W::SECURE => true,
            CHANNEL1W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel1 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL1W::SECURE)
    }
    #[doc = "Channel1 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL1W::NONSECURE)
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
#[doc = "Values that can be written to the field `CHANNEL2`"]
pub enum CHANNEL2W {
    #[doc = "Channel2 has its secure attribute set"]
    SECURE,
    #[doc = "Channel2 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL2W::SECURE => true,
            CHANNEL2W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel2 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL2W::SECURE)
    }
    #[doc = "Channel2 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL2W::NONSECURE)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL3`"]
pub enum CHANNEL3W {
    #[doc = "Channel3 has its secure attribute set"]
    SECURE,
    #[doc = "Channel3 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL3W::SECURE => true,
            CHANNEL3W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel3 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL3W::SECURE)
    }
    #[doc = "Channel3 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL3W::NONSECURE)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL4`"]
pub enum CHANNEL4W {
    #[doc = "Channel4 has its secure attribute set"]
    SECURE,
    #[doc = "Channel4 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL4W::SECURE => true,
            CHANNEL4W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL4W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel4 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL4W::SECURE)
    }
    #[doc = "Channel4 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL4W::NONSECURE)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL5`"]
pub enum CHANNEL5W {
    #[doc = "Channel5 has its secure attribute set"]
    SECURE,
    #[doc = "Channel5 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL5W::SECURE => true,
            CHANNEL5W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL5W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel5 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL5W::SECURE)
    }
    #[doc = "Channel5 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL5W::NONSECURE)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL6`"]
pub enum CHANNEL6W {
    #[doc = "Channel6 has its secure attribute set"]
    SECURE,
    #[doc = "Channel6 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL6W::SECURE => true,
            CHANNEL6W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL6W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel6 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL6W::SECURE)
    }
    #[doc = "Channel6 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL6W::NONSECURE)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL7`"]
pub enum CHANNEL7W {
    #[doc = "Channel7 has its secure attribute set"]
    SECURE,
    #[doc = "Channel7 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL7W::SECURE => true,
            CHANNEL7W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL7W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel7 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL7W::SECURE)
    }
    #[doc = "Channel7 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL7W::NONSECURE)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL8`"]
pub enum CHANNEL8W {
    #[doc = "Channel8 has its secure attribute set"]
    SECURE,
    #[doc = "Channel8 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL8W::SECURE => true,
            CHANNEL8W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL8W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel8 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL8W::SECURE)
    }
    #[doc = "Channel8 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL8W::NONSECURE)
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
#[doc = "Values that can be written to the field `CHANNEL9`"]
pub enum CHANNEL9W {
    #[doc = "Channel9 has its secure attribute set"]
    SECURE,
    #[doc = "Channel9 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL9W::SECURE => true,
            CHANNEL9W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL9W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel9 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL9W::SECURE)
    }
    #[doc = "Channel9 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL9W::NONSECURE)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL10`"]
pub enum CHANNEL10W {
    #[doc = "Channel10 has its secure attribute set"]
    SECURE,
    #[doc = "Channel10 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL10W::SECURE => true,
            CHANNEL10W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL10W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel10 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL10W::SECURE)
    }
    #[doc = "Channel10 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL10W::NONSECURE)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL11`"]
pub enum CHANNEL11W {
    #[doc = "Channel11 has its secure attribute set"]
    SECURE,
    #[doc = "Channel11 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL11W::SECURE => true,
            CHANNEL11W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL11W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel11 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL11W::SECURE)
    }
    #[doc = "Channel11 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL11W::NONSECURE)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL12`"]
pub enum CHANNEL12W {
    #[doc = "Channel12 has its secure attribute set"]
    SECURE,
    #[doc = "Channel12 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL12W::SECURE => true,
            CHANNEL12W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL12W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel12 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL12W::SECURE)
    }
    #[doc = "Channel12 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL12W::NONSECURE)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL13`"]
pub enum CHANNEL13W {
    #[doc = "Channel13 has its secure attribute set"]
    SECURE,
    #[doc = "Channel13 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL13W::SECURE => true,
            CHANNEL13W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL13W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel13 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL13W::SECURE)
    }
    #[doc = "Channel13 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL13W::NONSECURE)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL14`"]
pub enum CHANNEL14W {
    #[doc = "Channel14 has its secure attribute set"]
    SECURE,
    #[doc = "Channel14 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL14W::SECURE => true,
            CHANNEL14W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL14W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel14 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL14W::SECURE)
    }
    #[doc = "Channel14 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL14W::NONSECURE)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHANNEL15`"]
pub enum CHANNEL15W {
    #[doc = "Channel15 has its secure attribute set"]
    SECURE,
    #[doc = "Channel15 has its non-secure attribute set"]
    NONSECURE,
}
impl CHANNEL15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHANNEL15W::SECURE => true,
            CHANNEL15W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHANNEL15W<'a> {
    w: &'a mut W,
}
impl<'a> _CHANNEL15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHANNEL15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel15 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(CHANNEL15W::SECURE)
    }
    #[doc = "Channel15 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(CHANNEL15W::NONSECURE)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Select secure attribute."]
    #[inline]
    pub fn channel0(&self) -> CHANNEL0R {
        CHANNEL0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Select secure attribute."]
    #[inline]
    pub fn channel1(&self) -> CHANNEL1R {
        CHANNEL1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Select secure attribute."]
    #[inline]
    pub fn channel2(&self) -> CHANNEL2R {
        CHANNEL2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Select secure attribute."]
    #[inline]
    pub fn channel3(&self) -> CHANNEL3R {
        CHANNEL3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Select secure attribute."]
    #[inline]
    pub fn channel4(&self) -> CHANNEL4R {
        CHANNEL4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Select secure attribute."]
    #[inline]
    pub fn channel5(&self) -> CHANNEL5R {
        CHANNEL5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Select secure attribute."]
    #[inline]
    pub fn channel6(&self) -> CHANNEL6R {
        CHANNEL6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Select secure attribute."]
    #[inline]
    pub fn channel7(&self) -> CHANNEL7R {
        CHANNEL7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Select secure attribute."]
    #[inline]
    pub fn channel8(&self) -> CHANNEL8R {
        CHANNEL8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Select secure attribute."]
    #[inline]
    pub fn channel9(&self) -> CHANNEL9R {
        CHANNEL9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Select secure attribute."]
    #[inline]
    pub fn channel10(&self) -> CHANNEL10R {
        CHANNEL10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Select secure attribute."]
    #[inline]
    pub fn channel11(&self) -> CHANNEL11R {
        CHANNEL11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Select secure attribute."]
    #[inline]
    pub fn channel12(&self) -> CHANNEL12R {
        CHANNEL12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Select secure attribute."]
    #[inline]
    pub fn channel13(&self) -> CHANNEL13R {
        CHANNEL13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Select secure attribute."]
    #[inline]
    pub fn channel14(&self) -> CHANNEL14R {
        CHANNEL14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Select secure attribute."]
    #[inline]
    pub fn channel15(&self) -> CHANNEL15R {
        CHANNEL15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65535 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Select secure attribute."]
    #[inline]
    pub fn channel0(&mut self) -> _CHANNEL0W {
        _CHANNEL0W { w: self }
    }
    #[doc = "Bit 1 - Select secure attribute."]
    #[inline]
    pub fn channel1(&mut self) -> _CHANNEL1W {
        _CHANNEL1W { w: self }
    }
    #[doc = "Bit 2 - Select secure attribute."]
    #[inline]
    pub fn channel2(&mut self) -> _CHANNEL2W {
        _CHANNEL2W { w: self }
    }
    #[doc = "Bit 3 - Select secure attribute."]
    #[inline]
    pub fn channel3(&mut self) -> _CHANNEL3W {
        _CHANNEL3W { w: self }
    }
    #[doc = "Bit 4 - Select secure attribute."]
    #[inline]
    pub fn channel4(&mut self) -> _CHANNEL4W {
        _CHANNEL4W { w: self }
    }
    #[doc = "Bit 5 - Select secure attribute."]
    #[inline]
    pub fn channel5(&mut self) -> _CHANNEL5W {
        _CHANNEL5W { w: self }
    }
    #[doc = "Bit 6 - Select secure attribute."]
    #[inline]
    pub fn channel6(&mut self) -> _CHANNEL6W {
        _CHANNEL6W { w: self }
    }
    #[doc = "Bit 7 - Select secure attribute."]
    #[inline]
    pub fn channel7(&mut self) -> _CHANNEL7W {
        _CHANNEL7W { w: self }
    }
    #[doc = "Bit 8 - Select secure attribute."]
    #[inline]
    pub fn channel8(&mut self) -> _CHANNEL8W {
        _CHANNEL8W { w: self }
    }
    #[doc = "Bit 9 - Select secure attribute."]
    #[inline]
    pub fn channel9(&mut self) -> _CHANNEL9W {
        _CHANNEL9W { w: self }
    }
    #[doc = "Bit 10 - Select secure attribute."]
    #[inline]
    pub fn channel10(&mut self) -> _CHANNEL10W {
        _CHANNEL10W { w: self }
    }
    #[doc = "Bit 11 - Select secure attribute."]
    #[inline]
    pub fn channel11(&mut self) -> _CHANNEL11W {
        _CHANNEL11W { w: self }
    }
    #[doc = "Bit 12 - Select secure attribute."]
    #[inline]
    pub fn channel12(&mut self) -> _CHANNEL12W {
        _CHANNEL12W { w: self }
    }
    #[doc = "Bit 13 - Select secure attribute."]
    #[inline]
    pub fn channel13(&mut self) -> _CHANNEL13W {
        _CHANNEL13W { w: self }
    }
    #[doc = "Bit 14 - Select secure attribute."]
    #[inline]
    pub fn channel14(&mut self) -> _CHANNEL14W {
        _CHANNEL14W { w: self }
    }
    #[doc = "Bit 15 - Select secure attribute."]
    #[inline]
    pub fn channel15(&mut self) -> _CHANNEL15W {
        _CHANNEL15W { w: self }
    }
}

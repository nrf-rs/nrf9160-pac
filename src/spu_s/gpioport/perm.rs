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
#[doc = "Possible values of the field `PIN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN0R {
    #[doc = "Pin 0 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 0 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN0R {
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
            PIN0R::SECURE => true,
            PIN0R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN0R {
        match value {
            true => PIN0R::SECURE,
            false => PIN0R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN0R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN0R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN1R {
    #[doc = "Pin 1 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 1 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN1R {
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
            PIN1R::SECURE => true,
            PIN1R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN1R {
        match value {
            true => PIN1R::SECURE,
            false => PIN1R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN1R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN1R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN2R {
    #[doc = "Pin 2 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 2 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN2R {
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
            PIN2R::SECURE => true,
            PIN2R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN2R {
        match value {
            true => PIN2R::SECURE,
            false => PIN2R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN2R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN2R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN3R {
    #[doc = "Pin 3 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 3 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN3R {
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
            PIN3R::SECURE => true,
            PIN3R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN3R {
        match value {
            true => PIN3R::SECURE,
            false => PIN3R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN3R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN3R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN4R {
    #[doc = "Pin 4 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 4 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN4R {
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
            PIN4R::SECURE => true,
            PIN4R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN4R {
        match value {
            true => PIN4R::SECURE,
            false => PIN4R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN4R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN4R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN5R {
    #[doc = "Pin 5 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 5 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN5R {
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
            PIN5R::SECURE => true,
            PIN5R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN5R {
        match value {
            true => PIN5R::SECURE,
            false => PIN5R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN5R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN5R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN6R {
    #[doc = "Pin 6 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 6 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN6R {
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
            PIN6R::SECURE => true,
            PIN6R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN6R {
        match value {
            true => PIN6R::SECURE,
            false => PIN6R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN6R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN6R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN7R {
    #[doc = "Pin 7 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 7 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN7R {
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
            PIN7R::SECURE => true,
            PIN7R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN7R {
        match value {
            true => PIN7R::SECURE,
            false => PIN7R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN7R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN7R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN8R {
    #[doc = "Pin 8 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 8 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN8R {
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
            PIN8R::SECURE => true,
            PIN8R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN8R {
        match value {
            true => PIN8R::SECURE,
            false => PIN8R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN8R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN8R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN9R {
    #[doc = "Pin 9 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 9 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN9R {
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
            PIN9R::SECURE => true,
            PIN9R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN9R {
        match value {
            true => PIN9R::SECURE,
            false => PIN9R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN9R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN9R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN10R {
    #[doc = "Pin 10 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 10 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN10R {
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
            PIN10R::SECURE => true,
            PIN10R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN10R {
        match value {
            true => PIN10R::SECURE,
            false => PIN10R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN10R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN10R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN11R {
    #[doc = "Pin 11 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 11 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN11R {
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
            PIN11R::SECURE => true,
            PIN11R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN11R {
        match value {
            true => PIN11R::SECURE,
            false => PIN11R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN11R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN11R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN12R {
    #[doc = "Pin 12 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 12 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN12R {
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
            PIN12R::SECURE => true,
            PIN12R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN12R {
        match value {
            true => PIN12R::SECURE,
            false => PIN12R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN12R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN12R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN13R {
    #[doc = "Pin 13 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 13 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN13R {
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
            PIN13R::SECURE => true,
            PIN13R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN13R {
        match value {
            true => PIN13R::SECURE,
            false => PIN13R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN13R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN13R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN14R {
    #[doc = "Pin 14 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 14 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN14R {
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
            PIN14R::SECURE => true,
            PIN14R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN14R {
        match value {
            true => PIN14R::SECURE,
            false => PIN14R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN14R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN14R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN15R {
    #[doc = "Pin 15 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 15 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN15R {
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
            PIN15R::SECURE => true,
            PIN15R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN15R {
        match value {
            true => PIN15R::SECURE,
            false => PIN15R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN15R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN15R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN16R {
    #[doc = "Pin 16 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 16 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN16R {
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
            PIN16R::SECURE => true,
            PIN16R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN16R {
        match value {
            true => PIN16R::SECURE,
            false => PIN16R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN16R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN16R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN17R {
    #[doc = "Pin 17 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 17 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN17R {
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
            PIN17R::SECURE => true,
            PIN17R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN17R {
        match value {
            true => PIN17R::SECURE,
            false => PIN17R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN17R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN17R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN18R {
    #[doc = "Pin 18 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 18 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN18R {
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
            PIN18R::SECURE => true,
            PIN18R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN18R {
        match value {
            true => PIN18R::SECURE,
            false => PIN18R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN18R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN18R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN19R {
    #[doc = "Pin 19 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 19 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN19R {
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
            PIN19R::SECURE => true,
            PIN19R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN19R {
        match value {
            true => PIN19R::SECURE,
            false => PIN19R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN19R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN19R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN20R {
    #[doc = "Pin 20 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 20 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN20R {
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
            PIN20R::SECURE => true,
            PIN20R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN20R {
        match value {
            true => PIN20R::SECURE,
            false => PIN20R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN20R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN20R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN21R {
    #[doc = "Pin 21 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 21 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN21R {
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
            PIN21R::SECURE => true,
            PIN21R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN21R {
        match value {
            true => PIN21R::SECURE,
            false => PIN21R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN21R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN21R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN22R {
    #[doc = "Pin 22 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 22 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN22R {
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
            PIN22R::SECURE => true,
            PIN22R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN22R {
        match value {
            true => PIN22R::SECURE,
            false => PIN22R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN22R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN22R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN23R {
    #[doc = "Pin 23 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 23 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN23R {
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
            PIN23R::SECURE => true,
            PIN23R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN23R {
        match value {
            true => PIN23R::SECURE,
            false => PIN23R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN23R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN23R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN24R {
    #[doc = "Pin 24 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 24 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN24R {
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
            PIN24R::SECURE => true,
            PIN24R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN24R {
        match value {
            true => PIN24R::SECURE,
            false => PIN24R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN24R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN24R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN25R {
    #[doc = "Pin 25 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 25 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN25R {
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
            PIN25R::SECURE => true,
            PIN25R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN25R {
        match value {
            true => PIN25R::SECURE,
            false => PIN25R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN25R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN25R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN26R {
    #[doc = "Pin 26 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 26 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN26R {
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
            PIN26R::SECURE => true,
            PIN26R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN26R {
        match value {
            true => PIN26R::SECURE,
            false => PIN26R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN26R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN26R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN27R {
    #[doc = "Pin 27 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 27 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN27R {
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
            PIN27R::SECURE => true,
            PIN27R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN27R {
        match value {
            true => PIN27R::SECURE,
            false => PIN27R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN27R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN27R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN28R {
    #[doc = "Pin 28 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 28 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN28R {
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
            PIN28R::SECURE => true,
            PIN28R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN28R {
        match value {
            true => PIN28R::SECURE,
            false => PIN28R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN28R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN28R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN29R {
    #[doc = "Pin 29 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 29 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN29R {
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
            PIN29R::SECURE => true,
            PIN29R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN29R {
        match value {
            true => PIN29R::SECURE,
            false => PIN29R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN29R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN29R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN30R {
    #[doc = "Pin 30 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 30 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN30R {
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
            PIN30R::SECURE => true,
            PIN30R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN30R {
        match value {
            true => PIN30R::SECURE,
            false => PIN30R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN30R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN30R::NONSECURE
    }
}
#[doc = "Possible values of the field `PIN31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN31R {
    #[doc = "Pin 31 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 31 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN31R {
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
            PIN31R::SECURE => true,
            PIN31R::NONSECURE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN31R {
        match value {
            true => PIN31R::SECURE,
            false => PIN31R::NONSECURE,
        }
    }
    #[doc = "Checks if the value of the field is `SECURE`"]
    #[inline]
    pub fn is_secure(&self) -> bool {
        *self == PIN31R::SECURE
    }
    #[doc = "Checks if the value of the field is `NONSECURE`"]
    #[inline]
    pub fn is_non_secure(&self) -> bool {
        *self == PIN31R::NONSECURE
    }
}
#[doc = "Values that can be written to the field `PIN0`"]
pub enum PIN0W {
    #[doc = "Pin 0 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 0 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN0W::SECURE => true,
            PIN0W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 0 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN0W::SECURE)
    }
    #[doc = "Pin 0 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN0W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN1`"]
pub enum PIN1W {
    #[doc = "Pin 1 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 1 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN1W::SECURE => true,
            PIN1W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 1 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN1W::SECURE)
    }
    #[doc = "Pin 1 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN1W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN2`"]
pub enum PIN2W {
    #[doc = "Pin 2 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 2 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN2W::SECURE => true,
            PIN2W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 2 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN2W::SECURE)
    }
    #[doc = "Pin 2 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN2W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN3`"]
pub enum PIN3W {
    #[doc = "Pin 3 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 3 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN3W::SECURE => true,
            PIN3W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 3 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN3W::SECURE)
    }
    #[doc = "Pin 3 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN3W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN4`"]
pub enum PIN4W {
    #[doc = "Pin 4 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 4 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN4W::SECURE => true,
            PIN4W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN4W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 4 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN4W::SECURE)
    }
    #[doc = "Pin 4 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN4W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN5`"]
pub enum PIN5W {
    #[doc = "Pin 5 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 5 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN5W::SECURE => true,
            PIN5W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN5W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 5 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN5W::SECURE)
    }
    #[doc = "Pin 5 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN5W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN6`"]
pub enum PIN6W {
    #[doc = "Pin 6 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 6 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN6W::SECURE => true,
            PIN6W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN6W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 6 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN6W::SECURE)
    }
    #[doc = "Pin 6 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN6W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN7`"]
pub enum PIN7W {
    #[doc = "Pin 7 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 7 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN7W::SECURE => true,
            PIN7W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN7W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 7 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN7W::SECURE)
    }
    #[doc = "Pin 7 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN7W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN8`"]
pub enum PIN8W {
    #[doc = "Pin 8 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 8 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN8W::SECURE => true,
            PIN8W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN8W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 8 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN8W::SECURE)
    }
    #[doc = "Pin 8 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN8W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN9`"]
pub enum PIN9W {
    #[doc = "Pin 9 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 9 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN9W::SECURE => true,
            PIN9W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN9W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 9 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN9W::SECURE)
    }
    #[doc = "Pin 9 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN9W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN10`"]
pub enum PIN10W {
    #[doc = "Pin 10 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 10 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN10W::SECURE => true,
            PIN10W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN10W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 10 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN10W::SECURE)
    }
    #[doc = "Pin 10 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN10W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN11`"]
pub enum PIN11W {
    #[doc = "Pin 11 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 11 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN11W::SECURE => true,
            PIN11W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN11W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 11 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN11W::SECURE)
    }
    #[doc = "Pin 11 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN11W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN12`"]
pub enum PIN12W {
    #[doc = "Pin 12 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 12 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN12W::SECURE => true,
            PIN12W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN12W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 12 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN12W::SECURE)
    }
    #[doc = "Pin 12 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN12W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN13`"]
pub enum PIN13W {
    #[doc = "Pin 13 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 13 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN13W::SECURE => true,
            PIN13W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN13W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 13 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN13W::SECURE)
    }
    #[doc = "Pin 13 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN13W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN14`"]
pub enum PIN14W {
    #[doc = "Pin 14 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 14 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN14W::SECURE => true,
            PIN14W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN14W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 14 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN14W::SECURE)
    }
    #[doc = "Pin 14 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN14W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN15`"]
pub enum PIN15W {
    #[doc = "Pin 15 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 15 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN15W::SECURE => true,
            PIN15W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN15W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 15 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN15W::SECURE)
    }
    #[doc = "Pin 15 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN15W::NONSECURE)
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
#[doc = "Values that can be written to the field `PIN16`"]
pub enum PIN16W {
    #[doc = "Pin 16 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 16 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN16W::SECURE => true,
            PIN16W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN16W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 16 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN16W::SECURE)
    }
    #[doc = "Pin 16 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN16W::NONSECURE)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN17`"]
pub enum PIN17W {
    #[doc = "Pin 17 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 17 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN17W::SECURE => true,
            PIN17W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN17W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 17 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN17W::SECURE)
    }
    #[doc = "Pin 17 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN17W::NONSECURE)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN18`"]
pub enum PIN18W {
    #[doc = "Pin 18 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 18 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN18W::SECURE => true,
            PIN18W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN18W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 18 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN18W::SECURE)
    }
    #[doc = "Pin 18 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN18W::NONSECURE)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN19`"]
pub enum PIN19W {
    #[doc = "Pin 19 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 19 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN19W::SECURE => true,
            PIN19W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN19W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 19 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN19W::SECURE)
    }
    #[doc = "Pin 19 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN19W::NONSECURE)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN20`"]
pub enum PIN20W {
    #[doc = "Pin 20 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 20 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN20W::SECURE => true,
            PIN20W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN20W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 20 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN20W::SECURE)
    }
    #[doc = "Pin 20 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN20W::NONSECURE)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN21`"]
pub enum PIN21W {
    #[doc = "Pin 21 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 21 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN21W::SECURE => true,
            PIN21W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN21W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 21 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN21W::SECURE)
    }
    #[doc = "Pin 21 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN21W::NONSECURE)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN22`"]
pub enum PIN22W {
    #[doc = "Pin 22 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 22 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN22W::SECURE => true,
            PIN22W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN22W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 22 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN22W::SECURE)
    }
    #[doc = "Pin 22 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN22W::NONSECURE)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN23`"]
pub enum PIN23W {
    #[doc = "Pin 23 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 23 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN23W::SECURE => true,
            PIN23W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN23W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 23 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN23W::SECURE)
    }
    #[doc = "Pin 23 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN23W::NONSECURE)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN24`"]
pub enum PIN24W {
    #[doc = "Pin 24 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 24 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN24W::SECURE => true,
            PIN24W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN24W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 24 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN24W::SECURE)
    }
    #[doc = "Pin 24 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN24W::NONSECURE)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN25`"]
pub enum PIN25W {
    #[doc = "Pin 25 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 25 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN25W::SECURE => true,
            PIN25W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN25W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 25 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN25W::SECURE)
    }
    #[doc = "Pin 25 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN25W::NONSECURE)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN26`"]
pub enum PIN26W {
    #[doc = "Pin 26 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 26 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN26W::SECURE => true,
            PIN26W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN26W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 26 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN26W::SECURE)
    }
    #[doc = "Pin 26 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN26W::NONSECURE)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN27`"]
pub enum PIN27W {
    #[doc = "Pin 27 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 27 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN27W::SECURE => true,
            PIN27W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN27W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 27 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN27W::SECURE)
    }
    #[doc = "Pin 27 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN27W::NONSECURE)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN28`"]
pub enum PIN28W {
    #[doc = "Pin 28 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 28 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN28W::SECURE => true,
            PIN28W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN28W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 28 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN28W::SECURE)
    }
    #[doc = "Pin 28 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN28W::NONSECURE)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN29`"]
pub enum PIN29W {
    #[doc = "Pin 29 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 29 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN29W::SECURE => true,
            PIN29W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN29W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 29 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN29W::SECURE)
    }
    #[doc = "Pin 29 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN29W::NONSECURE)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN30`"]
pub enum PIN30W {
    #[doc = "Pin 30 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 30 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN30W::SECURE => true,
            PIN30W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN30W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 30 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN30W::SECURE)
    }
    #[doc = "Pin 30 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN30W::NONSECURE)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN31`"]
pub enum PIN31W {
    #[doc = "Pin 31 has its secure attribute set"]
    SECURE,
    #[doc = "Pin 31 has its non-secure attribute set"]
    NONSECURE,
}
impl PIN31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN31W::SECURE => true,
            PIN31W::NONSECURE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN31W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin 31 has its secure attribute set"]
    #[inline]
    pub fn secure(self) -> &'a mut W {
        self.variant(PIN31W::SECURE)
    }
    #[doc = "Pin 31 has its non-secure attribute set"]
    #[inline]
    pub fn non_secure(self) -> &'a mut W {
        self.variant(PIN31W::NONSECURE)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Select secure attribute attribute for PIN 0."]
    #[inline]
    pub fn pin0(&self) -> PIN0R {
        PIN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Select secure attribute attribute for PIN 1."]
    #[inline]
    pub fn pin1(&self) -> PIN1R {
        PIN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Select secure attribute attribute for PIN 2."]
    #[inline]
    pub fn pin2(&self) -> PIN2R {
        PIN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Select secure attribute attribute for PIN 3."]
    #[inline]
    pub fn pin3(&self) -> PIN3R {
        PIN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Select secure attribute attribute for PIN 4."]
    #[inline]
    pub fn pin4(&self) -> PIN4R {
        PIN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Select secure attribute attribute for PIN 5."]
    #[inline]
    pub fn pin5(&self) -> PIN5R {
        PIN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Select secure attribute attribute for PIN 6."]
    #[inline]
    pub fn pin6(&self) -> PIN6R {
        PIN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Select secure attribute attribute for PIN 7."]
    #[inline]
    pub fn pin7(&self) -> PIN7R {
        PIN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Select secure attribute attribute for PIN 8."]
    #[inline]
    pub fn pin8(&self) -> PIN8R {
        PIN8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Select secure attribute attribute for PIN 9."]
    #[inline]
    pub fn pin9(&self) -> PIN9R {
        PIN9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Select secure attribute attribute for PIN 10."]
    #[inline]
    pub fn pin10(&self) -> PIN10R {
        PIN10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Select secure attribute attribute for PIN 11."]
    #[inline]
    pub fn pin11(&self) -> PIN11R {
        PIN11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Select secure attribute attribute for PIN 12."]
    #[inline]
    pub fn pin12(&self) -> PIN12R {
        PIN12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Select secure attribute attribute for PIN 13."]
    #[inline]
    pub fn pin13(&self) -> PIN13R {
        PIN13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Select secure attribute attribute for PIN 14."]
    #[inline]
    pub fn pin14(&self) -> PIN14R {
        PIN14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Select secure attribute attribute for PIN 15."]
    #[inline]
    pub fn pin15(&self) -> PIN15R {
        PIN15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Select secure attribute attribute for PIN 16."]
    #[inline]
    pub fn pin16(&self) -> PIN16R {
        PIN16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Select secure attribute attribute for PIN 17."]
    #[inline]
    pub fn pin17(&self) -> PIN17R {
        PIN17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Select secure attribute attribute for PIN 18."]
    #[inline]
    pub fn pin18(&self) -> PIN18R {
        PIN18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Select secure attribute attribute for PIN 19."]
    #[inline]
    pub fn pin19(&self) -> PIN19R {
        PIN19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Select secure attribute attribute for PIN 20."]
    #[inline]
    pub fn pin20(&self) -> PIN20R {
        PIN20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Select secure attribute attribute for PIN 21."]
    #[inline]
    pub fn pin21(&self) -> PIN21R {
        PIN21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Select secure attribute attribute for PIN 22."]
    #[inline]
    pub fn pin22(&self) -> PIN22R {
        PIN22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Select secure attribute attribute for PIN 23."]
    #[inline]
    pub fn pin23(&self) -> PIN23R {
        PIN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Select secure attribute attribute for PIN 24."]
    #[inline]
    pub fn pin24(&self) -> PIN24R {
        PIN24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Select secure attribute attribute for PIN 25."]
    #[inline]
    pub fn pin25(&self) -> PIN25R {
        PIN25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Select secure attribute attribute for PIN 26."]
    #[inline]
    pub fn pin26(&self) -> PIN26R {
        PIN26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Select secure attribute attribute for PIN 27."]
    #[inline]
    pub fn pin27(&self) -> PIN27R {
        PIN27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Select secure attribute attribute for PIN 28."]
    #[inline]
    pub fn pin28(&self) -> PIN28R {
        PIN28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Select secure attribute attribute for PIN 29."]
    #[inline]
    pub fn pin29(&self) -> PIN29R {
        PIN29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Select secure attribute attribute for PIN 30."]
    #[inline]
    pub fn pin30(&self) -> PIN30R {
        PIN30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Select secure attribute attribute for PIN 31."]
    #[inline]
    pub fn pin31(&self) -> PIN31R {
        PIN31R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Select secure attribute attribute for PIN 0."]
    #[inline]
    pub fn pin0(&mut self) -> _PIN0W {
        _PIN0W { w: self }
    }
    #[doc = "Bit 1 - Select secure attribute attribute for PIN 1."]
    #[inline]
    pub fn pin1(&mut self) -> _PIN1W {
        _PIN1W { w: self }
    }
    #[doc = "Bit 2 - Select secure attribute attribute for PIN 2."]
    #[inline]
    pub fn pin2(&mut self) -> _PIN2W {
        _PIN2W { w: self }
    }
    #[doc = "Bit 3 - Select secure attribute attribute for PIN 3."]
    #[inline]
    pub fn pin3(&mut self) -> _PIN3W {
        _PIN3W { w: self }
    }
    #[doc = "Bit 4 - Select secure attribute attribute for PIN 4."]
    #[inline]
    pub fn pin4(&mut self) -> _PIN4W {
        _PIN4W { w: self }
    }
    #[doc = "Bit 5 - Select secure attribute attribute for PIN 5."]
    #[inline]
    pub fn pin5(&mut self) -> _PIN5W {
        _PIN5W { w: self }
    }
    #[doc = "Bit 6 - Select secure attribute attribute for PIN 6."]
    #[inline]
    pub fn pin6(&mut self) -> _PIN6W {
        _PIN6W { w: self }
    }
    #[doc = "Bit 7 - Select secure attribute attribute for PIN 7."]
    #[inline]
    pub fn pin7(&mut self) -> _PIN7W {
        _PIN7W { w: self }
    }
    #[doc = "Bit 8 - Select secure attribute attribute for PIN 8."]
    #[inline]
    pub fn pin8(&mut self) -> _PIN8W {
        _PIN8W { w: self }
    }
    #[doc = "Bit 9 - Select secure attribute attribute for PIN 9."]
    #[inline]
    pub fn pin9(&mut self) -> _PIN9W {
        _PIN9W { w: self }
    }
    #[doc = "Bit 10 - Select secure attribute attribute for PIN 10."]
    #[inline]
    pub fn pin10(&mut self) -> _PIN10W {
        _PIN10W { w: self }
    }
    #[doc = "Bit 11 - Select secure attribute attribute for PIN 11."]
    #[inline]
    pub fn pin11(&mut self) -> _PIN11W {
        _PIN11W { w: self }
    }
    #[doc = "Bit 12 - Select secure attribute attribute for PIN 12."]
    #[inline]
    pub fn pin12(&mut self) -> _PIN12W {
        _PIN12W { w: self }
    }
    #[doc = "Bit 13 - Select secure attribute attribute for PIN 13."]
    #[inline]
    pub fn pin13(&mut self) -> _PIN13W {
        _PIN13W { w: self }
    }
    #[doc = "Bit 14 - Select secure attribute attribute for PIN 14."]
    #[inline]
    pub fn pin14(&mut self) -> _PIN14W {
        _PIN14W { w: self }
    }
    #[doc = "Bit 15 - Select secure attribute attribute for PIN 15."]
    #[inline]
    pub fn pin15(&mut self) -> _PIN15W {
        _PIN15W { w: self }
    }
    #[doc = "Bit 16 - Select secure attribute attribute for PIN 16."]
    #[inline]
    pub fn pin16(&mut self) -> _PIN16W {
        _PIN16W { w: self }
    }
    #[doc = "Bit 17 - Select secure attribute attribute for PIN 17."]
    #[inline]
    pub fn pin17(&mut self) -> _PIN17W {
        _PIN17W { w: self }
    }
    #[doc = "Bit 18 - Select secure attribute attribute for PIN 18."]
    #[inline]
    pub fn pin18(&mut self) -> _PIN18W {
        _PIN18W { w: self }
    }
    #[doc = "Bit 19 - Select secure attribute attribute for PIN 19."]
    #[inline]
    pub fn pin19(&mut self) -> _PIN19W {
        _PIN19W { w: self }
    }
    #[doc = "Bit 20 - Select secure attribute attribute for PIN 20."]
    #[inline]
    pub fn pin20(&mut self) -> _PIN20W {
        _PIN20W { w: self }
    }
    #[doc = "Bit 21 - Select secure attribute attribute for PIN 21."]
    #[inline]
    pub fn pin21(&mut self) -> _PIN21W {
        _PIN21W { w: self }
    }
    #[doc = "Bit 22 - Select secure attribute attribute for PIN 22."]
    #[inline]
    pub fn pin22(&mut self) -> _PIN22W {
        _PIN22W { w: self }
    }
    #[doc = "Bit 23 - Select secure attribute attribute for PIN 23."]
    #[inline]
    pub fn pin23(&mut self) -> _PIN23W {
        _PIN23W { w: self }
    }
    #[doc = "Bit 24 - Select secure attribute attribute for PIN 24."]
    #[inline]
    pub fn pin24(&mut self) -> _PIN24W {
        _PIN24W { w: self }
    }
    #[doc = "Bit 25 - Select secure attribute attribute for PIN 25."]
    #[inline]
    pub fn pin25(&mut self) -> _PIN25W {
        _PIN25W { w: self }
    }
    #[doc = "Bit 26 - Select secure attribute attribute for PIN 26."]
    #[inline]
    pub fn pin26(&mut self) -> _PIN26W {
        _PIN26W { w: self }
    }
    #[doc = "Bit 27 - Select secure attribute attribute for PIN 27."]
    #[inline]
    pub fn pin27(&mut self) -> _PIN27W {
        _PIN27W { w: self }
    }
    #[doc = "Bit 28 - Select secure attribute attribute for PIN 28."]
    #[inline]
    pub fn pin28(&mut self) -> _PIN28W {
        _PIN28W { w: self }
    }
    #[doc = "Bit 29 - Select secure attribute attribute for PIN 29."]
    #[inline]
    pub fn pin29(&mut self) -> _PIN29W {
        _PIN29W { w: self }
    }
    #[doc = "Bit 30 - Select secure attribute attribute for PIN 30."]
    #[inline]
    pub fn pin30(&mut self) -> _PIN30W {
        _PIN30W { w: self }
    }
    #[doc = "Bit 31 - Select secure attribute attribute for PIN 31."]
    #[inline]
    pub fn pin31(&mut self) -> _PIN31W {
        _PIN31W { w: self }
    }
}

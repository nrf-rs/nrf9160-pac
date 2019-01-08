#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHENSET {
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
#[doc = "Possible values of the field `CH0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH0R {
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
            CH0R::DISABLED => false,
            CH0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0R {
        match value {
            false => CH0R::DISABLED,
            true => CH0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH0R::ENABLED
    }
}
#[doc = "Possible values of the field `CH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH1R {
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
            CH1R::DISABLED => false,
            CH1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1R {
        match value {
            false => CH1R::DISABLED,
            true => CH1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH1R::ENABLED
    }
}
#[doc = "Possible values of the field `CH2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH2R {
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
            CH2R::DISABLED => false,
            CH2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2R {
        match value {
            false => CH2R::DISABLED,
            true => CH2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH2R::ENABLED
    }
}
#[doc = "Possible values of the field `CH3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH3R {
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
            CH3R::DISABLED => false,
            CH3R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3R {
        match value {
            false => CH3R::DISABLED,
            true => CH3R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH3R::ENABLED
    }
}
#[doc = "Possible values of the field `CH4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH4R {
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
            CH4R::DISABLED => false,
            CH4R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4R {
        match value {
            false => CH4R::DISABLED,
            true => CH4R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH4R::ENABLED
    }
}
#[doc = "Possible values of the field `CH5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH5R {
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
            CH5R::DISABLED => false,
            CH5R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5R {
        match value {
            false => CH5R::DISABLED,
            true => CH5R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH5R::ENABLED
    }
}
#[doc = "Possible values of the field `CH6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH6R {
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
            CH6R::DISABLED => false,
            CH6R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6R {
        match value {
            false => CH6R::DISABLED,
            true => CH6R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH6R::ENABLED
    }
}
#[doc = "Possible values of the field `CH7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH7R {
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
            CH7R::DISABLED => false,
            CH7R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7R {
        match value {
            false => CH7R::DISABLED,
            true => CH7R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH7R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH7R::ENABLED
    }
}
#[doc = "Possible values of the field `CH8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH8R {
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
            CH8R::DISABLED => false,
            CH8R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH8R {
        match value {
            false => CH8R::DISABLED,
            true => CH8R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH8R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH8R::ENABLED
    }
}
#[doc = "Possible values of the field `CH9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH9R {
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
            CH9R::DISABLED => false,
            CH9R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH9R {
        match value {
            false => CH9R::DISABLED,
            true => CH9R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH9R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH9R::ENABLED
    }
}
#[doc = "Possible values of the field `CH10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH10R {
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
            CH10R::DISABLED => false,
            CH10R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH10R {
        match value {
            false => CH10R::DISABLED,
            true => CH10R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH10R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH10R::ENABLED
    }
}
#[doc = "Possible values of the field `CH11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH11R {
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
            CH11R::DISABLED => false,
            CH11R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH11R {
        match value {
            false => CH11R::DISABLED,
            true => CH11R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH11R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH11R::ENABLED
    }
}
#[doc = "Possible values of the field `CH12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH12R {
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
            CH12R::DISABLED => false,
            CH12R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH12R {
        match value {
            false => CH12R::DISABLED,
            true => CH12R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH12R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH12R::ENABLED
    }
}
#[doc = "Possible values of the field `CH13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH13R {
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
            CH13R::DISABLED => false,
            CH13R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH13R {
        match value {
            false => CH13R::DISABLED,
            true => CH13R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH13R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH13R::ENABLED
    }
}
#[doc = "Possible values of the field `CH14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH14R {
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
            CH14R::DISABLED => false,
            CH14R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH14R {
        match value {
            false => CH14R::DISABLED,
            true => CH14R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH14R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH14R::ENABLED
    }
}
#[doc = "Possible values of the field `CH15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15R {
    #[doc = "Read: channel disabled"]
    DISABLED,
    #[doc = "Read: channel enabled"]
    ENABLED,
}
impl CH15R {
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
            CH15R::DISABLED => false,
            CH15R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH15R {
        match value {
            false => CH15R::DISABLED,
            true => CH15R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CH15R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CH15R::ENABLED
    }
}
#[doc = "Values that can be written to the field `CH0`"]
pub enum CH0W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0W<'a> {
    w: &'a mut W,
}
impl<'a> _CH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH0W::SET)
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
#[doc = "Values that can be written to the field `CH1`"]
pub enum CH1W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1W<'a> {
    w: &'a mut W,
}
impl<'a> _CH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH1W::SET)
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
#[doc = "Values that can be written to the field `CH2`"]
pub enum CH2W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2W<'a> {
    w: &'a mut W,
}
impl<'a> _CH2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH2W::SET)
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
#[doc = "Values that can be written to the field `CH3`"]
pub enum CH3W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3W<'a> {
    w: &'a mut W,
}
impl<'a> _CH3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH3W::SET)
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
#[doc = "Values that can be written to the field `CH4`"]
pub enum CH4W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4W<'a> {
    w: &'a mut W,
}
impl<'a> _CH4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH4W::SET)
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
#[doc = "Values that can be written to the field `CH5`"]
pub enum CH5W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5W<'a> {
    w: &'a mut W,
}
impl<'a> _CH5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH5W::SET)
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
#[doc = "Values that can be written to the field `CH6`"]
pub enum CH6W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6W<'a> {
    w: &'a mut W,
}
impl<'a> _CH6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH6W::SET)
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
#[doc = "Values that can be written to the field `CH7`"]
pub enum CH7W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7W<'a> {
    w: &'a mut W,
}
impl<'a> _CH7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH7W::SET)
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
#[doc = "Values that can be written to the field `CH8`"]
pub enum CH8W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH8W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH8W<'a> {
    w: &'a mut W,
}
impl<'a> _CH8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH8W::SET)
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
#[doc = "Values that can be written to the field `CH9`"]
pub enum CH9W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH9W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH9W<'a> {
    w: &'a mut W,
}
impl<'a> _CH9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH9W::SET)
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
#[doc = "Values that can be written to the field `CH10`"]
pub enum CH10W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH10W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH10W<'a> {
    w: &'a mut W,
}
impl<'a> _CH10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH10W::SET)
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
#[doc = "Values that can be written to the field `CH11`"]
pub enum CH11W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH11W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH11W<'a> {
    w: &'a mut W,
}
impl<'a> _CH11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH11W::SET)
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
#[doc = "Values that can be written to the field `CH12`"]
pub enum CH12W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH12W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH12W<'a> {
    w: &'a mut W,
}
impl<'a> _CH12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH12W::SET)
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
#[doc = "Values that can be written to the field `CH13`"]
pub enum CH13W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH13W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH13W<'a> {
    w: &'a mut W,
}
impl<'a> _CH13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH13W::SET)
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
#[doc = "Values that can be written to the field `CH14`"]
pub enum CH14W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH14W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH14W<'a> {
    w: &'a mut W,
}
impl<'a> _CH14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH14W::SET)
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
#[doc = "Values that can be written to the field `CH15`"]
pub enum CH15W {
    #[doc = "Write: Enable channel"]
    SET,
}
impl CH15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH15W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH15W<'a> {
    w: &'a mut W,
}
impl<'a> _CH15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: Enable channel"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(CH15W::SET)
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
    #[doc = "Bit 0 - Channel 0 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch0(&self) -> CH0R {
        CH0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel 1 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch1(&self) -> CH1R {
        CH1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel 2 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch2(&self) -> CH2R {
        CH2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel 3 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch3(&self) -> CH3R {
        CH3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel 4 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch4(&self) -> CH4R {
        CH4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel 5 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch5(&self) -> CH5R {
        CH5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel 6 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch6(&self) -> CH6R {
        CH6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel 7 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch7(&self) -> CH7R {
        CH7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Channel 8 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch8(&self) -> CH8R {
        CH8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Channel 9 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch9(&self) -> CH9R {
        CH9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Channel 10 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch10(&self) -> CH10R {
        CH10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Channel 11 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch11(&self) -> CH11R {
        CH11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Channel 12 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch12(&self) -> CH12R {
        CH12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Channel 13 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch13(&self) -> CH13R {
        CH13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Channel 14 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch14(&self) -> CH14R {
        CH14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Channel 15 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch15(&self) -> CH15R {
        CH15R::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Channel 0 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch0(&mut self) -> _CH0W {
        _CH0W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch1(&mut self) -> _CH1W {
        _CH1W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch2(&mut self) -> _CH2W {
        _CH2W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch3(&mut self) -> _CH3W {
        _CH3W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch4(&mut self) -> _CH4W {
        _CH4W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch5(&mut self) -> _CH5W {
        _CH5W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch6(&mut self) -> _CH6W {
        _CH6W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch7(&mut self) -> _CH7W {
        _CH7W { w: self }
    }
    #[doc = "Bit 8 - Channel 8 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch8(&mut self) -> _CH8W {
        _CH8W { w: self }
    }
    #[doc = "Bit 9 - Channel 9 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch9(&mut self) -> _CH9W {
        _CH9W { w: self }
    }
    #[doc = "Bit 10 - Channel 10 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch10(&mut self) -> _CH10W {
        _CH10W { w: self }
    }
    #[doc = "Bit 11 - Channel 11 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch11(&mut self) -> _CH11W {
        _CH11W { w: self }
    }
    #[doc = "Bit 12 - Channel 12 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch12(&mut self) -> _CH12W {
        _CH12W { w: self }
    }
    #[doc = "Bit 13 - Channel 13 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch13(&mut self) -> _CH13W {
        _CH13W { w: self }
    }
    #[doc = "Bit 14 - Channel 14 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch14(&mut self) -> _CH14W {
        _CH14W { w: self }
    }
    #[doc = "Bit 15 - Channel 15 enable set register. Writing '0' has no effect"]
    #[inline]
    pub fn ch15(&mut self) -> _CH15W {
        _CH15W { w: self }
    }
}

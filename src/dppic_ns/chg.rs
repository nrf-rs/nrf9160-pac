#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHG {
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH0R::EXCLUDED => false,
            CH0R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH0R {
        match value {
            false => CH0R::EXCLUDED,
            true => CH0R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH0R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH0R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH1R::EXCLUDED => false,
            CH1R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH1R {
        match value {
            false => CH1R::EXCLUDED,
            true => CH1R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH1R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH1R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH2R::EXCLUDED => false,
            CH2R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH2R {
        match value {
            false => CH2R::EXCLUDED,
            true => CH2R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH2R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH2R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH3R::EXCLUDED => false,
            CH3R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH3R {
        match value {
            false => CH3R::EXCLUDED,
            true => CH3R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH3R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH3R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH4R::EXCLUDED => false,
            CH4R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH4R {
        match value {
            false => CH4R::EXCLUDED,
            true => CH4R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH4R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH4R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH5R::EXCLUDED => false,
            CH5R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH5R {
        match value {
            false => CH5R::EXCLUDED,
            true => CH5R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH5R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH5R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH6R::EXCLUDED => false,
            CH6R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH6R {
        match value {
            false => CH6R::EXCLUDED,
            true => CH6R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH6R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH6R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH7R::EXCLUDED => false,
            CH7R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH7R {
        match value {
            false => CH7R::EXCLUDED,
            true => CH7R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH7R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH7R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH8R::EXCLUDED => false,
            CH8R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH8R {
        match value {
            false => CH8R::EXCLUDED,
            true => CH8R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH8R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH8R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH9R::EXCLUDED => false,
            CH9R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH9R {
        match value {
            false => CH9R::EXCLUDED,
            true => CH9R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH9R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH9R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH10R::EXCLUDED => false,
            CH10R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH10R {
        match value {
            false => CH10R::EXCLUDED,
            true => CH10R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH10R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH10R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH11R::EXCLUDED => false,
            CH11R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH11R {
        match value {
            false => CH11R::EXCLUDED,
            true => CH11R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH11R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH11R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH12R::EXCLUDED => false,
            CH12R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH12R {
        match value {
            false => CH12R::EXCLUDED,
            true => CH12R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH12R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH12R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH13R::EXCLUDED => false,
            CH13R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH13R {
        match value {
            false => CH13R::EXCLUDED,
            true => CH13R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH13R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH13R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH14R::EXCLUDED => false,
            CH14R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH14R {
        match value {
            false => CH14R::EXCLUDED,
            true => CH14R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH14R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH14R::INCLUDED
    }
}
#[doc = "Possible values of the field `CH15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15R {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
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
            CH15R::EXCLUDED => false,
            CH15R::INCLUDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH15R {
        match value {
            false => CH15R::EXCLUDED,
            true => CH15R::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline]
    pub fn is_excluded(&self) -> bool {
        *self == CH15R::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline]
    pub fn is_included(&self) -> bool {
        *self == CH15R::INCLUDED
    }
}
#[doc = "Values that can be written to the field `CH0`"]
pub enum CH0W {
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH0W::EXCLUDED => false,
            CH0W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH0W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH0W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH1W::EXCLUDED => false,
            CH1W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH1W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH1W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH2W::EXCLUDED => false,
            CH2W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH2W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH2W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH3W::EXCLUDED => false,
            CH3W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH3W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH3W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH4W::EXCLUDED => false,
            CH4W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH4W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH4W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH5W::EXCLUDED => false,
            CH5W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH5W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH5W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH6W::EXCLUDED => false,
            CH6W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH6W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH6W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH7W::EXCLUDED => false,
            CH7W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH7W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH7W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH8W::EXCLUDED => false,
            CH8W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH8W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH8W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH9W::EXCLUDED => false,
            CH9W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH9W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH9W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH10W::EXCLUDED => false,
            CH10W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH10W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH10W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH11W::EXCLUDED => false,
            CH11W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH11W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH11W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH12W::EXCLUDED => false,
            CH12W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH12W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH12W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH13W::EXCLUDED => false,
            CH13W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH13W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH13W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH14W::EXCLUDED => false,
            CH14W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH14W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH14W::INCLUDED)
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
    #[doc = "Exclude"]
    EXCLUDED,
    #[doc = "Include"]
    INCLUDED,
}
impl CH15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CH15W::EXCLUDED => false,
            CH15W::INCLUDED => true,
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
    #[doc = "Exclude"]
    #[inline]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH15W::EXCLUDED)
    }
    #[doc = "Include"]
    #[inline]
    pub fn included(self) -> &'a mut W {
        self.variant(CH15W::INCLUDED)
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
    #[doc = "Bit 0 - Include or exclude channel 0"]
    #[inline]
    pub fn ch0(&self) -> CH0R {
        CH0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Include or exclude channel 1"]
    #[inline]
    pub fn ch1(&self) -> CH1R {
        CH1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Include or exclude channel 2"]
    #[inline]
    pub fn ch2(&self) -> CH2R {
        CH2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Include or exclude channel 3"]
    #[inline]
    pub fn ch3(&self) -> CH3R {
        CH3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Include or exclude channel 4"]
    #[inline]
    pub fn ch4(&self) -> CH4R {
        CH4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Include or exclude channel 5"]
    #[inline]
    pub fn ch5(&self) -> CH5R {
        CH5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Include or exclude channel 6"]
    #[inline]
    pub fn ch6(&self) -> CH6R {
        CH6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Include or exclude channel 7"]
    #[inline]
    pub fn ch7(&self) -> CH7R {
        CH7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Include or exclude channel 8"]
    #[inline]
    pub fn ch8(&self) -> CH8R {
        CH8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Include or exclude channel 9"]
    #[inline]
    pub fn ch9(&self) -> CH9R {
        CH9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Include or exclude channel 10"]
    #[inline]
    pub fn ch10(&self) -> CH10R {
        CH10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Include or exclude channel 11"]
    #[inline]
    pub fn ch11(&self) -> CH11R {
        CH11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Include or exclude channel 12"]
    #[inline]
    pub fn ch12(&self) -> CH12R {
        CH12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Include or exclude channel 13"]
    #[inline]
    pub fn ch13(&self) -> CH13R {
        CH13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Include or exclude channel 14"]
    #[inline]
    pub fn ch14(&self) -> CH14R {
        CH14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Include or exclude channel 15"]
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
    #[doc = "Bit 0 - Include or exclude channel 0"]
    #[inline]
    pub fn ch0(&mut self) -> _CH0W {
        _CH0W { w: self }
    }
    #[doc = "Bit 1 - Include or exclude channel 1"]
    #[inline]
    pub fn ch1(&mut self) -> _CH1W {
        _CH1W { w: self }
    }
    #[doc = "Bit 2 - Include or exclude channel 2"]
    #[inline]
    pub fn ch2(&mut self) -> _CH2W {
        _CH2W { w: self }
    }
    #[doc = "Bit 3 - Include or exclude channel 3"]
    #[inline]
    pub fn ch3(&mut self) -> _CH3W {
        _CH3W { w: self }
    }
    #[doc = "Bit 4 - Include or exclude channel 4"]
    #[inline]
    pub fn ch4(&mut self) -> _CH4W {
        _CH4W { w: self }
    }
    #[doc = "Bit 5 - Include or exclude channel 5"]
    #[inline]
    pub fn ch5(&mut self) -> _CH5W {
        _CH5W { w: self }
    }
    #[doc = "Bit 6 - Include or exclude channel 6"]
    #[inline]
    pub fn ch6(&mut self) -> _CH6W {
        _CH6W { w: self }
    }
    #[doc = "Bit 7 - Include or exclude channel 7"]
    #[inline]
    pub fn ch7(&mut self) -> _CH7W {
        _CH7W { w: self }
    }
    #[doc = "Bit 8 - Include or exclude channel 8"]
    #[inline]
    pub fn ch8(&mut self) -> _CH8W {
        _CH8W { w: self }
    }
    #[doc = "Bit 9 - Include or exclude channel 9"]
    #[inline]
    pub fn ch9(&mut self) -> _CH9W {
        _CH9W { w: self }
    }
    #[doc = "Bit 10 - Include or exclude channel 10"]
    #[inline]
    pub fn ch10(&mut self) -> _CH10W {
        _CH10W { w: self }
    }
    #[doc = "Bit 11 - Include or exclude channel 11"]
    #[inline]
    pub fn ch11(&mut self) -> _CH11W {
        _CH11W { w: self }
    }
    #[doc = "Bit 12 - Include or exclude channel 12"]
    #[inline]
    pub fn ch12(&mut self) -> _CH12W {
        _CH12W { w: self }
    }
    #[doc = "Bit 13 - Include or exclude channel 13"]
    #[inline]
    pub fn ch13(&mut self) -> _CH13W {
        _CH13W { w: self }
    }
    #[doc = "Bit 14 - Include or exclude channel 14"]
    #[inline]
    pub fn ch14(&mut self) -> _CH14W {
        _CH14W { w: self }
    }
    #[doc = "Bit 15 - Include or exclude channel 15"]
    #[inline]
    pub fn ch15(&mut self) -> _CH15W {
        _CH15W { w: self }
    }
}

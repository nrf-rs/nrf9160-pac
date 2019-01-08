#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POWER {
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
#[doc = "Possible values of the field `S0POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S0POWERR {
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
            S0POWERR::OFF => false,
            S0POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S0POWERR {
        match value {
            false => S0POWERR::OFF,
            true => S0POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S0POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S0POWERR::ON
    }
}
#[doc = "Possible values of the field `S1POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S1POWERR {
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
            S1POWERR::OFF => false,
            S1POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1POWERR {
        match value {
            false => S1POWERR::OFF,
            true => S1POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S1POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S1POWERR::ON
    }
}
#[doc = "Possible values of the field `S2POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S2POWERR {
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
            S2POWERR::OFF => false,
            S2POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S2POWERR {
        match value {
            false => S2POWERR::OFF,
            true => S2POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S2POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S2POWERR::ON
    }
}
#[doc = "Possible values of the field `S3POWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3POWERR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S3POWERR {
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
            S3POWERR::OFF => false,
            S3POWERR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S3POWERR {
        match value {
            false => S3POWERR::OFF,
            true => S3POWERR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S3POWERR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S3POWERR::ON
    }
}
#[doc = "Possible values of the field `S0RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S0RETENTIONR {
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
            S0RETENTIONR::OFF => false,
            S0RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S0RETENTIONR {
        match value {
            false => S0RETENTIONR::OFF,
            true => S0RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S0RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S0RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S1RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S1RETENTIONR {
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
            S1RETENTIONR::OFF => false,
            S1RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1RETENTIONR {
        match value {
            false => S1RETENTIONR::OFF,
            true => S1RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S1RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S1RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S2RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S2RETENTIONR {
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
            S2RETENTIONR::OFF => false,
            S2RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S2RETENTIONR {
        match value {
            false => S2RETENTIONR::OFF,
            true => S2RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S2RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S2RETENTIONR::ON
    }
}
#[doc = "Possible values of the field `S3RETENTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S3RETENTIONR {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S3RETENTIONR {
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
            S3RETENTIONR::OFF => false,
            S3RETENTIONR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S3RETENTIONR {
        match value {
            false => S3RETENTIONR::OFF,
            true => S3RETENTIONR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == S3RETENTIONR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == S3RETENTIONR::ON
    }
}
#[doc = "Values that can be written to the field `S0POWER`"]
pub enum S0POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S0POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S0POWERW::OFF => false,
            S0POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S0POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S0POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S0POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S0POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S0POWERW::ON)
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
#[doc = "Values that can be written to the field `S1POWER`"]
pub enum S1POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S1POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S1POWERW::OFF => false,
            S1POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S1POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S1POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S1POWERW::ON)
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
#[doc = "Values that can be written to the field `S2POWER`"]
pub enum S2POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S2POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S2POWERW::OFF => false,
            S2POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S2POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S2POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S2POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S2POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S2POWERW::ON)
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
#[doc = "Values that can be written to the field `S3POWER`"]
pub enum S3POWERW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S3POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S3POWERW::OFF => false,
            S3POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S3POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S3POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S3POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S3POWERW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S3POWERW::ON)
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
#[doc = "Values that can be written to the field `S0RETENTION`"]
pub enum S0RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S0RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S0RETENTIONW::OFF => false,
            S0RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S0RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S0RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S0RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S0RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S0RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S1RETENTION`"]
pub enum S1RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S1RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S1RETENTIONW::OFF => false,
            S1RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S1RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S1RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S1RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S2RETENTION`"]
pub enum S2RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S2RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S2RETENTIONW::OFF => false,
            S2RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S2RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S2RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S2RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S2RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S2RETENTIONW::ON)
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
#[doc = "Values that can be written to the field `S3RETENTION`"]
pub enum S3RETENTIONW {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl S3RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S3RETENTIONW::OFF => false,
            S3RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S3RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S3RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S3RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S3RETENTIONW::OFF)
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S3RETENTIONW::ON)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Keep RAM section S0 of RAM n on or off in System ON mode"]
    #[inline]
    pub fn s0power(&self) -> S0POWERR {
        S0POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Keep RAM section S1 of RAM n on or off in System ON mode"]
    #[inline]
    pub fn s1power(&self) -> S1POWERR {
        S1POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Keep RAM section S2 of RAM n on or off in System ON mode"]
    #[inline]
    pub fn s2power(&self) -> S2POWERR {
        S2POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Keep RAM section S3 of RAM n on or off in System ON mode"]
    #[inline]
    pub fn s3power(&self) -> S3POWERR {
        S3POWERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 of RAM n when RAM section is switched off"]
    #[inline]
    pub fn s0retention(&self) -> S0RETENTIONR {
        S0RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 of RAM n when RAM section is switched off"]
    #[inline]
    pub fn s1retention(&self) -> S1RETENTIONR {
        S1RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 of RAM n when RAM section is switched off"]
    #[inline]
    pub fn s2retention(&self) -> S2RETENTIONR {
        S2RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 of RAM n when RAM section is switched off"]
    #[inline]
    pub fn s3retention(&self) -> S3RETENTIONR {
        S3RETENTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
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
    #[doc = "Bit 0 - Keep RAM section S0 of RAM n on or off in System ON mode"]
    #[inline]
    pub fn s0power(&mut self) -> _S0POWERW {
        _S0POWERW { w: self }
    }
    #[doc = "Bit 1 - Keep RAM section S1 of RAM n on or off in System ON mode"]
    #[inline]
    pub fn s1power(&mut self) -> _S1POWERW {
        _S1POWERW { w: self }
    }
    #[doc = "Bit 2 - Keep RAM section S2 of RAM n on or off in System ON mode"]
    #[inline]
    pub fn s2power(&mut self) -> _S2POWERW {
        _S2POWERW { w: self }
    }
    #[doc = "Bit 3 - Keep RAM section S3 of RAM n on or off in System ON mode"]
    #[inline]
    pub fn s3power(&mut self) -> _S3POWERW {
        _S3POWERW { w: self }
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 of RAM n when RAM section is switched off"]
    #[inline]
    pub fn s0retention(&mut self) -> _S0RETENTIONW {
        _S0RETENTIONW { w: self }
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 of RAM n when RAM section is switched off"]
    #[inline]
    pub fn s1retention(&mut self) -> _S1RETENTIONW {
        _S1RETENTIONW { w: self }
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 of RAM n when RAM section is switched off"]
    #[inline]
    pub fn s2retention(&mut self) -> _S2RETENTIONW {
        _S2RETENTIONW { w: self }
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 of RAM n when RAM section is switched off"]
    #[inline]
    pub fn s3retention(&mut self) -> _S3RETENTIONW {
        _S3RETENTIONW { w: self }
    }
}

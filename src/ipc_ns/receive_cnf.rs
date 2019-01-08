#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RECEIVE_CNF {
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
#[doc = "Possible values of the field `CHEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN0R {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN0R {
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
            CHEN0R::DISABLE => false,
            CHEN0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHEN0R {
        match value {
            false => CHEN0R::DISABLE,
            true => CHEN0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CHEN0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CHEN0R::ENABLE
    }
}
#[doc = "Possible values of the field `CHEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN1R {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN1R {
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
            CHEN1R::DISABLE => false,
            CHEN1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHEN1R {
        match value {
            false => CHEN1R::DISABLE,
            true => CHEN1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CHEN1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CHEN1R::ENABLE
    }
}
#[doc = "Possible values of the field `CHEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN2R {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN2R {
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
            CHEN2R::DISABLE => false,
            CHEN2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHEN2R {
        match value {
            false => CHEN2R::DISABLE,
            true => CHEN2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CHEN2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CHEN2R::ENABLE
    }
}
#[doc = "Possible values of the field `CHEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN3R {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN3R {
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
            CHEN3R::DISABLE => false,
            CHEN3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHEN3R {
        match value {
            false => CHEN3R::DISABLE,
            true => CHEN3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CHEN3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CHEN3R::ENABLE
    }
}
#[doc = "Possible values of the field `CHEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN4R {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN4R {
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
            CHEN4R::DISABLE => false,
            CHEN4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHEN4R {
        match value {
            false => CHEN4R::DISABLE,
            true => CHEN4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CHEN4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CHEN4R::ENABLE
    }
}
#[doc = "Possible values of the field `CHEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN5R {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN5R {
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
            CHEN5R::DISABLE => false,
            CHEN5R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHEN5R {
        match value {
            false => CHEN5R::DISABLE,
            true => CHEN5R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CHEN5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CHEN5R::ENABLE
    }
}
#[doc = "Possible values of the field `CHEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN6R {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN6R {
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
            CHEN6R::DISABLE => false,
            CHEN6R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHEN6R {
        match value {
            false => CHEN6R::DISABLE,
            true => CHEN6R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CHEN6R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CHEN6R::ENABLE
    }
}
#[doc = "Possible values of the field `CHEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN7R {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN7R {
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
            CHEN7R::DISABLE => false,
            CHEN7R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHEN7R {
        match value {
            false => CHEN7R::DISABLE,
            true => CHEN7R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CHEN7R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CHEN7R::ENABLE
    }
}
#[doc = "Values that can be written to the field `CHEN0`"]
pub enum CHEN0W {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHEN0W::DISABLE => false,
            CHEN0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _CHEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable events."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN0W::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN0W::ENABLE)
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
#[doc = "Values that can be written to the field `CHEN1`"]
pub enum CHEN1W {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHEN1W::DISABLE => false,
            CHEN1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _CHEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable events."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN1W::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN1W::ENABLE)
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
#[doc = "Values that can be written to the field `CHEN2`"]
pub enum CHEN2W {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHEN2W::DISABLE => false,
            CHEN2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _CHEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable events."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN2W::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN2W::ENABLE)
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
#[doc = "Values that can be written to the field `CHEN3`"]
pub enum CHEN3W {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHEN3W::DISABLE => false,
            CHEN3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _CHEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable events."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN3W::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN3W::ENABLE)
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
#[doc = "Values that can be written to the field `CHEN4`"]
pub enum CHEN4W {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHEN4W::DISABLE => false,
            CHEN4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _CHEN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHEN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable events."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN4W::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN4W::ENABLE)
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
#[doc = "Values that can be written to the field `CHEN5`"]
pub enum CHEN5W {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHEN5W::DISABLE => false,
            CHEN5W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _CHEN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHEN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable events."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN5W::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN5W::ENABLE)
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
#[doc = "Values that can be written to the field `CHEN6`"]
pub enum CHEN6W {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHEN6W::DISABLE => false,
            CHEN6W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _CHEN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHEN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable events."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN6W::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN6W::ENABLE)
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
#[doc = "Values that can be written to the field `CHEN7`"]
pub enum CHEN7W {
    #[doc = "Disable events."]
    DISABLE,
    #[doc = "Enable events."]
    ENABLE,
}
impl CHEN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHEN7W::DISABLE => false,
            CHEN7W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _CHEN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHEN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable events."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN7W::DISABLE)
    }
    #[doc = "Enable events."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN7W::ENABLE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable subscription to channel 0."]
    #[inline]
    pub fn chen0(&self) -> CHEN0R {
        CHEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable subscription to channel 1."]
    #[inline]
    pub fn chen1(&self) -> CHEN1R {
        CHEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Enable subscription to channel 2."]
    #[inline]
    pub fn chen2(&self) -> CHEN2R {
        CHEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable subscription to channel 3."]
    #[inline]
    pub fn chen3(&self) -> CHEN3R {
        CHEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable subscription to channel 4."]
    #[inline]
    pub fn chen4(&self) -> CHEN4R {
        CHEN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable subscription to channel 5."]
    #[inline]
    pub fn chen5(&self) -> CHEN5R {
        CHEN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable subscription to channel 6."]
    #[inline]
    pub fn chen6(&self) -> CHEN6R {
        CHEN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable subscription to channel 7."]
    #[inline]
    pub fn chen7(&self) -> CHEN7R {
        CHEN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Enable subscription to channel 0."]
    #[inline]
    pub fn chen0(&mut self) -> _CHEN0W {
        _CHEN0W { w: self }
    }
    #[doc = "Bit 1 - Enable subscription to channel 1."]
    #[inline]
    pub fn chen1(&mut self) -> _CHEN1W {
        _CHEN1W { w: self }
    }
    #[doc = "Bit 2 - Enable subscription to channel 2."]
    #[inline]
    pub fn chen2(&mut self) -> _CHEN2W {
        _CHEN2W { w: self }
    }
    #[doc = "Bit 3 - Enable subscription to channel 3."]
    #[inline]
    pub fn chen3(&mut self) -> _CHEN3W {
        _CHEN3W { w: self }
    }
    #[doc = "Bit 4 - Enable subscription to channel 4."]
    #[inline]
    pub fn chen4(&mut self) -> _CHEN4W {
        _CHEN4W { w: self }
    }
    #[doc = "Bit 5 - Enable subscription to channel 5."]
    #[inline]
    pub fn chen5(&mut self) -> _CHEN5W {
        _CHEN5W { w: self }
    }
    #[doc = "Bit 6 - Enable subscription to channel 6."]
    #[inline]
    pub fn chen6(&mut self) -> _CHEN6W {
        _CHEN6W { w: self }
    }
    #[doc = "Bit 7 - Enable subscription to channel 7."]
    #[inline]
    pub fn chen7(&mut self) -> _CHEN7W {
        _CHEN7W { w: self }
    }
}

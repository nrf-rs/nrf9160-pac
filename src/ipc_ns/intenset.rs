#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENSET {
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
#[doc = "Possible values of the field `RECEIVE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE0R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl RECEIVE0R {
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
            RECEIVE0R::DISABLED => false,
            RECEIVE0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE0R {
        match value {
            false => RECEIVE0R::DISABLED,
            true => RECEIVE0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE0R::ENABLED
    }
}
#[doc = "Possible values of the field `RECEIVE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE1R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl RECEIVE1R {
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
            RECEIVE1R::DISABLED => false,
            RECEIVE1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE1R {
        match value {
            false => RECEIVE1R::DISABLED,
            true => RECEIVE1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE1R::ENABLED
    }
}
#[doc = "Possible values of the field `RECEIVE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE2R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl RECEIVE2R {
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
            RECEIVE2R::DISABLED => false,
            RECEIVE2R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE2R {
        match value {
            false => RECEIVE2R::DISABLED,
            true => RECEIVE2R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE2R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE2R::ENABLED
    }
}
#[doc = "Possible values of the field `RECEIVE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE3R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl RECEIVE3R {
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
            RECEIVE3R::DISABLED => false,
            RECEIVE3R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE3R {
        match value {
            false => RECEIVE3R::DISABLED,
            true => RECEIVE3R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE3R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE3R::ENABLED
    }
}
#[doc = "Possible values of the field `RECEIVE4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE4R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl RECEIVE4R {
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
            RECEIVE4R::DISABLED => false,
            RECEIVE4R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE4R {
        match value {
            false => RECEIVE4R::DISABLED,
            true => RECEIVE4R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE4R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE4R::ENABLED
    }
}
#[doc = "Possible values of the field `RECEIVE5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE5R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl RECEIVE5R {
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
            RECEIVE5R::DISABLED => false,
            RECEIVE5R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE5R {
        match value {
            false => RECEIVE5R::DISABLED,
            true => RECEIVE5R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE5R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE5R::ENABLED
    }
}
#[doc = "Possible values of the field `RECEIVE6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE6R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl RECEIVE6R {
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
            RECEIVE6R::DISABLED => false,
            RECEIVE6R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE6R {
        match value {
            false => RECEIVE6R::DISABLED,
            true => RECEIVE6R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE6R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE6R::ENABLED
    }
}
#[doc = "Possible values of the field `RECEIVE7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE7R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl RECEIVE7R {
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
            RECEIVE7R::DISABLED => false,
            RECEIVE7R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE7R {
        match value {
            false => RECEIVE7R::DISABLED,
            true => RECEIVE7R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RECEIVE7R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RECEIVE7R::ENABLED
    }
}
#[doc = "Values that can be written to the field `RECEIVE0`"]
pub enum RECEIVE0W {
    #[doc = "Enable"]
    SET,
}
impl RECEIVE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RECEIVE0W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RECEIVE0W<'a> {
    w: &'a mut W,
}
impl<'a> _RECEIVE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RECEIVE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RECEIVE0W::SET)
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
#[doc = "Values that can be written to the field `RECEIVE1`"]
pub enum RECEIVE1W {
    #[doc = "Enable"]
    SET,
}
impl RECEIVE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RECEIVE1W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RECEIVE1W<'a> {
    w: &'a mut W,
}
impl<'a> _RECEIVE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RECEIVE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RECEIVE1W::SET)
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
#[doc = "Values that can be written to the field `RECEIVE2`"]
pub enum RECEIVE2W {
    #[doc = "Enable"]
    SET,
}
impl RECEIVE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RECEIVE2W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RECEIVE2W<'a> {
    w: &'a mut W,
}
impl<'a> _RECEIVE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RECEIVE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RECEIVE2W::SET)
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
#[doc = "Values that can be written to the field `RECEIVE3`"]
pub enum RECEIVE3W {
    #[doc = "Enable"]
    SET,
}
impl RECEIVE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RECEIVE3W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RECEIVE3W<'a> {
    w: &'a mut W,
}
impl<'a> _RECEIVE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RECEIVE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RECEIVE3W::SET)
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
#[doc = "Values that can be written to the field `RECEIVE4`"]
pub enum RECEIVE4W {
    #[doc = "Enable"]
    SET,
}
impl RECEIVE4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RECEIVE4W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RECEIVE4W<'a> {
    w: &'a mut W,
}
impl<'a> _RECEIVE4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RECEIVE4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RECEIVE4W::SET)
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
#[doc = "Values that can be written to the field `RECEIVE5`"]
pub enum RECEIVE5W {
    #[doc = "Enable"]
    SET,
}
impl RECEIVE5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RECEIVE5W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RECEIVE5W<'a> {
    w: &'a mut W,
}
impl<'a> _RECEIVE5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RECEIVE5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RECEIVE5W::SET)
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
#[doc = "Values that can be written to the field `RECEIVE6`"]
pub enum RECEIVE6W {
    #[doc = "Enable"]
    SET,
}
impl RECEIVE6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RECEIVE6W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RECEIVE6W<'a> {
    w: &'a mut W,
}
impl<'a> _RECEIVE6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RECEIVE6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RECEIVE6W::SET)
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
#[doc = "Values that can be written to the field `RECEIVE7`"]
pub enum RECEIVE7W {
    #[doc = "Enable"]
    SET,
}
impl RECEIVE7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RECEIVE7W::SET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RECEIVE7W<'a> {
    w: &'a mut W,
}
impl<'a> _RECEIVE7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RECEIVE7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enable"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(RECEIVE7W::SET)
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
    #[doc = "Bit 0 - Write '1' to enable interrupt for event RECEIVE\\[0\\]"]
    #[inline]
    pub fn receive0(&self) -> RECEIVE0R {
        RECEIVE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event RECEIVE\\[1\\]"]
    #[inline]
    pub fn receive1(&self) -> RECEIVE1R {
        RECEIVE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event RECEIVE\\[2\\]"]
    #[inline]
    pub fn receive2(&self) -> RECEIVE2R {
        RECEIVE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event RECEIVE\\[3\\]"]
    #[inline]
    pub fn receive3(&self) -> RECEIVE3R {
        RECEIVE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event RECEIVE\\[4\\]"]
    #[inline]
    pub fn receive4(&self) -> RECEIVE4R {
        RECEIVE4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event RECEIVE\\[5\\]"]
    #[inline]
    pub fn receive5(&self) -> RECEIVE5R {
        RECEIVE5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event RECEIVE\\[6\\]"]
    #[inline]
    pub fn receive6(&self) -> RECEIVE6R {
        RECEIVE6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event RECEIVE\\[7\\]"]
    #[inline]
    pub fn receive7(&self) -> RECEIVE7R {
        RECEIVE7R::_from({
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
    #[doc = "Bit 0 - Write '1' to enable interrupt for event RECEIVE\\[0\\]"]
    #[inline]
    pub fn receive0(&mut self) -> _RECEIVE0W {
        _RECEIVE0W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event RECEIVE\\[1\\]"]
    #[inline]
    pub fn receive1(&mut self) -> _RECEIVE1W {
        _RECEIVE1W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event RECEIVE\\[2\\]"]
    #[inline]
    pub fn receive2(&mut self) -> _RECEIVE2W {
        _RECEIVE2W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event RECEIVE\\[3\\]"]
    #[inline]
    pub fn receive3(&mut self) -> _RECEIVE3W {
        _RECEIVE3W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event RECEIVE\\[4\\]"]
    #[inline]
    pub fn receive4(&mut self) -> _RECEIVE4W {
        _RECEIVE4W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event RECEIVE\\[5\\]"]
    #[inline]
    pub fn receive5(&mut self) -> _RECEIVE5W {
        _RECEIVE5W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event RECEIVE\\[6\\]"]
    #[inline]
    pub fn receive6(&mut self) -> _RECEIVE6W {
        _RECEIVE6W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event RECEIVE\\[7\\]"]
    #[inline]
    pub fn receive7(&mut self) -> _RECEIVE7W {
        _RECEIVE7W { w: self }
    }
}

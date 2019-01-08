#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::INTPEND {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RECEIVE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE0R {
    #[doc = "Read: Not pending"]
    NOTPENDING,
    #[doc = "Read: Pending"]
    PENDING,
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
            RECEIVE0R::NOTPENDING => false,
            RECEIVE0R::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE0R {
        match value {
            false => RECEIVE0R::NOTPENDING,
            true => RECEIVE0R::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE0R::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE0R::PENDING
    }
}
#[doc = "Possible values of the field `RECEIVE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE1R {
    #[doc = "Read: Not pending"]
    NOTPENDING,
    #[doc = "Read: Pending"]
    PENDING,
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
            RECEIVE1R::NOTPENDING => false,
            RECEIVE1R::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE1R {
        match value {
            false => RECEIVE1R::NOTPENDING,
            true => RECEIVE1R::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE1R::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE1R::PENDING
    }
}
#[doc = "Possible values of the field `RECEIVE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE2R {
    #[doc = "Read: Not pending"]
    NOTPENDING,
    #[doc = "Read: Pending"]
    PENDING,
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
            RECEIVE2R::NOTPENDING => false,
            RECEIVE2R::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE2R {
        match value {
            false => RECEIVE2R::NOTPENDING,
            true => RECEIVE2R::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE2R::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE2R::PENDING
    }
}
#[doc = "Possible values of the field `RECEIVE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE3R {
    #[doc = "Read: Not pending"]
    NOTPENDING,
    #[doc = "Read: Pending"]
    PENDING,
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
            RECEIVE3R::NOTPENDING => false,
            RECEIVE3R::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE3R {
        match value {
            false => RECEIVE3R::NOTPENDING,
            true => RECEIVE3R::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE3R::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE3R::PENDING
    }
}
#[doc = "Possible values of the field `RECEIVE4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE4R {
    #[doc = "Read: Not pending"]
    NOTPENDING,
    #[doc = "Read: Pending"]
    PENDING,
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
            RECEIVE4R::NOTPENDING => false,
            RECEIVE4R::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE4R {
        match value {
            false => RECEIVE4R::NOTPENDING,
            true => RECEIVE4R::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE4R::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE4R::PENDING
    }
}
#[doc = "Possible values of the field `RECEIVE5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE5R {
    #[doc = "Read: Not pending"]
    NOTPENDING,
    #[doc = "Read: Pending"]
    PENDING,
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
            RECEIVE5R::NOTPENDING => false,
            RECEIVE5R::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE5R {
        match value {
            false => RECEIVE5R::NOTPENDING,
            true => RECEIVE5R::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE5R::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE5R::PENDING
    }
}
#[doc = "Possible values of the field `RECEIVE6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE6R {
    #[doc = "Read: Not pending"]
    NOTPENDING,
    #[doc = "Read: Pending"]
    PENDING,
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
            RECEIVE6R::NOTPENDING => false,
            RECEIVE6R::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE6R {
        match value {
            false => RECEIVE6R::NOTPENDING,
            true => RECEIVE6R::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE6R::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE6R::PENDING
    }
}
#[doc = "Possible values of the field `RECEIVE7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE7R {
    #[doc = "Read: Not pending"]
    NOTPENDING,
    #[doc = "Read: Pending"]
    PENDING,
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
            RECEIVE7R::NOTPENDING => false,
            RECEIVE7R::PENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RECEIVE7R {
        match value {
            false => RECEIVE7R::NOTPENDING,
            true => RECEIVE7R::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE7R::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE7R::PENDING
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Read pending status of interrupt for event RECEIVE[0]"]
    #[inline]
    pub fn receive0(&self) -> RECEIVE0R {
        RECEIVE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event RECEIVE[1]"]
    #[inline]
    pub fn receive1(&self) -> RECEIVE1R {
        RECEIVE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event RECEIVE[2]"]
    #[inline]
    pub fn receive2(&self) -> RECEIVE2R {
        RECEIVE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Read pending status of interrupt for event RECEIVE[3]"]
    #[inline]
    pub fn receive3(&self) -> RECEIVE3R {
        RECEIVE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Read pending status of interrupt for event RECEIVE[4]"]
    #[inline]
    pub fn receive4(&self) -> RECEIVE4R {
        RECEIVE4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Read pending status of interrupt for event RECEIVE[5]"]
    #[inline]
    pub fn receive5(&self) -> RECEIVE5R {
        RECEIVE5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Read pending status of interrupt for event RECEIVE[6]"]
    #[inline]
    pub fn receive6(&self) -> RECEIVE6R {
        RECEIVE6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Read pending status of interrupt for event RECEIVE[7]"]
    #[inline]
    pub fn receive7(&self) -> RECEIVE7R {
        RECEIVE7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}

#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TASKS_START {
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
}
#[doc = "Values that can be written to the field `TASKS_START`"]
pub enum TASKS_STARTW {
    #[doc = "Trigger task"]
    TRIGGER,
}
impl TASKS_STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TASKS_STARTW::TRIGGER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TASKS_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _TASKS_STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TASKS_STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger task"]
    #[inline]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_STARTW::TRIGGER)
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
    #[doc = "Bit 0 - Start the ADC and prepare the result buffer in RAM"]
    #[inline]
    pub fn tasks_start(&mut self) -> _TASKS_STARTW {
        _TASKS_STARTW { w: self }
    }
}

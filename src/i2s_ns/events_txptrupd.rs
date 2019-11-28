#[doc = "Reader of register EVENTS_TXPTRUPD"]
pub type R = crate::R<u32, super::EVENTS_TXPTRUPD>;
#[doc = "Writer for register EVENTS_TXPTRUPD"]
pub type W = crate::W<u32, super::EVENTS_TXPTRUPD>;
#[doc = "Register EVENTS_TXPTRUPD `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_TXPTRUPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_TXPTRUPD_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_TXPTRUPD_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_TXPTRUPD_A) -> Self {
        match variant {
            EVENTS_TXPTRUPD_A::NOTGENERATED => false,
            EVENTS_TXPTRUPD_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_TXPTRUPD`"]
pub type EVENTS_TXPTRUPD_R = crate::R<bool, EVENTS_TXPTRUPD_A>;
impl EVENTS_TXPTRUPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_TXPTRUPD_A {
        match self.bits {
            false => EVENTS_TXPTRUPD_A::NOTGENERATED,
            true => EVENTS_TXPTRUPD_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_TXPTRUPD_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_TXPTRUPD_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_TXPTRUPD`"]
pub struct EVENTS_TXPTRUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_TXPTRUPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_TXPTRUPD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_TXPTRUPD_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_TXPTRUPD_A::GENERATED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    #[inline(always)]
    pub fn events_txptrupd(&self) -> EVENTS_TXPTRUPD_R {
        EVENTS_TXPTRUPD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    #[inline(always)]
    pub fn events_txptrupd(&mut self) -> EVENTS_TXPTRUPD_W {
        EVENTS_TXPTRUPD_W { w: self }
    }
}

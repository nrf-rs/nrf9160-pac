#[doc = "Reader of register EVENTS_RXPTRUPD"]
pub type R = crate::R<u32, super::EVENTS_RXPTRUPD>;
#[doc = "Writer for register EVENTS_RXPTRUPD"]
pub type W = crate::W<u32, super::EVENTS_RXPTRUPD>;
#[doc = "Register EVENTS_RXPTRUPD `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_RXPTRUPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_RXPTRUPD_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_RXPTRUPD_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_RXPTRUPD_A) -> Self {
        match variant {
            EVENTS_RXPTRUPD_A::NOTGENERATED => false,
            EVENTS_RXPTRUPD_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_RXPTRUPD`"]
pub type EVENTS_RXPTRUPD_R = crate::R<bool, EVENTS_RXPTRUPD_A>;
impl EVENTS_RXPTRUPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_RXPTRUPD_A {
        match self.bits {
            false => EVENTS_RXPTRUPD_A::NOTGENERATED,
            true => EVENTS_RXPTRUPD_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_RXPTRUPD_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_RXPTRUPD_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_RXPTRUPD`"]
pub struct EVENTS_RXPTRUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_RXPTRUPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_RXPTRUPD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_RXPTRUPD_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_RXPTRUPD_A::GENERATED)
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
    #[doc = "Bit 0 - The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
    #[inline(always)]
    pub fn events_rxptrupd(&self) -> EVENTS_RXPTRUPD_R {
        EVENTS_RXPTRUPD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
    #[inline(always)]
    pub fn events_rxptrupd(&mut self) -> EVENTS_RXPTRUPD_W {
        EVENTS_RXPTRUPD_W { w: self }
    }
}

#[doc = "Reader of register SEND_CNF[%s]"]
pub type R = crate::R<u32, super::SEND_CNF>;
#[doc = "Writer for register SEND_CNF[%s]"]
pub type W = crate::W<u32, super::SEND_CNF>;
#[doc = "Register SEND_CNF[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::SEND_CNF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable broadcasting on channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN0_A {
    #[doc = "0: Disable broadcast."]
    DISABLE,
    #[doc = "1: Enable broadcast."]
    ENABLE,
}
impl From<CHEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN0_A) -> Self {
        match variant {
            CHEN0_A::DISABLE => false,
            CHEN0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN0`"]
pub type CHEN0_R = crate::R<bool, CHEN0_A>;
impl CHEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN0_A {
        match self.bits {
            false => CHEN0_A::DISABLE,
            true => CHEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN0_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN0`"]
pub struct CHEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN0_A::DISABLE)
    }
    #[doc = "Enable broadcast."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN0_A::ENABLE)
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
#[doc = "Enable broadcasting on channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN1_A {
    #[doc = "0: Disable broadcast."]
    DISABLE,
    #[doc = "1: Enable broadcast."]
    ENABLE,
}
impl From<CHEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN1_A) -> Self {
        match variant {
            CHEN1_A::DISABLE => false,
            CHEN1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN1`"]
pub type CHEN1_R = crate::R<bool, CHEN1_A>;
impl CHEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN1_A {
        match self.bits {
            false => CHEN1_A::DISABLE,
            true => CHEN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN1_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN1`"]
pub struct CHEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN1_A::DISABLE)
    }
    #[doc = "Enable broadcast."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Enable broadcasting on channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN2_A {
    #[doc = "0: Disable broadcast."]
    DISABLE,
    #[doc = "1: Enable broadcast."]
    ENABLE,
}
impl From<CHEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN2_A) -> Self {
        match variant {
            CHEN2_A::DISABLE => false,
            CHEN2_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN2`"]
pub type CHEN2_R = crate::R<bool, CHEN2_A>;
impl CHEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN2_A {
        match self.bits {
            false => CHEN2_A::DISABLE,
            true => CHEN2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN2_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN2`"]
pub struct CHEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN2_A::DISABLE)
    }
    #[doc = "Enable broadcast."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN2_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Enable broadcasting on channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN3_A {
    #[doc = "0: Disable broadcast."]
    DISABLE,
    #[doc = "1: Enable broadcast."]
    ENABLE,
}
impl From<CHEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN3_A) -> Self {
        match variant {
            CHEN3_A::DISABLE => false,
            CHEN3_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN3`"]
pub type CHEN3_R = crate::R<bool, CHEN3_A>;
impl CHEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN3_A {
        match self.bits {
            false => CHEN3_A::DISABLE,
            true => CHEN3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN3_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN3`"]
pub struct CHEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN3_A::DISABLE)
    }
    #[doc = "Enable broadcast."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN3_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Enable broadcasting on channel 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN4_A {
    #[doc = "0: Disable broadcast."]
    DISABLE,
    #[doc = "1: Enable broadcast."]
    ENABLE,
}
impl From<CHEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN4_A) -> Self {
        match variant {
            CHEN4_A::DISABLE => false,
            CHEN4_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN4`"]
pub type CHEN4_R = crate::R<bool, CHEN4_A>;
impl CHEN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN4_A {
        match self.bits {
            false => CHEN4_A::DISABLE,
            true => CHEN4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN4_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN4`"]
pub struct CHEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN4_A::DISABLE)
    }
    #[doc = "Enable broadcast."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN4_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Enable broadcasting on channel 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN5_A {
    #[doc = "0: Disable broadcast."]
    DISABLE,
    #[doc = "1: Enable broadcast."]
    ENABLE,
}
impl From<CHEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN5_A) -> Self {
        match variant {
            CHEN5_A::DISABLE => false,
            CHEN5_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN5`"]
pub type CHEN5_R = crate::R<bool, CHEN5_A>;
impl CHEN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN5_A {
        match self.bits {
            false => CHEN5_A::DISABLE,
            true => CHEN5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN5_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN5`"]
pub struct CHEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN5_A::DISABLE)
    }
    #[doc = "Enable broadcast."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN5_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Enable broadcasting on channel 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN6_A {
    #[doc = "0: Disable broadcast."]
    DISABLE,
    #[doc = "1: Enable broadcast."]
    ENABLE,
}
impl From<CHEN6_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN6_A) -> Self {
        match variant {
            CHEN6_A::DISABLE => false,
            CHEN6_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN6`"]
pub type CHEN6_R = crate::R<bool, CHEN6_A>;
impl CHEN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN6_A {
        match self.bits {
            false => CHEN6_A::DISABLE,
            true => CHEN6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN6_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN6`"]
pub struct CHEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN6_A::DISABLE)
    }
    #[doc = "Enable broadcast."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN6_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable broadcasting on channel 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN7_A {
    #[doc = "0: Disable broadcast."]
    DISABLE,
    #[doc = "1: Enable broadcast."]
    ENABLE,
}
impl From<CHEN7_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN7_A) -> Self {
        match variant {
            CHEN7_A::DISABLE => false,
            CHEN7_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN7`"]
pub type CHEN7_R = crate::R<bool, CHEN7_A>;
impl CHEN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN7_A {
        match self.bits {
            false => CHEN7_A::DISABLE,
            true => CHEN7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN7_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN7`"]
pub struct CHEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN7_A::DISABLE)
    }
    #[doc = "Enable broadcast."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN7_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable broadcasting on channel 0."]
    #[inline(always)]
    pub fn chen0(&self) -> CHEN0_R {
        CHEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable broadcasting on channel 1."]
    #[inline(always)]
    pub fn chen1(&self) -> CHEN1_R {
        CHEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable broadcasting on channel 2."]
    #[inline(always)]
    pub fn chen2(&self) -> CHEN2_R {
        CHEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable broadcasting on channel 3."]
    #[inline(always)]
    pub fn chen3(&self) -> CHEN3_R {
        CHEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable broadcasting on channel 4."]
    #[inline(always)]
    pub fn chen4(&self) -> CHEN4_R {
        CHEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable broadcasting on channel 5."]
    #[inline(always)]
    pub fn chen5(&self) -> CHEN5_R {
        CHEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable broadcasting on channel 6."]
    #[inline(always)]
    pub fn chen6(&self) -> CHEN6_R {
        CHEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable broadcasting on channel 7."]
    #[inline(always)]
    pub fn chen7(&self) -> CHEN7_R {
        CHEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable broadcasting on channel 0."]
    #[inline(always)]
    pub fn chen0(&mut self) -> CHEN0_W {
        CHEN0_W { w: self }
    }
    #[doc = "Bit 1 - Enable broadcasting on channel 1."]
    #[inline(always)]
    pub fn chen1(&mut self) -> CHEN1_W {
        CHEN1_W { w: self }
    }
    #[doc = "Bit 2 - Enable broadcasting on channel 2."]
    #[inline(always)]
    pub fn chen2(&mut self) -> CHEN2_W {
        CHEN2_W { w: self }
    }
    #[doc = "Bit 3 - Enable broadcasting on channel 3."]
    #[inline(always)]
    pub fn chen3(&mut self) -> CHEN3_W {
        CHEN3_W { w: self }
    }
    #[doc = "Bit 4 - Enable broadcasting on channel 4."]
    #[inline(always)]
    pub fn chen4(&mut self) -> CHEN4_W {
        CHEN4_W { w: self }
    }
    #[doc = "Bit 5 - Enable broadcasting on channel 5."]
    #[inline(always)]
    pub fn chen5(&mut self) -> CHEN5_W {
        CHEN5_W { w: self }
    }
    #[doc = "Bit 6 - Enable broadcasting on channel 6."]
    #[inline(always)]
    pub fn chen6(&mut self) -> CHEN6_W {
        CHEN6_W { w: self }
    }
    #[doc = "Bit 7 - Enable broadcasting on channel 7."]
    #[inline(always)]
    pub fn chen7(&mut self) -> CHEN7_W {
        CHEN7_W { w: self }
    }
}

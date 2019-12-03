#[doc = "Writer for register WRITEUICRNS"]
pub type W = crate::W<u32, super::WRITEUICRNS>;
#[doc = "Register WRITEUICRNS `reset()`'s with value 0"]
impl crate::ResetValue for super::WRITEUICRNS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Allow non-secure code to set APPROTECT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_AW {
    #[doc = "1: Set value"]
    SET,
}
impl From<SET_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_AW) -> Self {
        match variant {
            SET_AW::SET => true,
        }
    }
}
#[doc = "Write proxy for field `SET`"]
pub struct SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Set value"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SET_AW::SET)
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
#[doc = "Key to write in order to validate the write operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_AW {
    #[doc = "184280487: Key value"]
    KEYVALID,
}
impl From<KEY_AW> for u32 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        match variant {
            KEY_AW::KEYVALID => 184280487,
        }
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Key value"]
    #[inline(always)]
    pub fn keyvalid(self) -> &'a mut W {
        self.variant(KEY_AW::KEYVALID)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Allow non-secure code to set APPROTECT"]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W { w: self }
    }
    #[doc = "Bits 4:31 - Key to write in order to validate the write operation"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}

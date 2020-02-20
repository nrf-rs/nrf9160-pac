#[doc = "Reader of register DISABLE"]
pub type R = crate::R<u32, super::DISABLE>;
#[doc = "Writer for register DISABLE"]
pub type W = crate::W<u32, super::DISABLE>;
#[doc = "Register DISABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::DISABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The ERASEALL sequence will be initiated if value of KEY fields are non-zero and KEY fields match on both CPU and debugger side"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The ERASEALL sequence will be initiated if value of KEY fields are non-zero and KEY fields match on both CPU and debugger side"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}

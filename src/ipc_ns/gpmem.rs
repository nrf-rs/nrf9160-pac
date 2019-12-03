#[doc = "Reader of register GPMEM[%s]"]
pub type R = crate::R<u32, super::GPMEM>;
#[doc = "Writer for register GPMEM[%s]"]
pub type W = crate::W<u32, super::GPMEM>;
#[doc = "Register GPMEM[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::GPMEM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPMEM`"]
pub type GPMEM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPMEM`"]
pub struct GPMEM_W<'a> {
    w: &'a mut W,
}
impl<'a> GPMEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - General purpose memory"]
    #[inline(always)]
    pub fn gpmem(&self) -> GPMEM_R {
        GPMEM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - General purpose memory"]
    #[inline(always)]
    pub fn gpmem(&mut self) -> GPMEM_W {
        GPMEM_W { w: self }
    }
}

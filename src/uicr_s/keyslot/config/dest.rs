#[doc = "Reader of register DEST"]
pub type R = crate::R<u32, super::DEST>;
#[doc = "Writer for register DEST"]
pub type W = crate::W<u32, super::DEST>;
#[doc = "Register DEST `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::DEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `DEST`"]
pub type DEST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DEST`"]
pub struct DEST_W<'a> {
    w: &'a mut W,
}
impl<'a> DEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Secure APB destination address"]
    #[inline(always)]
    pub fn dest(&self) -> DEST_R {
        DEST_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure APB destination address"]
    #[inline(always)]
    pub fn dest(&mut self) -> DEST_W {
        DEST_W { w: self }
    }
}

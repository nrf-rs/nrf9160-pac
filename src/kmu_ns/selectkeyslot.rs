#[doc = "Reader of register SELECTKEYSLOT"]
pub type R = crate::R<u32, super::SELECTKEYSLOT>;
#[doc = "Writer for register SELECTKEYSLOT"]
pub type W = crate::W<u32, super::SELECTKEYSLOT>;
#[doc = "Register SELECTKEYSLOT `reset()`'s with value 0"]
impl crate::ResetValue for super::SELECTKEYSLOT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\]
and UICR-&gt;KEYSLOT.CONFIG\\[N\\]
corresponds to KMU key slot ID=N+1"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Select key slot ID to be read over AHB, or pushed over secure APB, when TASKS_PUSH_KEYSLOT is started NOTE: ID=0 is not a valid key slot ID. The 0 ID should be used when the KMU is idle or not in use NOTE: Index N in UICR-&gt;KEYSLOT.KEY\\[N\\]
and UICR-&gt;KEYSLOT.CONFIG\\[N\\]
corresponds to KMU key slot ID=N+1"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
}

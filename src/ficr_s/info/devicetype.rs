#[doc = "Reader of register DEVICETYPE"]
pub type R = crate::R<u32, super::DEVICETYPE>;
#[doc = "Device type\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVICETYPE_A {
    #[doc = "0: Device is an physical DIE"]
    DIE,
    #[doc = "4294967295: Device is an FPGA"]
    FPGA,
}
impl From<DEVICETYPE_A> for u32 {
    #[inline(always)]
    fn from(variant: DEVICETYPE_A) -> Self {
        match variant {
            DEVICETYPE_A::DIE => 0,
            DEVICETYPE_A::FPGA => 4294967295,
        }
    }
}
#[doc = "Reader of field `DEVICETYPE`"]
pub type DEVICETYPE_R = crate::R<u32, DEVICETYPE_A>;
impl DEVICETYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, DEVICETYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DEVICETYPE_A::DIE),
            4294967295 => Val(DEVICETYPE_A::FPGA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIE`"]
    #[inline(always)]
    pub fn is_die(&self) -> bool {
        *self == DEVICETYPE_A::DIE
    }
    #[doc = "Checks if the value of the field is `FPGA`"]
    #[inline(always)]
    pub fn is_fpga(&self) -> bool {
        *self == DEVICETYPE_A::FPGA
    }
}
impl R {
    #[doc = "Bits 0:31 - Device type"]
    #[inline(always)]
    pub fn devicetype(&self) -> DEVICETYPE_R {
        DEVICETYPE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

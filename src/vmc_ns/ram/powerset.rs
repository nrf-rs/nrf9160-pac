#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POWERSET {
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
#[doc = "Values that can be written to the field `S0POWER`"]
pub enum S0POWERW {
    #[doc = "On"]
    ON,
}
impl S0POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S0POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S0POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S0POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S0POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S0POWERW::ON)
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
#[doc = "Values that can be written to the field `S1POWER`"]
pub enum S1POWERW {
    #[doc = "On"]
    ON,
}
impl S1POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S1POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S1POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S1POWERW::ON)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S2POWER`"]
pub enum S2POWERW {
    #[doc = "On"]
    ON,
}
impl S2POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S2POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S2POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S2POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S2POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S2POWERW::ON)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S3POWER`"]
pub enum S3POWERW {
    #[doc = "On"]
    ON,
}
impl S3POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S3POWERW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S3POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S3POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S3POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S3POWERW::ON)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S0RETENTION`"]
pub enum S0RETENTIONW {
    #[doc = "On"]
    ON,
}
impl S0RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S0RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S0RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S0RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S0RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S0RETENTIONW::ON)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S1RETENTION`"]
pub enum S1RETENTIONW {
    #[doc = "On"]
    ON,
}
impl S1RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S1RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S1RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S1RETENTIONW::ON)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S2RETENTION`"]
pub enum S2RETENTIONW {
    #[doc = "On"]
    ON,
}
impl S2RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S2RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S2RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S2RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S2RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S2RETENTIONW::ON)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S3RETENTION`"]
pub enum S3RETENTIONW {
    #[doc = "On"]
    ON,
}
impl S3RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S3RETENTIONW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S3RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S3RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S3RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(S3RETENTIONW::ON)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65535 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Keep RAM section S0 of RAM n on or off in System ON mode"]
    #[inline]
    pub fn s0power(&mut self) -> _S0POWERW {
        _S0POWERW { w: self }
    }
    #[doc = "Bit 1 - Keep RAM section S1 of RAM n on or off in System ON mode"]
    #[inline]
    pub fn s1power(&mut self) -> _S1POWERW {
        _S1POWERW { w: self }
    }
    #[doc = "Bit 2 - Keep RAM section S2 of RAM n on or off in System ON mode"]
    #[inline]
    pub fn s2power(&mut self) -> _S2POWERW {
        _S2POWERW { w: self }
    }
    #[doc = "Bit 3 - Keep RAM section S3 of RAM n on or off in System ON mode"]
    #[inline]
    pub fn s3power(&mut self) -> _S3POWERW {
        _S3POWERW { w: self }
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 of RAM n when RAM section is switched off"]
    #[inline]
    pub fn s0retention(&mut self) -> _S0RETENTIONW {
        _S0RETENTIONW { w: self }
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 of RAM n when RAM section is switched off"]
    #[inline]
    pub fn s1retention(&mut self) -> _S1RETENTIONW {
        _S1RETENTIONW { w: self }
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 of RAM n when RAM section is switched off"]
    #[inline]
    pub fn s2retention(&mut self) -> _S2RETENTIONW {
        _S2RETENTIONW { w: self }
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 of RAM n when RAM section is switched off"]
    #[inline]
    pub fn s3retention(&mut self) -> _S3RETENTIONW {
        _S3RETENTIONW { w: self }
    }
}

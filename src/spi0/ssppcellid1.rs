#[doc = "Register `SSPPCELLID1` reader"]
pub struct R(crate::R<SSPPCELLID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSPPCELLID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSPPCELLID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSPPCELLID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SSPPCELLID1` reader - These bits read back as 0xF0"]
pub struct SSPPCELLID1_R(crate::FieldReader<u8, u8>);
impl SSPPCELLID1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSPPCELLID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSPPCELLID1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xF0"]
    #[inline(always)]
    pub fn ssppcellid1(&self) -> SSPPCELLID1_R {
        SSPPCELLID1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ssppcellid1](index.html) module"]
pub struct SSPPCELLID1_SPEC;
impl crate::RegisterSpec for SSPPCELLID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssppcellid1::R](R) reader structure"]
impl crate::Readable for SSPPCELLID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SSPPCELLID1 to value 0xf0"]
impl crate::Resettable for SSPPCELLID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}

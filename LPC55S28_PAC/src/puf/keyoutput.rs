#[doc = "Register `KEYOUTPUT` reader"]
pub struct R(crate::R<KEYOUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYOUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYOUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYOUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEYOUT` reader - Key output data"]
pub type KEYOUT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key output data"]
    #[inline(always)]
    pub fn keyout(&self) -> KEYOUT_R {
        KEYOUT_R::new(self.bits)
    }
}
#[doc = "PUF Key Output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyoutput](index.html) module"]
pub struct KEYOUTPUT_SPEC;
impl crate::RegisterSpec for KEYOUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyoutput::R](R) reader structure"]
impl crate::Readable for KEYOUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets KEYOUTPUT to value 0"]
impl crate::Resettable for KEYOUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

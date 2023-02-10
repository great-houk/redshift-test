#[doc = "Register `DIGEST0[%s]` reader"]
pub struct R(crate::R<DIGEST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIGEST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIGEST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIGEST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIGEST` reader - One word of the Digest or output. Note that only 1st 4 are populated for AES and 1st 5 are populated for SHA1."]
pub type DIGEST_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - One word of the Digest or output. Note that only 1st 4 are populated for AES and 1st 5 are populated for SHA1."]
    #[inline(always)]
    pub fn digest(&self) -> DIGEST_R {
        DIGEST_R::new(self.bits)
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [digest0](index.html) module"]
pub struct DIGEST0_SPEC;
impl crate::RegisterSpec for DIGEST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [digest0::R](R) reader structure"]
impl crate::Readable for DIGEST0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIGEST0[%s]
to value 0"]
impl crate::Resettable for DIGEST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

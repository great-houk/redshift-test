#[doc = "Register `SHA256_DIGEST[%s]` reader"]
pub struct R(crate::R<SHA256_DIGEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHA256_DIGEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHA256_DIGEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHA256_DIGEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHA256_DIGEST[%s]` writer"]
pub struct W(crate::W<SHA256_DIGEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHA256_DIGEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SHA256_DIGEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHA256_DIGEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIELD` reader - ."]
pub type FIELD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FIELD` writer - ."]
pub type FIELD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHA256_DIGEST_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&self) -> FIELD_R {
        FIELD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    #[must_use]
    pub fn field(&mut self) -> FIELD_W<0> {
        FIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SHA256_DIGEST0 for DIGEST\\[31:0\\]
SHA256_DIGEST1 for DIGEST\\[63:32\\]
SHA256_DIGEST2 for DIGEST\\[95:64\\]
SHA256_DIGEST3 for DIGEST\\[127:96\\]
SHA256_DIGEST4 for DIGEST\\[159:128\\]
SHA256_DIGEST5 for DIGEST\\[191:160\\]
SHA256_DIGEST6 for DIGEST\\[223:192\\]
SHA256_DIGEST7 for DIGEST\\[255:224\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sha256_digest](index.html) module"]
pub struct SHA256_DIGEST_SPEC;
impl crate::RegisterSpec for SHA256_DIGEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sha256_digest::R](R) reader structure"]
impl crate::Readable for SHA256_DIGEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sha256_digest::W](W) writer structure"]
impl crate::Writable for SHA256_DIGEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHA256_DIGEST[%s]
to value 0"]
impl crate::Resettable for SHA256_DIGEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `SBKEY_HEADER1` reader"]
pub struct R(crate::R<SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBKEY_HEADER1` writer"]
pub struct W(crate::W<SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC>;
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
impl From<crate::W<SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TYPE` reader - ."]
pub type TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TYPE` writer - ."]
pub type TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC, u8, u8, 2, O>;
#[doc = "Field `INDEX` reader - ."]
pub type INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INDEX` writer - ."]
pub type INDEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SIZE` reader - ."]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` writer - ."]
pub type SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:1 - ."]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - ."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - ."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ."]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TYPE_W<0> {
        TYPE_W::new(self)
    }
    #[doc = "Bits 8:11 - ."]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<8> {
        INDEX_W::new(self)
    }
    #[doc = "Bits 24:29 - ."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<24> {
        SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbkey_key_code_sbkey_header1](index.html) module"]
pub struct SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC;
impl crate::RegisterSpec for SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbkey_key_code_sbkey_header1::R](R) reader structure"]
impl crate::Readable for SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbkey_key_code_sbkey_header1::W](W) writer structure"]
impl crate::Writable for SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBKEY_HEADER1 to value 0"]
impl crate::Resettable for SBKEY_KEY_CODE_SBKEY_HEADER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

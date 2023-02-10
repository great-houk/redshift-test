#[doc = "Register `KEYINDEX` reader"]
pub struct R(crate::R<KEYINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYINDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYINDEX` writer"]
pub struct W(crate::W<KEYINDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYINDEX_SPEC>;
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
impl From<crate::W<KEYINDEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYINDEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYIDX` reader - Key index for Set Key operations"]
pub type KEYIDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEYIDX` writer - Key index for Set Key operations"]
pub type KEYIDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYINDEX_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Key index for Set Key operations"]
    #[inline(always)]
    pub fn keyidx(&self) -> KEYIDX_R {
        KEYIDX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Key index for Set Key operations"]
    #[inline(always)]
    #[must_use]
    pub fn keyidx(&mut self) -> KEYIDX_W<0> {
        KEYIDX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Key Index register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyindex](index.html) module"]
pub struct KEYINDEX_SPEC;
impl crate::RegisterSpec for KEYINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyindex::R](R) reader structure"]
impl crate::Readable for KEYINDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyindex::W](W) writer structure"]
impl crate::Writable for KEYINDEX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYINDEX to value 0"]
impl crate::Resettable for KEYINDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

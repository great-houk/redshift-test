#[doc = "Register `SDIO_CFG` reader"]
pub struct R(crate::R<SDIO_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_CFG` writer"]
pub struct W(crate::W<SDIO_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_CFG_SPEC>;
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
impl From<crate::W<SDIO_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIELD` reader - ."]
pub type FIELD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FIELD` writer - ."]
pub type FIELD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDIO_CFG_SPEC, u32, u32, 32, O>;
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
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_cfg](index.html) module"]
pub struct SDIO_CFG_SPEC;
impl crate::RegisterSpec for SDIO_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_cfg::R](R) reader structure"]
impl crate::Readable for SDIO_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_cfg::W](W) writer structure"]
impl crate::Writable for SDIO_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_CFG to value 0"]
impl crate::Resettable for SDIO_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

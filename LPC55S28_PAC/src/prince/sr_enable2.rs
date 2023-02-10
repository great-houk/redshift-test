#[doc = "Register `SR_ENABLE2` reader"]
pub struct R(crate::R<SR_ENABLE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_ENABLE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_ENABLE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_ENABLE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR_ENABLE2` writer"]
pub struct W(crate::W<SR_ENABLE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_ENABLE2_SPEC>;
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
impl From<crate::W<SR_ENABLE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_ENABLE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
pub type EN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EN` writer - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
pub type EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_ENABLE2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub-Region Enable register for region 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr_enable2](index.html) module"]
pub struct SR_ENABLE2_SPEC;
impl crate::RegisterSpec for SR_ENABLE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr_enable2::R](R) reader structure"]
impl crate::Readable for SR_ENABLE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr_enable2::W](W) writer structure"]
impl crate::Writable for SR_ENABLE2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR_ENABLE2 to value 0"]
impl crate::Resettable for SR_ENABLE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

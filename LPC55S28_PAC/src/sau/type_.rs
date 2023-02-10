#[doc = "Register `TYPE` reader"]
pub struct R(crate::R<TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TYPE` writer"]
pub struct W(crate::W<TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TYPE_SPEC>;
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
impl From<crate::W<TYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SREGION` reader - SAU regions. The number of implemented SAU regions."]
pub type SREGION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SREGION` writer - SAU regions. The number of implemented SAU regions."]
pub type SREGION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TYPE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SAU regions. The number of implemented SAU regions."]
    #[inline(always)]
    pub fn sregion(&self) -> SREGION_R {
        SREGION_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SAU regions. The number of implemented SAU regions."]
    #[inline(always)]
    #[must_use]
    pub fn sregion(&mut self) -> SREGION_W<0> {
        SREGION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Attribution Unit Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type_](index.html) module"]
pub struct TYPE_SPEC;
impl crate::RegisterSpec for TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [type_::R](R) reader structure"]
impl crate::Readable for TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [type_::W](W) writer structure"]
impl crate::Writable for TYPE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TYPE to value 0"]
impl crate::Resettable for TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

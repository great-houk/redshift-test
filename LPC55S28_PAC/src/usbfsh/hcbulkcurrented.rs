#[doc = "Register `HCBULKCURRENTED` reader"]
pub struct R(crate::R<HCBULKCURRENTED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCBULKCURRENTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCBULKCURRENTED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCBULKCURRENTED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCBULKCURRENTED` writer"]
pub struct W(crate::W<HCBULKCURRENTED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCBULKCURRENTED_SPEC>;
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
impl From<crate::W<HCBULKCURRENTED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCBULKCURRENTED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCED` reader - BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
pub type BCED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BCED` writer - BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
pub type BCED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HCBULKCURRENTED_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 4:31 - BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
    #[inline(always)]
    pub fn bced(&self) -> BCED_R {
        BCED_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
    #[inline(always)]
    #[must_use]
    pub fn bced(&mut self) -> BCED_W<4> {
        BCED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the physical address of the current endpoint descriptor of the bulk list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcbulkcurrented](index.html) module"]
pub struct HCBULKCURRENTED_SPEC;
impl crate::RegisterSpec for HCBULKCURRENTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcbulkcurrented::R](R) reader structure"]
impl crate::Readable for HCBULKCURRENTED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcbulkcurrented::W](W) writer structure"]
impl crate::Writable for HCBULKCURRENTED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCBULKCURRENTED to value 0"]
impl crate::Resettable for HCBULKCURRENTED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

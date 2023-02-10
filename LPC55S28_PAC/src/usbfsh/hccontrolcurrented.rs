#[doc = "Register `HCCONTROLCURRENTED` reader"]
pub struct R(crate::R<HCCONTROLCURRENTED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCONTROLCURRENTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCONTROLCURRENTED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCONTROLCURRENTED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCONTROLCURRENTED` writer"]
pub struct W(crate::W<HCCONTROLCURRENTED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCONTROLCURRENTED_SPEC>;
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
impl From<crate::W<HCCONTROLCURRENTED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCONTROLCURRENTED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCED` reader - ControlCurrentED."]
pub type CCED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CCED` writer - ControlCurrentED."]
pub type CCED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HCCONTROLCURRENTED_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 4:31 - ControlCurrentED."]
    #[inline(always)]
    pub fn cced(&self) -> CCED_R {
        CCED_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - ControlCurrentED."]
    #[inline(always)]
    #[must_use]
    pub fn cced(&mut self) -> CCED_W<4> {
        CCED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the physical address of the current endpoint descriptor of the control list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccontrolcurrented](index.html) module"]
pub struct HCCONTROLCURRENTED_SPEC;
impl crate::RegisterSpec for HCCONTROLCURRENTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hccontrolcurrented::R](R) reader structure"]
impl crate::Readable for HCCONTROLCURRENTED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hccontrolcurrented::W](W) writer structure"]
impl crate::Writable for HCCONTROLCURRENTED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCCONTROLCURRENTED to value 0"]
impl crate::Resettable for HCCONTROLCURRENTED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

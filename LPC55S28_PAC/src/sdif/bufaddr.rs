#[doc = "Register `BUFADDR` reader"]
pub struct R(crate::R<BUFADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUFADDR` writer"]
pub struct W(crate::W<BUFADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUFADDR_SPEC>;
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
impl From<crate::W<BUFADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUFADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HBA` reader - Host Buffer Address Pointer."]
pub type HBA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HBA` writer - Host Buffer Address Pointer."]
pub type HBA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUFADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Host Buffer Address Pointer."]
    #[inline(always)]
    pub fn hba(&self) -> HBA_R {
        HBA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Buffer Address Pointer."]
    #[inline(always)]
    #[must_use]
    pub fn hba(&mut self) -> HBA_W<0> {
        HBA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current Buffer Descriptor Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bufaddr](index.html) module"]
pub struct BUFADDR_SPEC;
impl crate::RegisterSpec for BUFADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bufaddr::R](R) reader structure"]
impl crate::Readable for BUFADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bufaddr::W](W) writer structure"]
impl crate::Writable for BUFADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUFADDR to value 0"]
impl crate::Resettable for BUFADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

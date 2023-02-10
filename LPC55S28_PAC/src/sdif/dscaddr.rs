#[doc = "Register `DSCADDR` reader"]
pub struct R(crate::R<DSCADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSCADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSCADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSCADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSCADDR` writer"]
pub struct W(crate::W<DSCADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSCADDR_SPEC>;
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
impl From<crate::W<DSCADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSCADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDA` reader - Host Descriptor Address Pointer."]
pub type HDA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HDA` writer - Host Descriptor Address Pointer."]
pub type HDA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSCADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Host Descriptor Address Pointer."]
    #[inline(always)]
    pub fn hda(&self) -> HDA_R {
        HDA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Descriptor Address Pointer."]
    #[inline(always)]
    #[must_use]
    pub fn hda(&mut self) -> HDA_W<0> {
        HDA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current Host Descriptor Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscaddr](index.html) module"]
pub struct DSCADDR_SPEC;
impl crate::RegisterSpec for DSCADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dscaddr::R](R) reader structure"]
impl crate::Readable for DSCADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dscaddr::W](W) writer structure"]
impl crate::Writable for DSCADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSCADDR to value 0"]
impl crate::Resettable for DSCADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

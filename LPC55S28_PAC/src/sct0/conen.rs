#[doc = "Register `CONEN` reader"]
pub struct R(crate::R<CONEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONEN` writer"]
pub struct W(crate::W<CONEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONEN_SPEC>;
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
impl From<crate::W<CONEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCEN` reader - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub type NCEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NCEN` writer - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub type NCEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn ncen(&self) -> NCEN_R {
        NCEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn ncen(&mut self) -> NCEN_W<0> {
        NCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT conflict interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conen](index.html) module"]
pub struct CONEN_SPEC;
impl crate::RegisterSpec for CONEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conen::R](R) reader structure"]
impl crate::Readable for CONEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conen::W](W) writer structure"]
impl crate::Writable for CONEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONEN to value 0"]
impl crate::Resettable for CONEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

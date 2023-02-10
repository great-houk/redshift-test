#[doc = "Register `HCLSTHRESHOLD` reader"]
pub struct R(crate::R<HCLSTHRESHOLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCLSTHRESHOLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCLSTHRESHOLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCLSTHRESHOLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCLSTHRESHOLD` writer"]
pub struct W(crate::W<HCLSTHRESHOLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCLSTHRESHOLD_SPEC>;
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
impl From<crate::W<HCLSTHRESHOLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCLSTHRESHOLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LST` reader - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
pub type LST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LST` writer - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
pub type LST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCLSTHRESHOLD_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
    #[inline(always)]
    #[must_use]
    pub fn lst(&mut self) -> LST_W<0> {
        LST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hclsthreshold](index.html) module"]
pub struct HCLSTHRESHOLD_SPEC;
impl crate::RegisterSpec for HCLSTHRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hclsthreshold::R](R) reader structure"]
impl crate::Readable for HCLSTHRESHOLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hclsthreshold::W](W) writer structure"]
impl crate::Writable for HCLSTHRESHOLD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCLSTHRESHOLD to value 0x0628"]
impl crate::Resettable for HCLSTHRESHOLD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0628;
}

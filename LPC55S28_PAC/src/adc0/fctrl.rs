#[doc = "Register `FCTRL[%s]` reader"]
pub struct R(crate::R<FCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCTRL[%s]` writer"]
pub struct W(crate::W<FCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCTRL_SPEC>;
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
impl From<crate::W<FCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCOUNT` reader - Result FIFO counter"]
pub type FCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FWMARK` reader - Watermark level selection"]
pub type FWMARK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FWMARK` writer - Watermark level selection"]
pub type FWMARK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:4 - Result FIFO counter"]
    #[inline(always)]
    pub fn fcount(&self) -> FCOUNT_R {
        FCOUNT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Watermark level selection"]
    #[inline(always)]
    pub fn fwmark(&self) -> FWMARK_R {
        FWMARK_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Watermark level selection"]
    #[inline(always)]
    #[must_use]
    pub fn fwmark(&mut self) -> FWMARK_W<16> {
        FWMARK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctrl](index.html) module"]
pub struct FCTRL_SPEC;
impl crate::RegisterSpec for FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fctrl::R](R) reader structure"]
impl crate::Readable for FCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fctrl::W](W) writer structure"]
impl crate::Writable for FCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTRL[%s]
to value 0"]
impl crate::Resettable for FCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

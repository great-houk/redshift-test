#[doc = "Register `FLADJ_FRINDEX` reader"]
pub struct R(crate::R<FLADJ_FRINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLADJ_FRINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLADJ_FRINDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLADJ_FRINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLADJ_FRINDEX` writer"]
pub struct W(crate::W<FLADJ_FRINDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLADJ_FRINDEX_SPEC>;
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
impl From<crate::W<FLADJ_FRINDEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLADJ_FRINDEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLADJ` reader - Frame Length Timing Value."]
pub type FLADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLADJ` writer - Frame Length Timing Value."]
pub type FLADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLADJ_FRINDEX_SPEC, u8, u8, 6, O>;
#[doc = "Field `FRINDEX` reader - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
pub type FRINDEX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRINDEX` writer - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
pub type FRINDEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLADJ_FRINDEX_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:5 - Frame Length Timing Value."]
    #[inline(always)]
    pub fn fladj(&self) -> FLADJ_R {
        FLADJ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:29 - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[inline(always)]
    pub fn frindex(&self) -> FRINDEX_R {
        FRINDEX_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Frame Length Timing Value."]
    #[inline(always)]
    #[must_use]
    pub fn fladj(&mut self) -> FLADJ_W<0> {
        FLADJ_W::new(self)
    }
    #[doc = "Bits 16:29 - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[inline(always)]
    #[must_use]
    pub fn frindex(&mut self) -> FRINDEX_W<16> {
        FRINDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Length Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fladj_frindex](index.html) module"]
pub struct FLADJ_FRINDEX_SPEC;
impl crate::RegisterSpec for FLADJ_FRINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fladj_frindex::R](R) reader structure"]
impl crate::Readable for FLADJ_FRINDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fladj_frindex::W](W) writer structure"]
impl crate::Writable for FLADJ_FRINDEX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLADJ_FRINDEX to value 0x20"]
impl crate::Resettable for FLADJ_FRINDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}

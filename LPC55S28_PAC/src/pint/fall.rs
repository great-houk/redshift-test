#[doc = "Register `FALL` reader"]
pub struct R(crate::R<FALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FALL` writer"]
pub struct W(crate::W<FALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FALL_SPEC>;
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
impl From<crate::W<FALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDET` reader - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
pub type FDET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FDET` writer - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
pub type FDET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FALL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
    #[inline(always)]
    pub fn fdet(&self) -> FDET_R {
        FDET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
    #[inline(always)]
    #[must_use]
    pub fn fdet(&mut self) -> FDET_W<0> {
        FDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt falling edge register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fall](index.html) module"]
pub struct FALL_SPEC;
impl crate::RegisterSpec for FALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fall::R](R) reader structure"]
impl crate::Readable for FALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fall::W](W) writer structure"]
impl crate::Writable for FALL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FALL to value 0"]
impl crate::Resettable for FALL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

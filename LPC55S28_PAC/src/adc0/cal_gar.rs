#[doc = "Register `CAL_GAR[%s]` reader"]
pub struct R(crate::R<CAL_GAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_GAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_GAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_GAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_GAR[%s]` writer"]
pub struct W(crate::W<CAL_GAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_GAR_SPEC>;
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
impl From<crate::W<CAL_GAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_GAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL_GAR_VAL` reader - Calibration General A Side Register Element"]
pub type CAL_GAR_VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAL_GAR_VAL` writer - Calibration General A Side Register Element"]
pub type CAL_GAR_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAL_GAR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Calibration General A Side Register Element"]
    #[inline(always)]
    pub fn cal_gar_val(&self) -> CAL_GAR_VAL_R {
        CAL_GAR_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Calibration General A Side Register Element"]
    #[inline(always)]
    #[must_use]
    pub fn cal_gar_val(&mut self) -> CAL_GAR_VAL_W<0> {
        CAL_GAR_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration General A-Side Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_gar](index.html) module"]
pub struct CAL_GAR_SPEC;
impl crate::RegisterSpec for CAL_GAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_gar::R](R) reader structure"]
impl crate::Readable for CAL_GAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_gar::W](W) writer structure"]
impl crate::Writable for CAL_GAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_GAR[%s]
to value 0"]
impl crate::Resettable for CAL_GAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

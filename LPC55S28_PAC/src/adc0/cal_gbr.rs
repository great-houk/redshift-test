#[doc = "Register `CAL_GBR[%s]` reader"]
pub struct R(crate::R<CAL_GBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_GBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_GBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_GBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_GBR[%s]` writer"]
pub struct W(crate::W<CAL_GBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_GBR_SPEC>;
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
impl From<crate::W<CAL_GBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_GBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL_GBR_VAL` reader - Calibration General B Side Register Element"]
pub type CAL_GBR_VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CAL_GBR_VAL` writer - Calibration General B Side Register Element"]
pub type CAL_GBR_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAL_GBR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Calibration General B Side Register Element"]
    #[inline(always)]
    pub fn cal_gbr_val(&self) -> CAL_GBR_VAL_R {
        CAL_GBR_VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Calibration General B Side Register Element"]
    #[inline(always)]
    #[must_use]
    pub fn cal_gbr_val(&mut self) -> CAL_GBR_VAL_W<0> {
        CAL_GBR_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration General B-Side Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_gbr](index.html) module"]
pub struct CAL_GBR_SPEC;
impl crate::RegisterSpec for CAL_GBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_gbr::R](R) reader structure"]
impl crate::Readable for CAL_GBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_gbr::W](W) writer structure"]
impl crate::Writable for CAL_GBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_GBR[%s]
to value 0"]
impl crate::Resettable for CAL_GBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `CPU0NSTCKCAL` reader"]
pub struct R(crate::R<CPU0NSTCKCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU0NSTCKCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU0NSTCKCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU0NSTCKCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU0NSTCKCAL` writer"]
pub struct W(crate::W<CPU0NSTCKCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU0NSTCKCAL_SPEC>;
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
impl From<crate::W<CPU0NSTCKCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU0NSTCKCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TENMS` reader - Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
pub type TENMS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TENMS` writer - Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
pub type TENMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPU0NSTCKCAL_SPEC, u32, u32, 24, O>;
#[doc = "Field `SKEW` reader - Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
pub type SKEW_R = crate::BitReader<bool>;
#[doc = "Field `SKEW` writer - Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
pub type SKEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU0NSTCKCAL_SPEC, bool, O>;
#[doc = "Field `NOREF` reader - Initial value for the Systick timer."]
pub type NOREF_R = crate::BitReader<bool>;
#[doc = "Field `NOREF` writer - Initial value for the Systick timer."]
pub type NOREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU0NSTCKCAL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    #[must_use]
    pub fn tenms(&mut self) -> TENMS_W<0> {
        TENMS_W::new(self)
    }
    #[doc = "Bit 24 - Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
    #[inline(always)]
    #[must_use]
    pub fn skew(&mut self) -> SKEW_W<24> {
        SKEW_W::new(self)
    }
    #[doc = "Bit 25 - Initial value for the Systick timer."]
    #[inline(always)]
    #[must_use]
    pub fn noref(&mut self) -> NOREF_W<25> {
        NOREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System tick calibration for non-secure part of CPU0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu0nstckcal](index.html) module"]
pub struct CPU0NSTCKCAL_SPEC;
impl crate::RegisterSpec for CPU0NSTCKCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu0nstckcal::R](R) reader structure"]
impl crate::Readable for CPU0NSTCKCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu0nstckcal::W](W) writer structure"]
impl crate::Writable for CPU0NSTCKCAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU0NSTCKCAL to value 0"]
impl crate::Resettable for CPU0NSTCKCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

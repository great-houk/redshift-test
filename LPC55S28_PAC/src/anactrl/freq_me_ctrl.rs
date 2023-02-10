#[doc = "Register `FREQ_ME_CTRL` reader"]
pub struct R(crate::R<FREQ_ME_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQ_ME_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQ_ME_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQ_ME_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQ_ME_CTRL` writer"]
pub struct W(crate::W<FREQ_ME_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQ_ME_CTRL_SPEC>;
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
impl From<crate::W<FREQ_ME_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQ_ME_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPVAL_SCALE` reader - Frequency measure result /Frequency measur scale"]
pub type CAPVAL_SCALE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CAPVAL_SCALE` writer - Frequency measure result /Frequency measur scale"]
pub type CAPVAL_SCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FREQ_ME_CTRL_SPEC, u32, u32, 31, O>;
#[doc = "Field `PROG` reader - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
pub type PROG_R = crate::BitReader<bool>;
#[doc = "Field `PROG` writer - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
pub type PROG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FREQ_ME_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:30 - Frequency measure result /Frequency measur scale"]
    #[inline(always)]
    pub fn capval_scale(&self) -> CAPVAL_SCALE_R {
        CAPVAL_SCALE_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Frequency measure result /Frequency measur scale"]
    #[inline(always)]
    #[must_use]
    pub fn capval_scale(&mut self) -> CAPVAL_SCALE_W<0> {
        CAPVAL_SCALE_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
    #[inline(always)]
    #[must_use]
    pub fn prog(&mut self) -> PROG_W<31> {
        PROG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency Measure function control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freq_me_ctrl](index.html) module"]
pub struct FREQ_ME_CTRL_SPEC;
impl crate::RegisterSpec for FREQ_ME_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freq_me_ctrl::R](R) reader structure"]
impl crate::Readable for FREQ_ME_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freq_me_ctrl::W](W) writer structure"]
impl crate::Writable for FREQ_ME_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREQ_ME_CTRL to value 0"]
impl crate::Resettable for FREQ_ME_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `IST` reader"]
pub struct R(crate::R<IST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IST` writer"]
pub struct W(crate::W<IST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IST_SPEC>;
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
impl From<crate::W<IST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSTAT` reader - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
pub type PSTAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSTAT` writer - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
pub type PSTAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IST_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
    #[inline(always)]
    pub fn pstat(&self) -> PSTAT_R {
        PSTAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
    #[inline(always)]
    #[must_use]
    pub fn pstat(&mut self) -> PSTAT_W<0> {
        PSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ist](index.html) module"]
pub struct IST_SPEC;
impl crate::RegisterSpec for IST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ist::R](R) reader structure"]
impl crate::Readable for IST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ist::W](W) writer structure"]
impl crate::Writable for IST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IST to value 0"]
impl crate::Resettable for IST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

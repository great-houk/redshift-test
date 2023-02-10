#[doc = "Register `IENF` reader"]
pub struct R(crate::R<IENF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IENF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IENF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IENF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IENF` writer"]
pub struct W(crate::W<IENF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IENF_SPEC>;
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
impl From<crate::W<IENF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IENF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENAF` reader - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
pub type ENAF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENAF` writer - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
pub type ENAF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IENF_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
    #[inline(always)]
    pub fn enaf(&self) -> ENAF_R {
        ENAF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
    #[inline(always)]
    #[must_use]
    pub fn enaf(&mut self) -> ENAF_W<0> {
        ENAF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt active level or falling edge interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ienf](index.html) module"]
pub struct IENF_SPEC;
impl crate::RegisterSpec for IENF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ienf::R](R) reader structure"]
impl crate::Readable for IENF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ienf::W](W) writer structure"]
impl crate::Writable for IENF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IENF to value 0"]
impl crate::Resettable for IENF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

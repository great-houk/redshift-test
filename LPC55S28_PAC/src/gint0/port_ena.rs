#[doc = "Register `PORT_ENA[%s]` reader"]
pub struct R(crate::R<PORT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_ENA[%s]` writer"]
pub struct W(crate::W<PORT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_ENA_SPEC>;
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
impl From<crate::W<PORT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA` reader - Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ENA` writer - Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt."]
pub type ENA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORT_ENA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable port 0 pin for group interrupt. Bit n corresponds to pin Pm_n of port m. 0 = the port 0 pin is disabled and does not contribute to the grouped interrupt. 1 = the port 0 pin is enabled and contributes to the grouped interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ena(&mut self) -> ENA_W<0> {
        ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO grouped interrupt port 0 enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_ena](index.html) module"]
pub struct PORT_ENA_SPEC;
impl crate::RegisterSpec for PORT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_ena::R](R) reader structure"]
impl crate::Readable for PORT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_ena::W](W) writer structure"]
impl crate::Writable for PORT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORT_ENA[%s]
to value 0"]
impl crate::Resettable for PORT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

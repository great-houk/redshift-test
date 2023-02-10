#[doc = "Register `PORT_POL[%s]` reader"]
pub struct R(crate::R<PORT_POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORT_POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORT_POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT_POL[%s]` writer"]
pub struct W(crate::W<PORT_POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_POL_SPEC>;
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
impl From<crate::W<PORT_POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORT_POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POL` reader - Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `POL` writer - Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
pub type POL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORT_POL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure pin polarity of port m pins for group interrupt. Bit n corresponds to pin PIOm_n of port m. 0 = the pin is active LOW. If the level on this pin is LOW, the pin contributes to the group interrupt. 1 = the pin is active HIGH. If the level on this pin is HIGH, the pin contributes to the group interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<0> {
        POL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO grouped interrupt port 0 polarity register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_pol](index.html) module"]
pub struct PORT_POL_SPEC;
impl crate::RegisterSpec for PORT_POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port_pol::R](R) reader structure"]
impl crate::Readable for PORT_POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port_pol::W](W) writer structure"]
impl crate::Writable for PORT_POL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORT_POL[%s]
to value 0xffff_ffff"]
impl crate::Resettable for PORT_POL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}

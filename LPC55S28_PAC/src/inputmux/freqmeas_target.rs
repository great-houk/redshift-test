#[doc = "Register `FREQMEAS_TARGET` reader"]
pub struct R(crate::R<FREQMEAS_TARGET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQMEAS_TARGET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQMEAS_TARGET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQMEAS_TARGET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQMEAS_TARGET` writer"]
pub struct W(crate::W<FREQMEAS_TARGET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQMEAS_TARGET_SPEC>;
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
impl From<crate::W<FREQMEAS_TARGET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQMEAS_TARGET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKIN` reader - Clock source number (decimal value) for frequency measure function target clock: 0 = CLK_IN 1 = FRO 12 MHz oscillator 2 = Watchdog oscillator 3 = 32 kHz RTC oscillator 4 = Main clock (see Section 4.5.23) 5 = PIO0_4 6 = PIO0_20 7 = PIO0_24 8 = PIO1_4"]
pub type CLKIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKIN` writer - Clock source number (decimal value) for frequency measure function target clock: 0 = CLK_IN 1 = FRO 12 MHz oscillator 2 = Watchdog oscillator 3 = 32 kHz RTC oscillator 4 = Main clock (see Section 4.5.23) 5 = PIO0_4 6 = PIO0_20 7 = PIO0_24 8 = PIO1_4"]
pub type CLKIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FREQMEAS_TARGET_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Clock source number (decimal value) for frequency measure function target clock: 0 = CLK_IN 1 = FRO 12 MHz oscillator 2 = Watchdog oscillator 3 = 32 kHz RTC oscillator 4 = Main clock (see Section 4.5.23) 5 = PIO0_4 6 = PIO0_20 7 = PIO0_24 8 = PIO1_4"]
    #[inline(always)]
    pub fn clkin(&self) -> CLKIN_R {
        CLKIN_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clock source number (decimal value) for frequency measure function target clock: 0 = CLK_IN 1 = FRO 12 MHz oscillator 2 = Watchdog oscillator 3 = 32 kHz RTC oscillator 4 = Main clock (see Section 4.5.23) 5 = PIO0_4 6 = PIO0_20 7 = PIO0_24 8 = PIO1_4"]
    #[inline(always)]
    #[must_use]
    pub fn clkin(&mut self) -> CLKIN_W<0> {
        CLKIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selection for frequency measurement target clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freqmeas_target](index.html) module"]
pub struct FREQMEAS_TARGET_SPEC;
impl crate::RegisterSpec for FREQMEAS_TARGET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freqmeas_target::R](R) reader structure"]
impl crate::Readable for FREQMEAS_TARGET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freqmeas_target::W](W) writer structure"]
impl crate::Writable for FREQMEAS_TARGET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREQMEAS_TARGET to value 0x1f"]
impl crate::Resettable for FREQMEAS_TARGET_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}

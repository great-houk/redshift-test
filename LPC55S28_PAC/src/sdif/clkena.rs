#[doc = "Register `CLKENA` reader"]
pub struct R(crate::R<CLKENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKENA` writer"]
pub struct W(crate::W<CLKENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKENA_SPEC>;
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
impl From<crate::W<CLKENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCLK0_ENABLE` reader - Clock-enable control for SD card 0 clock."]
pub type CCLK0_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CCLK0_ENABLE` writer - Clock-enable control for SD card 0 clock."]
pub type CCLK0_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKENA_SPEC, bool, O>;
#[doc = "Field `CCLK1_ENABLE` reader - Clock-enable control for SD card 1 clock."]
pub type CCLK1_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CCLK1_ENABLE` writer - Clock-enable control for SD card 1 clock."]
pub type CCLK1_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKENA_SPEC, bool, O>;
#[doc = "Field `CCLK0_LOW_POWER` reader - Low-power control for SD card 0 clock."]
pub type CCLK0_LOW_POWER_R = crate::BitReader<bool>;
#[doc = "Field `CCLK0_LOW_POWER` writer - Low-power control for SD card 0 clock."]
pub type CCLK0_LOW_POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKENA_SPEC, bool, O>;
#[doc = "Field `CCLK1_LOW_POWER` reader - Low-power control for SD card 1 clock."]
pub type CCLK1_LOW_POWER_R = crate::BitReader<bool>;
#[doc = "Field `CCLK1_LOW_POWER` writer - Low-power control for SD card 1 clock."]
pub type CCLK1_LOW_POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKENA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clock-enable control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_enable(&self) -> CCLK0_ENABLE_R {
        CCLK0_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock-enable control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_enable(&self) -> CCLK1_ENABLE_R {
        CCLK1_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Low-power control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_low_power(&self) -> CCLK0_LOW_POWER_R {
        CCLK0_LOW_POWER_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-power control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_low_power(&self) -> CCLK1_LOW_POWER_R {
        CCLK1_LOW_POWER_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock-enable control for SD card 0 clock."]
    #[inline(always)]
    #[must_use]
    pub fn cclk0_enable(&mut self) -> CCLK0_ENABLE_W<0> {
        CCLK0_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Clock-enable control for SD card 1 clock."]
    #[inline(always)]
    #[must_use]
    pub fn cclk1_enable(&mut self) -> CCLK1_ENABLE_W<1> {
        CCLK1_ENABLE_W::new(self)
    }
    #[doc = "Bit 16 - Low-power control for SD card 0 clock."]
    #[inline(always)]
    #[must_use]
    pub fn cclk0_low_power(&mut self) -> CCLK0_LOW_POWER_W<16> {
        CCLK0_LOW_POWER_W::new(self)
    }
    #[doc = "Bit 17 - Low-power control for SD card 1 clock."]
    #[inline(always)]
    #[must_use]
    pub fn cclk1_low_power(&mut self) -> CCLK1_LOW_POWER_W<17> {
        CCLK1_LOW_POWER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkena](index.html) module"]
pub struct CLKENA_SPEC;
impl crate::RegisterSpec for CLKENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkena::R](R) reader structure"]
impl crate::Readable for CLKENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkena::W](W) writer structure"]
impl crate::Writable for CLKENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKENA to value 0"]
impl crate::Resettable for CLKENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

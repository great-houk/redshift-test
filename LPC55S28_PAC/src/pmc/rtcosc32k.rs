#[doc = "Register `RTCOSC32K` reader"]
pub struct R(crate::R<RTCOSC32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCOSC32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCOSC32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCOSC32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCOSC32K` writer"]
pub struct W(crate::W<RTCOSC32K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCOSC32K_SPEC>;
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
impl From<crate::W<RTCOSC32K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCOSC32K_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
pub type SEL_R = crate::BitReader<SEL_A>;
#[doc = "Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEL_A {
    #[doc = "0: FRO 32 KHz."]
    FRO32K = 0,
    #[doc = "1: XTAL 32KHz."]
    XTAL32K = 1,
}
impl From<SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            false => SEL_A::FRO32K,
            true => SEL_A::XTAL32K,
        }
    }
    #[doc = "Checks if the value of the field is `FRO32K`"]
    #[inline(always)]
    pub fn is_fro32k(&self) -> bool {
        *self == SEL_A::FRO32K
    }
    #[doc = "Checks if the value of the field is `XTAL32K`"]
    #[inline(always)]
    pub fn is_xtal32k(&self) -> bool {
        *self == SEL_A::XTAL32K
    }
}
#[doc = "Field `SEL` writer - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
pub type SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCOSC32K_SPEC, SEL_A, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "FRO 32 KHz."]
    #[inline(always)]
    pub fn fro32k(self) -> &'a mut W {
        self.variant(SEL_A::FRO32K)
    }
    #[doc = "XTAL 32KHz."]
    #[inline(always)]
    pub fn xtal32k(self) -> &'a mut W {
        self.variant(SEL_A::XTAL32K)
    }
}
#[doc = "Field `CLK1KHZDIV` reader - Actual division ratio is : 28 + CLK1KHZDIV."]
pub type CLK1KHZDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK1KHZDIV` writer - Actual division ratio is : 28 + CLK1KHZDIV."]
pub type CLK1KHZDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCOSC32K_SPEC, u8, u8, 3, O>;
#[doc = "Field `CLK1KHZDIVUPDATEREQ` reader - RTC 1KHz clock Divider status flag."]
pub type CLK1KHZDIVUPDATEREQ_R = crate::BitReader<bool>;
#[doc = "Field `CLK1KHZDIVUPDATEREQ` writer - RTC 1KHz clock Divider status flag."]
pub type CLK1KHZDIVUPDATEREQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RTCOSC32K_SPEC, bool, O>;
#[doc = "Field `CLK1HZDIV` reader - Actual division ratio is : 31744 + CLK1HZDIV."]
pub type CLK1HZDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLK1HZDIV` writer - Actual division ratio is : 31744 + CLK1HZDIV."]
pub type CLK1HZDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTCOSC32K_SPEC, u16, u16, 11, O>;
#[doc = "Field `CLK1HZDIVHALT` reader - Halts the divider counter."]
pub type CLK1HZDIVHALT_R = crate::BitReader<bool>;
#[doc = "Field `CLK1HZDIVHALT` writer - Halts the divider counter."]
pub type CLK1HZDIVHALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCOSC32K_SPEC, bool, O>;
#[doc = "Field `CLK1HZDIVUPDATEREQ` reader - RTC 1Hz Divider status flag."]
pub type CLK1HZDIVUPDATEREQ_R = crate::BitReader<bool>;
#[doc = "Field `CLK1HZDIVUPDATEREQ` writer - RTC 1Hz Divider status flag."]
pub type CLK1HZDIVUPDATEREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCOSC32K_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Actual division ratio is : 28 + CLK1KHZDIV."]
    #[inline(always)]
    pub fn clk1khzdiv(&self) -> CLK1KHZDIV_R {
        CLK1KHZDIV_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 15 - RTC 1KHz clock Divider status flag."]
    #[inline(always)]
    pub fn clk1khzdivupdatereq(&self) -> CLK1KHZDIVUPDATEREQ_R {
        CLK1KHZDIVUPDATEREQ_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Actual division ratio is : 31744 + CLK1HZDIV."]
    #[inline(always)]
    pub fn clk1hzdiv(&self) -> CLK1HZDIV_R {
        CLK1HZDIV_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    pub fn clk1hzdivhalt(&self) -> CLK1HZDIVHALT_R {
        CLK1HZDIVHALT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RTC 1Hz Divider status flag."]
    #[inline(always)]
    pub fn clk1hzdivupdatereq(&self) -> CLK1HZDIVUPDATEREQ_R {
        CLK1HZDIVUPDATEREQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select the 32K oscillator to be used in Deep Power Down Mode for the RTC (either XTAL32KHz or FRO32KHz) ."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Bits 1:3 - Actual division ratio is : 28 + CLK1KHZDIV."]
    #[inline(always)]
    #[must_use]
    pub fn clk1khzdiv(&mut self) -> CLK1KHZDIV_W<1> {
        CLK1KHZDIV_W::new(self)
    }
    #[doc = "Bit 15 - RTC 1KHz clock Divider status flag."]
    #[inline(always)]
    #[must_use]
    pub fn clk1khzdivupdatereq(&mut self) -> CLK1KHZDIVUPDATEREQ_W<15> {
        CLK1KHZDIVUPDATEREQ_W::new(self)
    }
    #[doc = "Bits 16:26 - Actual division ratio is : 31744 + CLK1HZDIV."]
    #[inline(always)]
    #[must_use]
    pub fn clk1hzdiv(&mut self) -> CLK1HZDIV_W<16> {
        CLK1HZDIV_W::new(self)
    }
    #[doc = "Bit 30 - Halts the divider counter."]
    #[inline(always)]
    #[must_use]
    pub fn clk1hzdivhalt(&mut self) -> CLK1HZDIVHALT_W<30> {
        CLK1HZDIVHALT_W::new(self)
    }
    #[doc = "Bit 31 - RTC 1Hz Divider status flag."]
    #[inline(always)]
    #[must_use]
    pub fn clk1hzdivupdatereq(&mut self) -> CLK1HZDIVUPDATEREQ_W<31> {
        CLK1HZDIVUPDATEREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC 1 KHZ and 1 Hz clocks source control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcosc32k](index.html) module"]
pub struct RTCOSC32K_SPEC;
impl crate::RegisterSpec for RTCOSC32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcosc32k::R](R) reader structure"]
impl crate::Readable for RTCOSC32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcosc32k::W](W) writer structure"]
impl crate::Writable for RTCOSC32K_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCOSC32K to value 0x03ff_0008"]
impl crate::Resettable for RTCOSC32K_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff_0008;
}

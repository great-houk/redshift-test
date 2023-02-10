#[doc = "Register `OSTIMER` reader"]
pub struct R(crate::R<OSTIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSTIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSTIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSTIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSTIMER` writer"]
pub struct W(crate::W<OSTIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSTIMER_SPEC>;
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
impl From<crate::W<OSTIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSTIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFTRESET` reader - Active high reset."]
pub type SOFTRESET_R = crate::BitReader<bool>;
#[doc = "Field `SOFTRESET` writer - Active high reset."]
pub type SOFTRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSTIMER_SPEC, bool, O>;
#[doc = "Field `CLOCKENABLE` reader - Enable OSTIMER 32 KHz clock."]
pub type CLOCKENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CLOCKENABLE` writer - Enable OSTIMER 32 KHz clock."]
pub type CLOCKENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSTIMER_SPEC, bool, O>;
#[doc = "Field `DPDWAKEUPENABLE` reader - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
pub type DPDWAKEUPENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DPDWAKEUPENABLE` writer - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
pub type DPDWAKEUPENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSTIMER_SPEC, bool, O>;
#[doc = "Field `OSC32KPD` reader - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
pub type OSC32KPD_R = crate::BitReader<bool>;
#[doc = "Field `OSC32KPD` writer - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
pub type OSC32KPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSTIMER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Active high reset."]
    #[inline(always)]
    pub fn softreset(&self) -> SOFTRESET_R {
        SOFTRESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable OSTIMER 32 KHz clock."]
    #[inline(always)]
    pub fn clockenable(&self) -> CLOCKENABLE_R {
        CLOCKENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    pub fn dpdwakeupenable(&self) -> DPDWAKEUPENABLE_R {
        DPDWAKEUPENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    pub fn osc32kpd(&self) -> OSC32KPD_R {
        OSC32KPD_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active high reset."]
    #[inline(always)]
    #[must_use]
    pub fn softreset(&mut self) -> SOFTRESET_W<0> {
        SOFTRESET_W::new(self)
    }
    #[doc = "Bit 1 - Enable OSTIMER 32 KHz clock."]
    #[inline(always)]
    #[must_use]
    pub fn clockenable(&mut self) -> CLOCKENABLE_W<1> {
        CLOCKENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    #[must_use]
    pub fn dpdwakeupenable(&mut self) -> DPDWAKEUPENABLE_W<2> {
        DPDWAKEUPENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    #[must_use]
    pub fn osc32kpd(&mut self) -> OSC32KPD_W<3> {
        OSC32KPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ostimer](index.html) module"]
pub struct OSTIMER_SPEC;
impl crate::RegisterSpec for OSTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ostimer::R](R) reader structure"]
impl crate::Readable for OSTIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ostimer::W](W) writer structure"]
impl crate::Writable for OSTIMER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSTIMER to value 0x08"]
impl crate::Resettable for OSTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}

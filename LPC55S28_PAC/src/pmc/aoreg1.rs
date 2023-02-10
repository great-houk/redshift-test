#[doc = "Register `AOREG1` reader"]
pub struct R(crate::R<AOREG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AOREG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AOREG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AOREG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AOREG1` writer"]
pub struct W(crate::W<AOREG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AOREG1_SPEC>;
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
impl From<crate::W<AOREG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AOREG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POR` reader - The last chip reset was caused by a Power On Reset."]
pub type POR_R = crate::BitReader<bool>;
#[doc = "Field `POR` writer - The last chip reset was caused by a Power On Reset."]
pub type POR_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOREG1_SPEC, bool, O>;
#[doc = "Field `PADRESET` reader - The last chip reset was caused by a Pin Reset."]
pub type PADRESET_R = crate::BitReader<bool>;
#[doc = "Field `PADRESET` writer - The last chip reset was caused by a Pin Reset."]
pub type PADRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOREG1_SPEC, bool, O>;
#[doc = "Field `BODRESET` reader - The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
pub type BODRESET_R = crate::BitReader<bool>;
#[doc = "Field `BODRESET` writer - The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
pub type BODRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOREG1_SPEC, bool, O>;
#[doc = "Field `SYSTEMRESET` reader - The last chip reset was caused by a System Reset requested by the ARM CPU."]
pub type SYSTEMRESET_R = crate::BitReader<bool>;
#[doc = "Field `SYSTEMRESET` writer - The last chip reset was caused by a System Reset requested by the ARM CPU."]
pub type SYSTEMRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOREG1_SPEC, bool, O>;
#[doc = "Field `WDTRESET` reader - The last chip reset was caused by the Watchdog Timer."]
pub type WDTRESET_R = crate::BitReader<bool>;
#[doc = "Field `WDTRESET` writer - The last chip reset was caused by the Watchdog Timer."]
pub type WDTRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOREG1_SPEC, bool, O>;
#[doc = "Field `SWRRESET` reader - The last chip reset was caused by a Software event."]
pub type SWRRESET_R = crate::BitReader<bool>;
#[doc = "Field `SWRRESET` writer - The last chip reset was caused by a Software event."]
pub type SWRRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOREG1_SPEC, bool, O>;
#[doc = "Field `DPDRESET_WAKEUPIO` reader - The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
pub type DPDRESET_WAKEUPIO_R = crate::BitReader<bool>;
#[doc = "Field `DPDRESET_WAKEUPIO` writer - The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
pub type DPDRESET_WAKEUPIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOREG1_SPEC, bool, O>;
#[doc = "Field `DPDRESET_RTC` reader - The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
pub type DPDRESET_RTC_R = crate::BitReader<bool>;
#[doc = "Field `DPDRESET_RTC` writer - The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
pub type DPDRESET_RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOREG1_SPEC, bool, O>;
#[doc = "Field `DPDRESET_OSTIMER` reader - The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
pub type DPDRESET_OSTIMER_R = crate::BitReader<bool>;
#[doc = "Field `DPDRESET_OSTIMER` writer - The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
pub type DPDRESET_OSTIMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, AOREG1_SPEC, bool, O>;
#[doc = "Field `BOOTERRORCOUNTER` reader - ROM Boot Fatal Error Counter."]
pub type BOOTERRORCOUNTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOOTERRORCOUNTER` writer - ROM Boot Fatal Error Counter."]
pub type BOOTERRORCOUNTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AOREG1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 4 - The last chip reset was caused by a Power On Reset."]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The last chip reset was caused by a Pin Reset."]
    #[inline(always)]
    pub fn padreset(&self) -> PADRESET_R {
        PADRESET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
    #[inline(always)]
    pub fn bodreset(&self) -> BODRESET_R {
        BODRESET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The last chip reset was caused by a System Reset requested by the ARM CPU."]
    #[inline(always)]
    pub fn systemreset(&self) -> SYSTEMRESET_R {
        SYSTEMRESET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The last chip reset was caused by the Watchdog Timer."]
    #[inline(always)]
    pub fn wdtreset(&self) -> WDTRESET_R {
        WDTRESET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The last chip reset was caused by a Software event."]
    #[inline(always)]
    pub fn swrreset(&self) -> SWRRESET_R {
        SWRRESET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_wakeupio(&self) -> DPDRESET_WAKEUPIO_R {
        DPDRESET_WAKEUPIO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_rtc(&self) -> DPDRESET_RTC_R {
        DPDRESET_RTC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
    #[inline(always)]
    pub fn dpdreset_ostimer(&self) -> DPDRESET_OSTIMER_R {
        DPDRESET_OSTIMER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - ROM Boot Fatal Error Counter."]
    #[inline(always)]
    pub fn booterrorcounter(&self) -> BOOTERRORCOUNTER_R {
        BOOTERRORCOUNTER_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - The last chip reset was caused by a Power On Reset."]
    #[inline(always)]
    #[must_use]
    pub fn por(&mut self) -> POR_W<4> {
        POR_W::new(self)
    }
    #[doc = "Bit 5 - The last chip reset was caused by a Pin Reset."]
    #[inline(always)]
    #[must_use]
    pub fn padreset(&mut self) -> PADRESET_W<5> {
        PADRESET_W::new(self)
    }
    #[doc = "Bit 6 - The last chip reset was caused by a Brown Out Detector (BoD), either VBAT BoD or Core Logic BoD."]
    #[inline(always)]
    #[must_use]
    pub fn bodreset(&mut self) -> BODRESET_W<6> {
        BODRESET_W::new(self)
    }
    #[doc = "Bit 7 - The last chip reset was caused by a System Reset requested by the ARM CPU."]
    #[inline(always)]
    #[must_use]
    pub fn systemreset(&mut self) -> SYSTEMRESET_W<7> {
        SYSTEMRESET_W::new(self)
    }
    #[doc = "Bit 8 - The last chip reset was caused by the Watchdog Timer."]
    #[inline(always)]
    #[must_use]
    pub fn wdtreset(&mut self) -> WDTRESET_W<8> {
        WDTRESET_W::new(self)
    }
    #[doc = "Bit 9 - The last chip reset was caused by a Software event."]
    #[inline(always)]
    #[must_use]
    pub fn swrreset(&mut self) -> SWRRESET_W<9> {
        SWRRESET_W::new(self)
    }
    #[doc = "Bit 10 - The last chip reset was caused by a Wake-up I/O reset event during a Deep Power-Down mode."]
    #[inline(always)]
    #[must_use]
    pub fn dpdreset_wakeupio(&mut self) -> DPDRESET_WAKEUPIO_W<10> {
        DPDRESET_WAKEUPIO_W::new(self)
    }
    #[doc = "Bit 11 - The last chip reset was caused by an RTC (either RTC Alarm or RTC wake up) reset event during a Deep Power-Down mode."]
    #[inline(always)]
    #[must_use]
    pub fn dpdreset_rtc(&mut self) -> DPDRESET_RTC_W<11> {
        DPDRESET_RTC_W::new(self)
    }
    #[doc = "Bit 12 - The last chip reset was caused by an OS Event Timer reset event during a Deep Power-Down mode."]
    #[inline(always)]
    #[must_use]
    pub fn dpdreset_ostimer(&mut self) -> DPDRESET_OSTIMER_W<12> {
        DPDRESET_OSTIMER_W::new(self)
    }
    #[doc = "Bits 16:19 - ROM Boot Fatal Error Counter."]
    #[inline(always)]
    #[must_use]
    pub fn booterrorcounter(&mut self) -> BOOTERRORCOUNTER_W<16> {
        BOOTERRORCOUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General purpose always on domain data storage \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aoreg1](index.html) module"]
pub struct AOREG1_SPEC;
impl crate::RegisterSpec for AOREG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aoreg1::R](R) reader structure"]
impl crate::Readable for AOREG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aoreg1::W](W) writer structure"]
impl crate::Writable for AOREG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AOREG1 to value 0"]
impl crate::Resettable for AOREG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

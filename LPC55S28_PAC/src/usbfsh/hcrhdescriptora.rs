#[doc = "Register `HCRHDESCRIPTORA` reader"]
pub struct R(crate::R<HCRHDESCRIPTORA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHDESCRIPTORA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCRHDESCRIPTORA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCRHDESCRIPTORA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCRHDESCRIPTORA` writer"]
pub struct W(crate::W<HCRHDESCRIPTORA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHDESCRIPTORA_SPEC>;
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
impl From<crate::W<HCRHDESCRIPTORA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCRHDESCRIPTORA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDP` reader - NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
pub type NDP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NDP` writer - NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
pub type NDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCRHDESCRIPTORA_SPEC, u8, u8, 8, O>;
#[doc = "Field `PSM` reader - PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
pub type PSM_R = crate::BitReader<bool>;
#[doc = "Field `PSM` writer - PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
pub type PSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHDESCRIPTORA_SPEC, bool, O>;
#[doc = "Field `NPS` reader - NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
pub type NPS_R = crate::BitReader<bool>;
#[doc = "Field `NPS` writer - NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
pub type NPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHDESCRIPTORA_SPEC, bool, O>;
#[doc = "Field `DT` reader - DeviceType This bit specifies that the root hub is not a compound device."]
pub type DT_R = crate::BitReader<bool>;
#[doc = "Field `DT` writer - DeviceType This bit specifies that the root hub is not a compound device."]
pub type DT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHDESCRIPTORA_SPEC, bool, O>;
#[doc = "Field `OCPM` reader - OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
pub type OCPM_R = crate::BitReader<bool>;
#[doc = "Field `OCPM` writer - OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
pub type OCPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHDESCRIPTORA_SPEC, bool, O>;
#[doc = "Field `NOCP` reader - NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
pub type NOCP_R = crate::BitReader<bool>;
#[doc = "Field `NOCP` writer - NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
pub type NOCP_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHDESCRIPTORA_SPEC, bool, O>;
#[doc = "Field `POTPGT` reader - PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
pub type POTPGT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POTPGT` writer - PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
pub type POTPGT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HCRHDESCRIPTORA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
    #[inline(always)]
    pub fn ndp(&self) -> NDP_R {
        NDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
    #[inline(always)]
    pub fn psm(&self) -> PSM_R {
        PSM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
    #[inline(always)]
    pub fn nps(&self) -> NPS_R {
        NPS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DeviceType This bit specifies that the root hub is not a compound device."]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub fn ocpm(&self) -> OCPM_R {
        OCPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 24:31 - PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
    #[inline(always)]
    pub fn potpgt(&self) -> POTPGT_R {
        POTPGT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
    #[inline(always)]
    #[must_use]
    pub fn ndp(&mut self) -> NDP_W<0> {
        NDP_W::new(self)
    }
    #[doc = "Bit 8 - PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
    #[inline(always)]
    #[must_use]
    pub fn psm(&mut self) -> PSM_W<8> {
        PSM_W::new(self)
    }
    #[doc = "Bit 9 - NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
    #[inline(always)]
    #[must_use]
    pub fn nps(&mut self) -> NPS_W<9> {
        NPS_W::new(self)
    }
    #[doc = "Bit 10 - DeviceType This bit specifies that the root hub is not a compound device."]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<10> {
        DT_W::new(self)
    }
    #[doc = "Bit 11 - OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    #[must_use]
    pub fn ocpm(&mut self) -> OCPM_W<11> {
        OCPM_W::new(self)
    }
    #[doc = "Bit 12 - NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    #[must_use]
    pub fn nocp(&mut self) -> NOCP_W<12> {
        NOCP_W::new(self)
    }
    #[doc = "Bits 24:31 - PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
    #[inline(always)]
    #[must_use]
    pub fn potpgt(&mut self) -> POTPGT_W<24> {
        POTPGT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "First of the two registers which describes the characteristics of the root hub\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhdescriptora](index.html) module"]
pub struct HCRHDESCRIPTORA_SPEC;
impl crate::RegisterSpec for HCRHDESCRIPTORA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrhdescriptora::R](R) reader structure"]
impl crate::Readable for HCRHDESCRIPTORA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcrhdescriptora::W](W) writer structure"]
impl crate::Writable for HCRHDESCRIPTORA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCRHDESCRIPTORA to value 0xff00_0902"]
impl crate::Resettable for HCRHDESCRIPTORA_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00_0902;
}

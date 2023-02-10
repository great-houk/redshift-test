#[doc = "Register `HCRHSTATUS` reader"]
pub struct R(crate::R<HCRHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCRHSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCRHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCRHSTATUS` writer"]
pub struct W(crate::W<HCRHSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHSTATUS_SPEC>;
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
impl From<crate::W<HCRHSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCRHSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPS` reader - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
pub type LPS_R = crate::BitReader<bool>;
#[doc = "Field `LPS` writer - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
pub type LPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHSTATUS_SPEC, bool, O>;
#[doc = "Field `OCI` reader - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
pub type OCI_R = crate::BitReader<bool>;
#[doc = "Field `OCI` writer - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
pub type OCI_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHSTATUS_SPEC, bool, O>;
#[doc = "Field `DRWE` reader - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
pub type DRWE_R = crate::BitReader<bool>;
#[doc = "Field `DRWE` writer - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
pub type DRWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHSTATUS_SPEC, bool, O>;
#[doc = "Field `LPSC` reader - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
pub type LPSC_R = crate::BitReader<bool>;
#[doc = "Field `LPSC` writer - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
pub type LPSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHSTATUS_SPEC, bool, O>;
#[doc = "Field `OCIC` reader - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
pub type OCIC_R = crate::BitReader<bool>;
#[doc = "Field `OCIC` writer - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
pub type OCIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHSTATUS_SPEC, bool, O>;
#[doc = "Field `CRWE` reader - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
pub type CRWE_R = crate::BitReader<bool>;
#[doc = "Field `CRWE` writer - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
pub type CRWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHSTATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[inline(always)]
    pub fn lps(&self) -> LPS_R {
        LPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[inline(always)]
    pub fn oci(&self) -> OCI_R {
        OCI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[inline(always)]
    pub fn drwe(&self) -> DRWE_R {
        DRWE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[inline(always)]
    pub fn lpsc(&self) -> LPSC_R {
        LPSC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[inline(always)]
    pub fn ocic(&self) -> OCIC_R {
        OCIC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[inline(always)]
    pub fn crwe(&self) -> CRWE_R {
        CRWE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn lps(&mut self) -> LPS_W<0> {
        LPS_W::new(self)
    }
    #[doc = "Bit 1 - OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn oci(&mut self) -> OCI_W<1> {
        OCI_W::new(self)
    }
    #[doc = "Bit 15 - (read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn drwe(&mut self) -> DRWE_W<15> {
        DRWE_W::new(self)
    }
    #[doc = "Bit 16 - (read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[inline(always)]
    #[must_use]
    pub fn lpsc(&mut self) -> LPSC_W<16> {
        LPSC_W::new(self)
    }
    #[doc = "Bit 17 - OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[inline(always)]
    #[must_use]
    pub fn ocic(&mut self) -> OCIC_W<17> {
        OCIC_W::new(self)
    }
    #[doc = "Bit 31 - (write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[inline(always)]
    #[must_use]
    pub fn crwe(&mut self) -> CRWE_W<31> {
        CRWE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is divided into two parts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhstatus](index.html) module"]
pub struct HCRHSTATUS_SPEC;
impl crate::RegisterSpec for HCRHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrhstatus::R](R) reader structure"]
impl crate::Readable for HCRHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcrhstatus::W](W) writer structure"]
impl crate::Writable for HCRHSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCRHSTATUS to value 0"]
impl crate::Resettable for HCRHSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `HCRHPORTSTATUS` reader"]
pub struct R(crate::R<HCRHPORTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHPORTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCRHPORTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCRHPORTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCRHPORTSTATUS` writer"]
pub struct W(crate::W<HCRHPORTSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHPORTSTATUS_SPEC>;
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
impl From<crate::W<HCRHPORTSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCRHPORTSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCS` reader - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
pub type CCS_R = crate::BitReader<bool>;
#[doc = "Field `CCS` writer - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
pub type CCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHPORTSTATUS_SPEC, bool, O>;
#[doc = "Field `PES` reader - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
pub type PES_R = crate::BitReader<bool>;
#[doc = "Field `PES` writer - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
pub type PES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHPORTSTATUS_SPEC, bool, O>;
#[doc = "Field `PSS` reader - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
pub type PSS_R = crate::BitReader<bool>;
#[doc = "Field `PSS` writer - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
pub type PSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHPORTSTATUS_SPEC, bool, O>;
#[doc = "Field `POCI` reader - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
pub type POCI_R = crate::BitReader<bool>;
#[doc = "Field `POCI` writer - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
pub type POCI_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHPORTSTATUS_SPEC, bool, O>;
#[doc = "Field `PRS` reader - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
pub type PRS_R = crate::BitReader<bool>;
#[doc = "Field `PRS` writer - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
pub type PRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHPORTSTATUS_SPEC, bool, O>;
#[doc = "Field `PPS` reader - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
pub type PPS_R = crate::BitReader<bool>;
#[doc = "Field `PPS` writer - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
pub type PPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHPORTSTATUS_SPEC, bool, O>;
#[doc = "Field `LSDA` reader - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
pub type LSDA_R = crate::BitReader<bool>;
#[doc = "Field `LSDA` writer - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
pub type LSDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHPORTSTATUS_SPEC, bool, O>;
#[doc = "Field `CSC` reader - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
pub type CSC_R = crate::BitReader<bool>;
#[doc = "Field `CSC` writer - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
pub type CSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHPORTSTATUS_SPEC, bool, O>;
#[doc = "Field `PESC` reader - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
pub type PESC_R = crate::BitReader<bool>;
#[doc = "Field `PESC` writer - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
pub type PESC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHPORTSTATUS_SPEC, bool, O>;
#[doc = "Field `PSSC` reader - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
pub type PSSC_R = crate::BitReader<bool>;
#[doc = "Field `PSSC` writer - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
pub type PSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHPORTSTATUS_SPEC, bool, O>;
#[doc = "Field `OCIC` reader - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
pub type OCIC_R = crate::BitReader<bool>;
#[doc = "Field `OCIC` writer - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
pub type OCIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHPORTSTATUS_SPEC, bool, O>;
#[doc = "Field `PRSC` reader - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
pub type PRSC_R = crate::BitReader<bool>;
#[doc = "Field `PRSC` writer - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
pub type PRSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCRHPORTSTATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
    #[inline(always)]
    pub fn pss(&self) -> PSS_R {
        PSS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn poci(&self) -> POCI_R {
        POCI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
    #[inline(always)]
    pub fn pps(&self) -> PPS_R {
        PPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
    #[inline(always)]
    pub fn lsda(&self) -> LSDA_R {
        LSDA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
    #[inline(always)]
    pub fn pesc(&self) -> PESC_R {
        PESC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub fn ocic(&self) -> OCIC_R {
        OCIC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
    #[inline(always)]
    pub fn prsc(&self) -> PRSC_R {
        PRSC_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
    #[inline(always)]
    #[must_use]
    pub fn ccs(&mut self) -> CCS_W<0> {
        CCS_W::new(self)
    }
    #[doc = "Bit 1 - (read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
    #[inline(always)]
    #[must_use]
    pub fn pes(&mut self) -> PES_W<1> {
        PES_W::new(self)
    }
    #[doc = "Bit 2 - (read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
    #[inline(always)]
    #[must_use]
    pub fn pss(&mut self) -> PSS_W<2> {
        PSS_W::new(self)
    }
    #[doc = "Bit 3 - (read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    #[must_use]
    pub fn poci(&mut self) -> POCI_W<3> {
        POCI_W::new(self)
    }
    #[doc = "Bit 4 - (read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PRS_W<4> {
        PRS_W::new(self)
    }
    #[doc = "Bit 8 - (read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
    #[inline(always)]
    #[must_use]
    pub fn pps(&mut self) -> PPS_W<8> {
        PPS_W::new(self)
    }
    #[doc = "Bit 9 - (read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
    #[inline(always)]
    #[must_use]
    pub fn lsda(&mut self) -> LSDA_W<9> {
        LSDA_W::new(self)
    }
    #[doc = "Bit 16 - ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
    #[inline(always)]
    #[must_use]
    pub fn csc(&mut self) -> CSC_W<16> {
        CSC_W::new(self)
    }
    #[doc = "Bit 17 - PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn pesc(&mut self) -> PESC_W<17> {
        PESC_W::new(self)
    }
    #[doc = "Bit 18 - PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
    #[inline(always)]
    #[must_use]
    pub fn pssc(&mut self) -> PSSC_W<18> {
        PSSC_W::new(self)
    }
    #[doc = "Bit 19 - PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    #[must_use]
    pub fn ocic(&mut self) -> OCIC_W<19> {
        OCIC_W::new(self)
    }
    #[doc = "Bit 20 - PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
    #[inline(always)]
    #[must_use]
    pub fn prsc(&mut self) -> PRSC_W<20> {
        PRSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls and reports the port events on a per-port basis\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhportstatus](index.html) module"]
pub struct HCRHPORTSTATUS_SPEC;
impl crate::RegisterSpec for HCRHPORTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrhportstatus::R](R) reader structure"]
impl crate::Readable for HCRHPORTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcrhportstatus::W](W) writer structure"]
impl crate::Writable for HCRHPORTSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCRHPORTSTATUS to value 0"]
impl crate::Resettable for HCRHPORTSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

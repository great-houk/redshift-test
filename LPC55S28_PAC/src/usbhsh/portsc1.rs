#[doc = "Register `PORTSC1` reader"]
pub struct R(crate::R<PORTSC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTSC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTSC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTSC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTSC1` writer"]
pub struct W(crate::W<PORTSC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTSC1_SPEC>;
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
impl From<crate::W<PORTSC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTSC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCS` reader - Current Connect Status: Logic 1 indicates a device is present on the port."]
pub type CCS_R = crate::BitReader<bool>;
#[doc = "Field `CCS` writer - Current Connect Status: Logic 1 indicates a device is present on the port."]
pub type CCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `CSC` reader - Connect Status Change: Logic 1 means that the value of CCS has changed."]
pub type CSC_R = crate::BitReader<bool>;
#[doc = "Field `CSC` writer - Connect Status Change: Logic 1 means that the value of CCS has changed."]
pub type CSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `PED` reader - Port Enabled/Disabled."]
pub type PED_R = crate::BitReader<bool>;
#[doc = "Field `PED` writer - Port Enabled/Disabled."]
pub type PED_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `PEDC` reader - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
pub type PEDC_R = crate::BitReader<bool>;
#[doc = "Field `PEDC` writer - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
pub type PEDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `OCA` reader - Over-current active: Logic 1 means that this port has an over-current condition."]
pub type OCA_R = crate::BitReader<bool>;
#[doc = "Field `OCA` writer - Over-current active: Logic 1 means that this port has an over-current condition."]
pub type OCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `OCC` reader - Over-current change: Logic 1 means that the value of OCA has changed."]
pub type OCC_R = crate::BitReader<bool>;
#[doc = "Field `OCC` writer - Over-current change: Logic 1 means that the value of OCA has changed."]
pub type OCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `FPR` reader - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
pub type FPR_R = crate::BitReader<bool>;
#[doc = "Field `FPR` writer - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
pub type FPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `SUSP` reader - Suspend: Logic 1 means port is in the suspend state."]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `SUSP` writer - Suspend: Logic 1 means port is in the suspend state."]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `PR` reader - Port Reset: Logic 1 means the port is in the reset state."]
pub type PR_R = crate::BitReader<bool>;
#[doc = "Field `PR` writer - Port Reset: Logic 1 means the port is in the reset state."]
pub type PR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `LS` reader - Line Status: This field reflects the current logical levels of the DP (bit 11) and DM (bit 10) signal lines."]
pub type LS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PP` reader - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
pub type PP_R = crate::BitReader<bool>;
#[doc = "Field `PP` writer - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
pub type PP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
#[doc = "Field `PIC` reader - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
pub type PIC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIC` writer - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
pub type PIC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORTSC1_SPEC, u8, u8, 2, O>;
#[doc = "Field `PTC` reader - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
pub type PTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PTC` writer - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
pub type PTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORTSC1_SPEC, u8, u8, 4, O>;
#[doc = "Field `PSPD` reader - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
pub type PSPD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSPD` writer - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
pub type PSPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PORTSC1_SPEC, u8, u8, 2, O>;
#[doc = "Field `WOO` reader - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
pub type WOO_R = crate::BitReader<bool>;
#[doc = "Field `WOO` writer - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
pub type WOO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTSC1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Current Connect Status: Logic 1 indicates a device is present on the port."]
    #[inline(always)]
    pub fn ccs(&self) -> CCS_R {
        CCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Connect Status Change: Logic 1 means that the value of CCS has changed."]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled."]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
    #[inline(always)]
    pub fn pedc(&self) -> PEDC_R {
        PEDC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Over-current active: Logic 1 means that this port has an over-current condition."]
    #[inline(always)]
    pub fn oca(&self) -> OCA_R {
        OCA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Over-current change: Logic 1 means that the value of OCA has changed."]
    #[inline(always)]
    pub fn occ(&self) -> OCC_R {
        OCC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
    #[inline(always)]
    pub fn fpr(&self) -> FPR_R {
        FPR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspend: Logic 1 means port is in the suspend state."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset: Logic 1 means the port is in the reset state."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Line Status: This field reflects the current logical levels of the DP (bit 11) and DM (bit 10) signal lines."]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
    #[inline(always)]
    pub fn pp(&self) -> PP_R {
        PP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
    #[inline(always)]
    pub fn pic(&self) -> PIC_R {
        PIC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
    #[inline(always)]
    pub fn ptc(&self) -> PTC_R {
        PTC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
    #[inline(always)]
    pub fn woo(&self) -> WOO_R {
        WOO_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Current Connect Status: Logic 1 indicates a device is present on the port."]
    #[inline(always)]
    #[must_use]
    pub fn ccs(&mut self) -> CCS_W<0> {
        CCS_W::new(self)
    }
    #[doc = "Bit 1 - Connect Status Change: Logic 1 means that the value of CCS has changed."]
    #[inline(always)]
    #[must_use]
    pub fn csc(&mut self) -> CSC_W<1> {
        CSC_W::new(self)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PED_W<2> {
        PED_W::new(self)
    }
    #[doc = "Bit 3 - Port Enabled/Disabled Change: Logic 1 means that the value of PED has changed."]
    #[inline(always)]
    #[must_use]
    pub fn pedc(&mut self) -> PEDC_W<3> {
        PEDC_W::new(self)
    }
    #[doc = "Bit 4 - Over-current active: Logic 1 means that this port has an over-current condition."]
    #[inline(always)]
    #[must_use]
    pub fn oca(&mut self) -> OCA_W<4> {
        OCA_W::new(self)
    }
    #[doc = "Bit 5 - Over-current change: Logic 1 means that the value of OCA has changed."]
    #[inline(always)]
    #[must_use]
    pub fn occ(&mut self) -> OCC_W<5> {
        OCC_W::new(self)
    }
    #[doc = "Bit 6 - Force Port Resume: Logic 1 means resume (K-state) detected or driven on the port."]
    #[inline(always)]
    #[must_use]
    pub fn fpr(&mut self) -> FPR_W<6> {
        FPR_W::new(self)
    }
    #[doc = "Bit 7 - Suspend: Logic 1 means port is in the suspend state."]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<7> {
        SUSP_W::new(self)
    }
    #[doc = "Bit 8 - Port Reset: Logic 1 means the port is in the reset state."]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<8> {
        PR_W::new(self)
    }
    #[doc = "Bit 12 - Port Power: The function of this bit depends on the value of the Port Power Control (PPC) bit in the HCSPARAMS register."]
    #[inline(always)]
    #[must_use]
    pub fn pp(&mut self) -> PP_W<12> {
        PP_W::new(self)
    }
    #[doc = "Bits 14:15 - Port Indicator Control : Writing to this field has no effect if the P_INDICATOR bit in the HCSPARAMS register is logic 0."]
    #[inline(always)]
    #[must_use]
    pub fn pic(&mut self) -> PIC_W<14> {
        PIC_W::new(self)
    }
    #[doc = "Bits 16:19 - Port Test Control: A non-zero value indicates that the port is operating in the test mode as indicated by the value."]
    #[inline(always)]
    #[must_use]
    pub fn ptc(&mut self) -> PTC_W<16> {
        PTC_W::new(self)
    }
    #[doc = "Bits 20:21 - Port Speed: 00b: Low-speed 01b: Full-speed 10b: High-speed 11b: Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn pspd(&mut self) -> PSPD_W<20> {
        PSPD_W::new(self)
    }
    #[doc = "Bit 22 - Wake on overcurrent enable: Writing this bit to a one enables the port to be sensitive to overcurrent conditions as wake-up events."]
    #[inline(always)]
    #[must_use]
    pub fn woo(&mut self) -> WOO_W<22> {
        WOO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Status and Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portsc1](index.html) module"]
pub struct PORTSC1_SPEC;
impl crate::RegisterSpec for PORTSC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portsc1::R](R) reader structure"]
impl crate::Readable for PORTSC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portsc1::W](W) writer structure"]
impl crate::Writable for PORTSC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTSC1 to value 0"]
impl crate::Resettable for PORTSC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

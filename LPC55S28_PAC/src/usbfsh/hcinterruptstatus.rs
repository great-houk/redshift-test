#[doc = "Register `HCINTERRUPTSTATUS` reader"]
pub struct R(crate::R<HCINTERRUPTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINTERRUPTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINTERRUPTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINTERRUPTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINTERRUPTSTATUS` writer"]
pub struct W(crate::W<HCINTERRUPTSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINTERRUPTSTATUS_SPEC>;
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
impl From<crate::W<HCINTERRUPTSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINTERRUPTSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SO` reader - SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
pub type SO_R = crate::BitReader<bool>;
#[doc = "Field `SO` writer - SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
pub type SO_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTSTATUS_SPEC, bool, O>;
#[doc = "Field `WDH` reader - WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
pub type WDH_R = crate::BitReader<bool>;
#[doc = "Field `WDH` writer - WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
pub type WDH_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTSTATUS_SPEC, bool, O>;
#[doc = "Field `SF` reader - StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
pub type SF_R = crate::BitReader<bool>;
#[doc = "Field `SF` writer - StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
pub type SF_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTSTATUS_SPEC, bool, O>;
#[doc = "Field `RD` reader - ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
pub type RD_R = crate::BitReader<bool>;
#[doc = "Field `RD` writer - ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
pub type RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTSTATUS_SPEC, bool, O>;
#[doc = "Field `UE` reader - UnrecoverableError This bit is set when HC detects a system error not related to USB."]
pub type UE_R = crate::BitReader<bool>;
#[doc = "Field `UE` writer - UnrecoverableError This bit is set when HC detects a system error not related to USB."]
pub type UE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTSTATUS_SPEC, bool, O>;
#[doc = "Field `FNO` reader - FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
pub type FNO_R = crate::BitReader<bool>;
#[doc = "Field `FNO` writer - FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
pub type FNO_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTSTATUS_SPEC, bool, O>;
#[doc = "Field `RHSC` reader - RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\]
has changed."]
pub type RHSC_R = crate::BitReader<bool>;
#[doc = "Field `RHSC` writer - RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\]
has changed."]
pub type RHSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTSTATUS_SPEC, bool, O>;
#[doc = "Field `OC` reader - OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
pub type OC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OC` writer - OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
pub type OC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HCINTERRUPTSTATUS_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bit 0 - SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn so(&self) -> SO_R {
        SO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
    #[inline(always)]
    pub fn wdh(&self) -> WDH_R {
        WDH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub fn sf(&self) -> SF_R {
        SF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UnrecoverableError This bit is set when HC detects a system error not related to USB."]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
    #[inline(always)]
    pub fn fno(&self) -> FNO_R {
        FNO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\]
has changed."]
    #[inline(always)]
    pub fn rhsc(&self) -> RHSC_R {
        RHSC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:31 - OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
    #[inline(always)]
    pub fn oc(&self) -> OC_R {
        OC_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
    #[inline(always)]
    #[must_use]
    pub fn so(&mut self) -> SO_W<0> {
        SO_W::new(self)
    }
    #[doc = "Bit 1 - WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
    #[inline(always)]
    #[must_use]
    pub fn wdh(&mut self) -> WDH_W<1> {
        WDH_W::new(self)
    }
    #[doc = "Bit 2 - StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
    #[inline(always)]
    #[must_use]
    pub fn sf(&mut self) -> SF_W<2> {
        SF_W::new(self)
    }
    #[doc = "Bit 3 - ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<3> {
        RD_W::new(self)
    }
    #[doc = "Bit 4 - UnrecoverableError This bit is set when HC detects a system error not related to USB."]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<4> {
        UE_W::new(self)
    }
    #[doc = "Bit 5 - FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
    #[inline(always)]
    #[must_use]
    pub fn fno(&mut self) -> FNO_W<5> {
        FNO_W::new(self)
    }
    #[doc = "Bit 6 - RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\]
has changed."]
    #[inline(always)]
    #[must_use]
    pub fn rhsc(&mut self) -> RHSC_W<6> {
        RHSC_W::new(self)
    }
    #[doc = "Bits 10:31 - OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
    #[inline(always)]
    #[must_use]
    pub fn oc(&mut self) -> OC_W<10> {
        OC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicates the status on various events that cause hardware interrupts by setting the appropriate bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcinterruptstatus](index.html) module"]
pub struct HCINTERRUPTSTATUS_SPEC;
impl crate::RegisterSpec for HCINTERRUPTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcinterruptstatus::R](R) reader structure"]
impl crate::Readable for HCINTERRUPTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcinterruptstatus::W](W) writer structure"]
impl crate::Writable for HCINTERRUPTSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINTERRUPTSTATUS to value 0"]
impl crate::Resettable for HCINTERRUPTSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

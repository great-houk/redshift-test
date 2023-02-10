#[doc = "Register `HCCOMMANDSTATUS` reader"]
pub struct R(crate::R<HCCOMMANDSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCOMMANDSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCOMMANDSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCOMMANDSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCOMMANDSTATUS` writer"]
pub struct W(crate::W<HCCOMMANDSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCOMMANDSTATUS_SPEC>;
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
impl From<crate::W<HCCOMMANDSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCOMMANDSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HCR` reader - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
pub type HCR_R = crate::BitReader<bool>;
#[doc = "Field `HCR` writer - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
pub type HCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCOMMANDSTATUS_SPEC, bool, O>;
#[doc = "Field `CLF` reader - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
pub type CLF_R = crate::BitReader<bool>;
#[doc = "Field `CLF` writer - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
pub type CLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCOMMANDSTATUS_SPEC, bool, O>;
#[doc = "Field `BLF` reader - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
pub type BLF_R = crate::BitReader<bool>;
#[doc = "Field `BLF` writer - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
pub type BLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCOMMANDSTATUS_SPEC, bool, O>;
#[doc = "Field `OCR` reader - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
pub type OCR_R = crate::BitReader<bool>;
#[doc = "Field `OCR` writer - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
pub type OCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCOMMANDSTATUS_SPEC, bool, O>;
#[doc = "Field `SOC` reader - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
pub type SOC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOC` writer - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
pub type SOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCOMMANDSTATUS_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
    #[inline(always)]
    pub fn hcr(&self) -> HCR_R {
        HCR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
    #[inline(always)]
    pub fn clf(&self) -> CLF_R {
        CLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
    #[inline(always)]
    pub fn blf(&self) -> BLF_R {
        BLF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
    #[inline(always)]
    pub fn ocr(&self) -> OCR_R {
        OCR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7 - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
    #[inline(always)]
    pub fn soc(&self) -> SOC_R {
        SOC_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
    #[inline(always)]
    #[must_use]
    pub fn hcr(&mut self) -> HCR_W<0> {
        HCR_W::new(self)
    }
    #[doc = "Bit 1 - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
    #[inline(always)]
    #[must_use]
    pub fn clf(&mut self) -> CLF_W<1> {
        CLF_W::new(self)
    }
    #[doc = "Bit 2 - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
    #[inline(always)]
    #[must_use]
    pub fn blf(&mut self) -> BLF_W<2> {
        BLF_W::new(self)
    }
    #[doc = "Bit 3 - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
    #[inline(always)]
    #[must_use]
    pub fn ocr(&mut self) -> OCR_W<3> {
        OCR_W::new(self)
    }
    #[doc = "Bits 6:7 - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
    #[inline(always)]
    #[must_use]
    pub fn soc(&mut self) -> SOC_W<6> {
        SOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to receive the commands from the Host Controller Driver (HCD)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccommandstatus](index.html) module"]
pub struct HCCOMMANDSTATUS_SPEC;
impl crate::RegisterSpec for HCCOMMANDSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hccommandstatus::R](R) reader structure"]
impl crate::Readable for HCCOMMANDSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hccommandstatus::W](W) writer structure"]
impl crate::Writable for HCCOMMANDSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCCOMMANDSTATUS to value 0"]
impl crate::Resettable for HCCOMMANDSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

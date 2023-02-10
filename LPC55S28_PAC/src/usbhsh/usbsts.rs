#[doc = "Register `USBSTS` reader"]
pub struct R(crate::R<USBSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBSTS` writer"]
pub struct W(crate::W<USBSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBSTS_SPEC>;
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
impl From<crate::W<USBSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCD` reader - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
pub type PCD_R = crate::BitReader<bool>;
#[doc = "Field `PCD` writer - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
pub type PCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `FLR` reader - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
pub type FLR_R = crate::BitReader<bool>;
#[doc = "Field `FLR` writer - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
pub type FLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `ATL_IRQ` reader - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
pub type ATL_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `ATL_IRQ` writer - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
pub type ATL_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `ISO_IRQ` reader - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
pub type ISO_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `ISO_IRQ` writer - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
pub type ISO_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `INT_IRQ` reader - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
pub type INT_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `INT_IRQ` writer - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
pub type INT_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `SOF_IRQ` reader - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
pub type SOF_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `SOF_IRQ` writer - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
pub type SOF_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
    #[inline(always)]
    pub fn pcd(&self) -> PCD_R {
        PCD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[inline(always)]
    pub fn flr(&self) -> FLR_R {
        FLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn atl_irq(&self) -> ATL_IRQ_R {
        ATL_IRQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn iso_irq(&self) -> ISO_IRQ_R {
        ISO_IRQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[inline(always)]
    pub fn int_irq(&self) -> INT_IRQ_R {
        INT_IRQ_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[inline(always)]
    pub fn sof_irq(&self) -> SOF_IRQ_R {
        SOF_IRQ_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Port Change Detect: The host controller sets this bit to logic 1 when any port has a change bit transition from a 0 to a one or a Force Port Resume bit transition from a 0 to a 1 as a result of a J-K transition detected on a suspended port."]
    #[inline(always)]
    #[must_use]
    pub fn pcd(&mut self) -> PCD_W<2> {
        PCD_W::new(self)
    }
    #[doc = "Bit 3 - Frame List Rollover: The host controller sets this bit to logic 1 when the frame list index rolls over its maximum value to 0."]
    #[inline(always)]
    #[must_use]
    pub fn flr(&mut self) -> FLR_W<3> {
        FLR_W::new(self)
    }
    #[doc = "Bit 16 - ATL IRQ: Indicates that an ATL PTD (with I-bit set) was completed."]
    #[inline(always)]
    #[must_use]
    pub fn atl_irq(&mut self) -> ATL_IRQ_W<16> {
        ATL_IRQ_W::new(self)
    }
    #[doc = "Bit 17 - ISO IRQ: Indicates that an ISO PTD (with I-bit set) was completed."]
    #[inline(always)]
    #[must_use]
    pub fn iso_irq(&mut self) -> ISO_IRQ_W<17> {
        ISO_IRQ_W::new(self)
    }
    #[doc = "Bit 18 - INT IRQ: Indicates that an INT PTD (with I-bit set) was completed."]
    #[inline(always)]
    #[must_use]
    pub fn int_irq(&mut self) -> INT_IRQ_W<18> {
        INT_IRQ_W::new(self)
    }
    #[doc = "Bit 19 - SOF interrupt: Every time when the host sends a Start of Frame token on the USB bus, this bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn sof_irq(&mut self) -> SOF_IRQ_W<19> {
        SOF_IRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbsts](index.html) module"]
pub struct USBSTS_SPEC;
impl crate::RegisterSpec for USBSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbsts::R](R) reader structure"]
impl crate::Readable for USBSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbsts::W](W) writer structure"]
impl crate::Writable for USBSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBSTS to value 0"]
impl crate::Resettable for USBSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

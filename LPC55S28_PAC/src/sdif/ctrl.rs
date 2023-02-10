#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONTROLLER_RESET` reader - Controller reset."]
pub type CONTROLLER_RESET_R = crate::BitReader<bool>;
#[doc = "Field `CONTROLLER_RESET` writer - Controller reset."]
pub type CONTROLLER_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FIFO_RESET` reader - Fifo reset."]
pub type FIFO_RESET_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_RESET` writer - Fifo reset."]
pub type FIFO_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DMA_RESET` reader - DMA reset."]
pub type DMA_RESET_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RESET` writer - DMA reset."]
pub type DMA_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `INT_ENABLE` reader - Global interrupt enable/disable bit."]
pub type INT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `INT_ENABLE` writer - Global interrupt enable/disable bit."]
pub type INT_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `READ_WAIT` reader - Read/wait."]
pub type READ_WAIT_R = crate::BitReader<bool>;
#[doc = "Field `READ_WAIT` writer - Read/wait."]
pub type READ_WAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SEND_IRQ_RESPONSE` reader - Send irq response."]
pub type SEND_IRQ_RESPONSE_R = crate::BitReader<bool>;
#[doc = "Field `SEND_IRQ_RESPONSE` writer - Send irq response."]
pub type SEND_IRQ_RESPONSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ABORT_READ_DATA` reader - Abort read data."]
pub type ABORT_READ_DATA_R = crate::BitReader<bool>;
#[doc = "Field `ABORT_READ_DATA` writer - Abort read data."]
pub type ABORT_READ_DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SEND_CCSD` reader - Send ccsd."]
pub type SEND_CCSD_R = crate::BitReader<bool>;
#[doc = "Field `SEND_CCSD` writer - Send ccsd."]
pub type SEND_CCSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SEND_AUTO_STOP_CCSD` reader - Send auto stop ccsd."]
pub type SEND_AUTO_STOP_CCSD_R = crate::BitReader<bool>;
#[doc = "Field `SEND_AUTO_STOP_CCSD` writer - Send auto stop ccsd."]
pub type SEND_AUTO_STOP_CCSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CEATA_DEVICE_INTERRUPT_STATUS` reader - CEATA device interrupt status."]
pub type CEATA_DEVICE_INTERRUPT_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `CEATA_DEVICE_INTERRUPT_STATUS` writer - CEATA device interrupt status."]
pub type CEATA_DEVICE_INTERRUPT_STATUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CARD_VOLTAGE_A0` reader - Controls the state of the SD_VOLT0 pin."]
pub type CARD_VOLTAGE_A0_R = crate::BitReader<bool>;
#[doc = "Field `CARD_VOLTAGE_A0` writer - Controls the state of the SD_VOLT0 pin."]
pub type CARD_VOLTAGE_A0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CARD_VOLTAGE_A1` reader - Controls the state of the SD_VOLT1 pin."]
pub type CARD_VOLTAGE_A1_R = crate::BitReader<bool>;
#[doc = "Field `CARD_VOLTAGE_A1` writer - Controls the state of the SD_VOLT1 pin."]
pub type CARD_VOLTAGE_A1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CARD_VOLTAGE_A2` reader - Controls the state of the SD_VOLT2 pin."]
pub type CARD_VOLTAGE_A2_R = crate::BitReader<bool>;
#[doc = "Field `CARD_VOLTAGE_A2` writer - Controls the state of the SD_VOLT2 pin."]
pub type CARD_VOLTAGE_A2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `USE_INTERNAL_DMAC` reader - SD/MMC DMA use."]
pub type USE_INTERNAL_DMAC_R = crate::BitReader<bool>;
#[doc = "Field `USE_INTERNAL_DMAC` writer - SD/MMC DMA use."]
pub type USE_INTERNAL_DMAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Controller reset."]
    #[inline(always)]
    pub fn controller_reset(&self) -> CONTROLLER_RESET_R {
        CONTROLLER_RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fifo reset."]
    #[inline(always)]
    pub fn fifo_reset(&self) -> FIFO_RESET_R {
        FIFO_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA reset."]
    #[inline(always)]
    pub fn dma_reset(&self) -> DMA_RESET_R {
        DMA_RESET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit."]
    #[inline(always)]
    pub fn int_enable(&self) -> INT_ENABLE_R {
        INT_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Read/wait."]
    #[inline(always)]
    pub fn read_wait(&self) -> READ_WAIT_R {
        READ_WAIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Send irq response."]
    #[inline(always)]
    pub fn send_irq_response(&self) -> SEND_IRQ_RESPONSE_R {
        SEND_IRQ_RESPONSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Abort read data."]
    #[inline(always)]
    pub fn abort_read_data(&self) -> ABORT_READ_DATA_R {
        ABORT_READ_DATA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Send ccsd."]
    #[inline(always)]
    pub fn send_ccsd(&self) -> SEND_CCSD_R {
        SEND_CCSD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Send auto stop ccsd."]
    #[inline(always)]
    pub fn send_auto_stop_ccsd(&self) -> SEND_AUTO_STOP_CCSD_R {
        SEND_AUTO_STOP_CCSD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CEATA device interrupt status."]
    #[inline(always)]
    pub fn ceata_device_interrupt_status(&self) -> CEATA_DEVICE_INTERRUPT_STATUS_R {
        CEATA_DEVICE_INTERRUPT_STATUS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Controls the state of the SD_VOLT0 pin."]
    #[inline(always)]
    pub fn card_voltage_a0(&self) -> CARD_VOLTAGE_A0_R {
        CARD_VOLTAGE_A0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Controls the state of the SD_VOLT1 pin."]
    #[inline(always)]
    pub fn card_voltage_a1(&self) -> CARD_VOLTAGE_A1_R {
        CARD_VOLTAGE_A1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Controls the state of the SD_VOLT2 pin."]
    #[inline(always)]
    pub fn card_voltage_a2(&self) -> CARD_VOLTAGE_A2_R {
        CARD_VOLTAGE_A2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 25 - SD/MMC DMA use."]
    #[inline(always)]
    pub fn use_internal_dmac(&self) -> USE_INTERNAL_DMAC_R {
        USE_INTERNAL_DMAC_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controller reset."]
    #[inline(always)]
    #[must_use]
    pub fn controller_reset(&mut self) -> CONTROLLER_RESET_W<0> {
        CONTROLLER_RESET_W::new(self)
    }
    #[doc = "Bit 1 - Fifo reset."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_reset(&mut self) -> FIFO_RESET_W<1> {
        FIFO_RESET_W::new(self)
    }
    #[doc = "Bit 2 - DMA reset."]
    #[inline(always)]
    #[must_use]
    pub fn dma_reset(&mut self) -> DMA_RESET_W<2> {
        DMA_RESET_W::new(self)
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit."]
    #[inline(always)]
    #[must_use]
    pub fn int_enable(&mut self) -> INT_ENABLE_W<4> {
        INT_ENABLE_W::new(self)
    }
    #[doc = "Bit 6 - Read/wait."]
    #[inline(always)]
    #[must_use]
    pub fn read_wait(&mut self) -> READ_WAIT_W<6> {
        READ_WAIT_W::new(self)
    }
    #[doc = "Bit 7 - Send irq response."]
    #[inline(always)]
    #[must_use]
    pub fn send_irq_response(&mut self) -> SEND_IRQ_RESPONSE_W<7> {
        SEND_IRQ_RESPONSE_W::new(self)
    }
    #[doc = "Bit 8 - Abort read data."]
    #[inline(always)]
    #[must_use]
    pub fn abort_read_data(&mut self) -> ABORT_READ_DATA_W<8> {
        ABORT_READ_DATA_W::new(self)
    }
    #[doc = "Bit 9 - Send ccsd."]
    #[inline(always)]
    #[must_use]
    pub fn send_ccsd(&mut self) -> SEND_CCSD_W<9> {
        SEND_CCSD_W::new(self)
    }
    #[doc = "Bit 10 - Send auto stop ccsd."]
    #[inline(always)]
    #[must_use]
    pub fn send_auto_stop_ccsd(&mut self) -> SEND_AUTO_STOP_CCSD_W<10> {
        SEND_AUTO_STOP_CCSD_W::new(self)
    }
    #[doc = "Bit 11 - CEATA device interrupt status."]
    #[inline(always)]
    #[must_use]
    pub fn ceata_device_interrupt_status(&mut self) -> CEATA_DEVICE_INTERRUPT_STATUS_W<11> {
        CEATA_DEVICE_INTERRUPT_STATUS_W::new(self)
    }
    #[doc = "Bit 16 - Controls the state of the SD_VOLT0 pin."]
    #[inline(always)]
    #[must_use]
    pub fn card_voltage_a0(&mut self) -> CARD_VOLTAGE_A0_W<16> {
        CARD_VOLTAGE_A0_W::new(self)
    }
    #[doc = "Bit 17 - Controls the state of the SD_VOLT1 pin."]
    #[inline(always)]
    #[must_use]
    pub fn card_voltage_a1(&mut self) -> CARD_VOLTAGE_A1_W<17> {
        CARD_VOLTAGE_A1_W::new(self)
    }
    #[doc = "Bit 18 - Controls the state of the SD_VOLT2 pin."]
    #[inline(always)]
    #[must_use]
    pub fn card_voltage_a2(&mut self) -> CARD_VOLTAGE_A2_W<18> {
        CARD_VOLTAGE_A2_W::new(self)
    }
    #[doc = "Bit 25 - SD/MMC DMA use."]
    #[inline(always)]
    #[must_use]
    pub fn use_internal_dmac(&mut self) -> USE_INTERNAL_DMAC_W<25> {
        USE_INTERNAL_DMAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

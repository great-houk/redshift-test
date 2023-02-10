#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_RX_WATERMARK` reader - FIFO reached Receive watermark level; not qualified with data transfer."]
pub type FIFO_RX_WATERMARK_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_RX_WATERMARK` writer - FIFO reached Receive watermark level; not qualified with data transfer."]
pub type FIFO_RX_WATERMARK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `FIFO_TX_WATERMARK` reader - FIFO reached Transmit watermark level; not qualified with data transfer."]
pub type FIFO_TX_WATERMARK_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_TX_WATERMARK` writer - FIFO reached Transmit watermark level; not qualified with data transfer."]
pub type FIFO_TX_WATERMARK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `FIFO_EMPTY` reader - FIFO is empty status."]
pub type FIFO_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_EMPTY` writer - FIFO is empty status."]
pub type FIFO_EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `FIFO_FULL` reader - FIFO is full status."]
pub type FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_FULL` writer - FIFO is full status."]
pub type FIFO_FULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CMDFSMSTATES` reader - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
pub type CMDFSMSTATES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDFSMSTATES` writer - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
pub type CMDFSMSTATES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATA_3_STATUS` reader - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
pub type DATA_3_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `DATA_3_STATUS` writer - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
pub type DATA_3_STATUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `DATA_BUSY` reader - Inverted version of raw selected card_data\\[0\\]
0 - card data not busy 1 - card data busy."]
pub type DATA_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `DATA_BUSY` writer - Inverted version of raw selected card_data\\[0\\]
0 - card data not busy 1 - card data busy."]
pub type DATA_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `DATA_STATE_MC_BUSY` reader - Data transmit or receive state-machine is busy."]
pub type DATA_STATE_MC_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `DATA_STATE_MC_BUSY` writer - Data transmit or receive state-machine is busy."]
pub type DATA_STATE_MC_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `RESPONSE_INDEX` reader - Index of previous response, including any auto-stop sent by core."]
pub type RESPONSE_INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESPONSE_INDEX` writer - Index of previous response, including any auto-stop sent by core."]
pub type RESPONSE_INDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS_SPEC, u8, u8, 6, O>;
#[doc = "Field `FIFO_COUNT` reader - FIFO count - Number of filled locations in FIFO."]
pub type FIFO_COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FIFO_COUNT` writer - FIFO count - Number of filled locations in FIFO."]
pub type FIFO_COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS_SPEC, u16, u16, 13, O>;
#[doc = "Field `DMA_ACK` reader - DMA acknowledge signal state."]
pub type DMA_ACK_R = crate::BitReader<bool>;
#[doc = "Field `DMA_ACK` writer - DMA acknowledge signal state."]
pub type DMA_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `DMA_REQ` reader - DMA request signal state."]
pub type DMA_REQ_R = crate::BitReader<bool>;
#[doc = "Field `DMA_REQ` writer - DMA request signal state."]
pub type DMA_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FIFO reached Receive watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_rx_watermark(&self) -> FIFO_RX_WATERMARK_R {
        FIFO_RX_WATERMARK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_tx_watermark(&self) -> FIFO_TX_WATERMARK_R {
        FIFO_TX_WATERMARK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO is empty status."]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO is full status."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[inline(always)]
    pub fn cmdfsmstates(&self) -> CMDFSMSTATES_R {
        CMDFSMSTATES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[inline(always)]
    pub fn data_3_status(&self) -> DATA_3_STATUS_R {
        DATA_3_STATUS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Inverted version of raw selected card_data\\[0\\]
0 - card data not busy 1 - card data busy."]
    #[inline(always)]
    pub fn data_busy(&self) -> DATA_BUSY_R {
        DATA_BUSY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data transmit or receive state-machine is busy."]
    #[inline(always)]
    pub fn data_state_mc_busy(&self) -> DATA_STATE_MC_BUSY_R {
        DATA_STATE_MC_BUSY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:16 - Index of previous response, including any auto-stop sent by core."]
    #[inline(always)]
    pub fn response_index(&self) -> RESPONSE_INDEX_R {
        RESPONSE_INDEX_R::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:29 - FIFO count - Number of filled locations in FIFO."]
    #[inline(always)]
    pub fn fifo_count(&self) -> FIFO_COUNT_R {
        FIFO_COUNT_R::new(((self.bits >> 17) & 0x1fff) as u16)
    }
    #[doc = "Bit 30 - DMA acknowledge signal state."]
    #[inline(always)]
    pub fn dma_ack(&self) -> DMA_ACK_R {
        DMA_ACK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA request signal state."]
    #[inline(always)]
    pub fn dma_req(&self) -> DMA_REQ_R {
        DMA_REQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO reached Receive watermark level; not qualified with data transfer."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_rx_watermark(&mut self) -> FIFO_RX_WATERMARK_W<0> {
        FIFO_RX_WATERMARK_W::new(self)
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_tx_watermark(&mut self) -> FIFO_TX_WATERMARK_W<1> {
        FIFO_TX_WATERMARK_W::new(self)
    }
    #[doc = "Bit 2 - FIFO is empty status."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_empty(&mut self) -> FIFO_EMPTY_W<2> {
        FIFO_EMPTY_W::new(self)
    }
    #[doc = "Bit 3 - FIFO is full status."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_full(&mut self) -> FIFO_FULL_W<3> {
        FIFO_FULL_W::new(self)
    }
    #[doc = "Bits 4:7 - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[inline(always)]
    #[must_use]
    pub fn cmdfsmstates(&mut self) -> CMDFSMSTATES_W<4> {
        CMDFSMSTATES_W::new(self)
    }
    #[doc = "Bit 8 - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[inline(always)]
    #[must_use]
    pub fn data_3_status(&mut self) -> DATA_3_STATUS_W<8> {
        DATA_3_STATUS_W::new(self)
    }
    #[doc = "Bit 9 - Inverted version of raw selected card_data\\[0\\]
0 - card data not busy 1 - card data busy."]
    #[inline(always)]
    #[must_use]
    pub fn data_busy(&mut self) -> DATA_BUSY_W<9> {
        DATA_BUSY_W::new(self)
    }
    #[doc = "Bit 10 - Data transmit or receive state-machine is busy."]
    #[inline(always)]
    #[must_use]
    pub fn data_state_mc_busy(&mut self) -> DATA_STATE_MC_BUSY_W<10> {
        DATA_STATE_MC_BUSY_W::new(self)
    }
    #[doc = "Bits 11:16 - Index of previous response, including any auto-stop sent by core."]
    #[inline(always)]
    #[must_use]
    pub fn response_index(&mut self) -> RESPONSE_INDEX_W<11> {
        RESPONSE_INDEX_W::new(self)
    }
    #[doc = "Bits 17:29 - FIFO count - Number of filled locations in FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_count(&mut self) -> FIFO_COUNT_W<17> {
        FIFO_COUNT_W::new(self)
    }
    #[doc = "Bit 30 - DMA acknowledge signal state."]
    #[inline(always)]
    #[must_use]
    pub fn dma_ack(&mut self) -> DMA_ACK_W<30> {
        DMA_ACK_W::new(self)
    }
    #[doc = "Bit 31 - DMA request signal state."]
    #[inline(always)]
    #[must_use]
    pub fn dma_req(&mut self) -> DMA_REQ_W<31> {
        DMA_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x0406"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0406;
}

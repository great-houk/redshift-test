#[doc = "Register `FIFOINTSTAT` reader"]
pub struct R(crate::R<FIFOINTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOINTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOINTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOINTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXERR` reader - TX FIFO error."]
pub type TXERR_R = crate::BitReader<bool>;
#[doc = "Field `RXERR` reader - RX FIFO error."]
pub type RXERR_R = crate::BitReader<bool>;
#[doc = "Field `TXLVL` reader - Transmit FIFO level interrupt."]
pub type TXLVL_R = crate::BitReader<bool>;
#[doc = "Field `RXLVL` reader - Receive FIFO level interrupt."]
pub type RXLVL_R = crate::BitReader<bool>;
#[doc = "Field `PERINT` reader - Peripheral interrupt."]
pub type PERINT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - TX FIFO error."]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO error."]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO level interrupt."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO level interrupt."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Peripheral interrupt."]
    #[inline(always)]
    pub fn perint(&self) -> PERINT_R {
        PERINT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "FIFO interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifointstat](index.html) module"]
pub struct FIFOINTSTAT_SPEC;
impl crate::RegisterSpec for FIFOINTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifointstat::R](R) reader structure"]
impl crate::Readable for FIFOINTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFOINTSTAT to value 0"]
impl crate::Resettable for FIFOINTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

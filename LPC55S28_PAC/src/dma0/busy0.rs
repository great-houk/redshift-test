#[doc = "Register `BUSY0` reader"]
pub struct R(crate::R<BUSY0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUSY0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUSY0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUSY0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BSY` reader - Busy flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not busy. 1 = busy."]
pub type BSY_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Busy flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not busy. 1 = busy."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(self.bits)
    }
}
#[doc = "Channel Busy status for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [busy0](index.html) module"]
pub struct BUSY0_SPEC;
impl crate::RegisterSpec for BUSY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [busy0::R](R) reader structure"]
impl crate::Readable for BUSY0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUSY0 to value 0"]
impl crate::Resettable for BUSY0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `ACTIVE0` reader"]
pub struct R(crate::R<ACTIVE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTIVE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTIVE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTIVE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACT` reader - Active flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not active. 1 = active."]
pub type ACT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Active flag for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = not active. 1 = active."]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(self.bits)
    }
}
#[doc = "Channel Active status for all DMA channels.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [active0](index.html) module"]
pub struct ACTIVE0_SPEC;
impl crate::RegisterSpec for ACTIVE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [active0::R](R) reader structure"]
impl crate::Readable for ACTIVE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACTIVE0 to value 0"]
impl crate::Resettable for ACTIVE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

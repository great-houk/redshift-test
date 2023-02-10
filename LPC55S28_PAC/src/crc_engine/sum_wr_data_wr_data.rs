#[doc = "Register `WR_DATA` writer"]
pub struct W(crate::W<SUM_WR_DATA_WR_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUM_WR_DATA_WR_DATA_SPEC>;
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
impl From<crate::W<SUM_WR_DATA_WR_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUM_WR_DATA_WR_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_WR_DATA` writer - Data written to this register will be taken to perform CRC calculation with selected bit order and 1's complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
pub type CRC_WR_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SUM_WR_DATA_WR_DATA_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Data written to this register will be taken to perform CRC calculation with selected bit order and 1's complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
    #[inline(always)]
    #[must_use]
    pub fn crc_wr_data(&mut self) -> CRC_WR_DATA_W<0> {
        CRC_WR_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC data register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sum_wr_data_wr_data](index.html) module"]
pub struct SUM_WR_DATA_WR_DATA_SPEC;
impl crate::RegisterSpec for SUM_WR_DATA_WR_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sum_wr_data_wr_data::W](W) writer structure"]
impl crate::Writable for SUM_WR_DATA_WR_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_DATA to value 0"]
impl crate::Resettable for SUM_WR_DATA_WR_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

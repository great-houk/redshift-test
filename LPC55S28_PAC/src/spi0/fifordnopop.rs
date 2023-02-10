#[doc = "Register `FIFORDNOPOP` reader"]
pub struct R(crate::R<FIFORDNOPOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFORDNOPOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFORDNOPOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFORDNOPOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA` reader - Received data from the FIFO."]
pub type RXDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXSSEL0_N` reader - Slave Select for receive."]
pub type RXSSEL0_N_R = crate::BitReader<bool>;
#[doc = "Field `RXSSEL1_N` reader - Slave Select for receive."]
pub type RXSSEL1_N_R = crate::BitReader<bool>;
#[doc = "Field `RXSSEL2_N` reader - Slave Select for receive."]
pub type RXSSEL2_N_R = crate::BitReader<bool>;
#[doc = "Field `RXSSEL3_N` reader - Slave Select for receive."]
pub type RXSSEL3_N_R = crate::BitReader<bool>;
#[doc = "Field `SOT` reader - Start of transfer flag."]
pub type SOT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - Received data from the FIFO."]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Slave Select for receive."]
    #[inline(always)]
    pub fn rxssel0_n(&self) -> RXSSEL0_N_R {
        RXSSEL0_N_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave Select for receive."]
    #[inline(always)]
    pub fn rxssel1_n(&self) -> RXSSEL1_N_R {
        RXSSEL1_N_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave Select for receive."]
    #[inline(always)]
    pub fn rxssel2_n(&self) -> RXSSEL2_N_R {
        RXSSEL2_N_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slave Select for receive."]
    #[inline(always)]
    pub fn rxssel3_n(&self) -> RXSSEL3_N_R {
        RXSSEL3_N_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Start of transfer flag."]
    #[inline(always)]
    pub fn sot(&self) -> SOT_R {
        SOT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "FIFO data read with no FIFO pop.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifordnopop](index.html) module"]
pub struct FIFORDNOPOP_SPEC;
impl crate::RegisterSpec for FIFORDNOPOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifordnopop::R](R) reader structure"]
impl crate::Readable for FIFORDNOPOP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFORDNOPOP to value 0"]
impl crate::Resettable for FIFORDNOPOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

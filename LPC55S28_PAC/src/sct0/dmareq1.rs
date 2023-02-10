#[doc = "Register `DMAREQ1` reader"]
pub struct R(crate::R<DMAREQ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAREQ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAREQ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAREQ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAREQ1` writer"]
pub struct W(crate::W<DMAREQ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAREQ1_SPEC>;
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
impl From<crate::W<DMAREQ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAREQ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_1` reader - If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type DEV_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DEV_1` writer - If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type DEV_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAREQ1_SPEC, u16, u16, 16, O>;
#[doc = "Field `DRL1` reader - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
pub type DRL1_R = crate::BitReader<bool>;
#[doc = "Field `DRL1` writer - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
pub type DRL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
#[doc = "Field `DRQ1` reader - This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
pub type DRQ1_R = crate::BitReader<bool>;
#[doc = "Field `DRQ1` writer - This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
pub type DRQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn dev_1(&self) -> DEV_1_R {
        DEV_1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[inline(always)]
    pub fn drl1(&self) -> DRL1_R {
        DRL1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub fn drq1(&self) -> DRQ1_R {
        DRQ1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn dev_1(&mut self) -> DEV_1_W<0> {
        DEV_1_W::new(self)
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[inline(always)]
    #[must_use]
    pub fn drl1(&mut self) -> DRL1_W<30> {
        DRL1_W::new(self)
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    #[must_use]
    pub fn drq1(&mut self) -> DRQ1_W<31> {
        DRQ1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT DMA request 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmareq1](index.html) module"]
pub struct DMAREQ1_SPEC;
impl crate::RegisterSpec for DMAREQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmareq1::R](R) reader structure"]
impl crate::Readable for DMAREQ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmareq1::W](W) writer structure"]
impl crate::Writable for DMAREQ1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAREQ1 to value 0"]
impl crate::Resettable for DMAREQ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

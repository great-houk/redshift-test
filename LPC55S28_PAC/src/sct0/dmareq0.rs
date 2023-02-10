#[doc = "Register `DMAREQ0` reader"]
pub struct R(crate::R<DMAREQ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAREQ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAREQ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAREQ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAREQ0` writer"]
pub struct W(crate::W<DMAREQ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAREQ0_SPEC>;
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
impl From<crate::W<DMAREQ0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAREQ0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_0` reader - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type DEV_0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DEV_0` writer - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type DEV_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAREQ0_SPEC, u16, u16, 16, O>;
#[doc = "Field `DRL0` reader - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
pub type DRL0_R = crate::BitReader<bool>;
#[doc = "Field `DRL0` writer - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
pub type DRL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ0_SPEC, bool, O>;
#[doc = "Field `DRQ0` reader - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
pub type DRQ0_R = crate::BitReader<bool>;
#[doc = "Field `DRQ0` writer - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
pub type DRQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAREQ0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn dev_0(&self) -> DEV_0_R {
        DEV_0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[inline(always)]
    pub fn drl0(&self) -> DRL0_R {
        DRL0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub fn drq0(&self) -> DRQ0_R {
        DRQ0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn dev_0(&mut self) -> DEV_0_W<0> {
        DEV_0_W::new(self)
    }
    #[doc = "Bit 30 - A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[inline(always)]
    #[must_use]
    pub fn drl0(&mut self) -> DRL0_W<30> {
        DRL0_W::new(self)
    }
    #[doc = "Bit 31 - This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    #[must_use]
    pub fn drq0(&mut self) -> DRQ0_W<31> {
        DRQ0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT DMA request 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmareq0](index.html) module"]
pub struct DMAREQ0_SPEC;
impl crate::RegisterSpec for DMAREQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmareq0::R](R) reader structure"]
impl crate::Readable for DMAREQ0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmareq0::W](W) writer structure"]
impl crate::Writable for DMAREQ0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAREQ0 to value 0"]
impl crate::Resettable for DMAREQ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `SEED` reader"]
pub struct R(crate::R<SEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEED` writer"]
pub struct W(crate::W<SEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEED_SPEC>;
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
impl From<crate::W<SEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_SEED` reader - A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
pub type CRC_SEED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRC_SEED` writer - A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
pub type CRC_SEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEED_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
    #[inline(always)]
    pub fn crc_seed(&self) -> CRC_SEED_R {
        CRC_SEED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
    #[inline(always)]
    #[must_use]
    pub fn crc_seed(&mut self) -> CRC_SEED_W<0> {
        CRC_SEED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC seed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seed](index.html) module"]
pub struct SEED_SPEC;
impl crate::RegisterSpec for SEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seed::R](R) reader structure"]
impl crate::Readable for SEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seed::W](W) writer structure"]
impl crate::Writable for SEED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEED to value 0xffff"]
impl crate::Resettable for SEED_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}

#[doc = "Register `SRAMBASE` reader"]
pub struct R(crate::R<SRAMBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAMBASE` writer"]
pub struct W(crate::W<SRAMBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAMBASE_SPEC>;
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
impl From<crate::W<SRAMBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAMBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET` reader - Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
pub type OFFSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OFFSET` writer - Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
pub type OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRAMBASE_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bits 9:31 - Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 9:31 - Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<9> {
        OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM address of the channel configuration table.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srambase](index.html) module"]
pub struct SRAMBASE_SPEC;
impl crate::RegisterSpec for SRAMBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srambase::R](R) reader structure"]
impl crate::Readable for SRAMBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srambase::W](W) writer structure"]
impl crate::Writable for SRAMBASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAMBASE to value 0"]
impl crate::Resettable for SRAMBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

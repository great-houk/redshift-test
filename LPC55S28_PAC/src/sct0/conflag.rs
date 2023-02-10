#[doc = "Register `CONFLAG` reader"]
pub struct R(crate::R<CONFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFLAG` writer"]
pub struct W(crate::W<CONFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFLAG_SPEC>;
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
impl From<crate::W<CONFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCFLAG` reader - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub type NCFLAG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NCFLAG` writer - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub type NCFLAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFLAG_SPEC, u16, u16, 16, O>;
#[doc = "Field `BUSERRL` reader - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
pub type BUSERRL_R = crate::BitReader<bool>;
#[doc = "Field `BUSERRL` writer - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
pub type BUSERRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, bool, O>;
#[doc = "Field `BUSERRH` reader - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
pub type BUSERRH_R = crate::BitReader<bool>;
#[doc = "Field `BUSERRH` writer - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
pub type BUSERRH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFLAG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn ncflag(&self) -> NCFLAG_R {
        NCFLAG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[inline(always)]
    pub fn buserrl(&self) -> BUSERRL_R {
        BUSERRL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[inline(always)]
    pub fn buserrh(&self) -> BUSERRH_R {
        BUSERRH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn ncflag(&mut self) -> NCFLAG_W<0> {
        NCFLAG_W::new(self)
    }
    #[doc = "Bit 30 - The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[inline(always)]
    #[must_use]
    pub fn buserrl(&mut self) -> BUSERRL_W<30> {
        BUSERRL_W::new(self)
    }
    #[doc = "Bit 31 - The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[inline(always)]
    #[must_use]
    pub fn buserrh(&mut self) -> BUSERRH_W<31> {
        BUSERRH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT conflict flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conflag](index.html) module"]
pub struct CONFLAG_SPEC;
impl crate::RegisterSpec for CONFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conflag::R](R) reader structure"]
impl crate::Readable for CONFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conflag::W](W) writer structure"]
impl crate::Writable for CONFLAG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFLAG to value 0"]
impl crate::Resettable for CONFLAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

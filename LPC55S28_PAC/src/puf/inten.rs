#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READYEN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type READYEN_R = crate::BitReader<bool>;
#[doc = "Field `READYEN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type READYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `SUCCESEN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type SUCCESEN_R = crate::BitReader<bool>;
#[doc = "Field `SUCCESEN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type SUCCESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `ERROREN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type ERROREN_R = crate::BitReader<bool>;
#[doc = "Field `ERROREN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type ERROREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `KEYINREQEN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type KEYINREQEN_R = crate::BitReader<bool>;
#[doc = "Field `KEYINREQEN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type KEYINREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `KEYOUTAVAILEN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type KEYOUTAVAILEN_R = crate::BitReader<bool>;
#[doc = "Field `KEYOUTAVAILEN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type KEYOUTAVAILEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `CODEINREQEN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type CODEINREQEN_R = crate::BitReader<bool>;
#[doc = "Field `CODEINREQEN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type CODEINREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `CODEOUTAVAILEN` reader - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type CODEOUTAVAILEN_R = crate::BitReader<bool>;
#[doc = "Field `CODEOUTAVAILEN` writer - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
pub type CODEOUTAVAILEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn readyen(&self) -> READYEN_R {
        READYEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn succesen(&self) -> SUCCESEN_R {
        SUCCESEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn erroren(&self) -> ERROREN_R {
        ERROREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn keyinreqen(&self) -> KEYINREQEN_R {
        KEYINREQEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn keyoutavailen(&self) -> KEYOUTAVAILEN_R {
        KEYOUTAVAILEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn codeinreqen(&self) -> CODEINREQEN_R {
        CODEINREQEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    pub fn codeoutavailen(&self) -> CODEOUTAVAILEN_R {
        CODEOUTAVAILEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    #[must_use]
    pub fn readyen(&mut self) -> READYEN_W<0> {
        READYEN_W::new(self)
    }
    #[doc = "Bit 1 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    #[must_use]
    pub fn succesen(&mut self) -> SUCCESEN_W<1> {
        SUCCESEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    #[must_use]
    pub fn erroren(&mut self) -> ERROREN_W<2> {
        ERROREN_W::new(self)
    }
    #[doc = "Bit 4 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    #[must_use]
    pub fn keyinreqen(&mut self) -> KEYINREQEN_W<4> {
        KEYINREQEN_W::new(self)
    }
    #[doc = "Bit 5 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    #[must_use]
    pub fn keyoutavailen(&mut self) -> KEYOUTAVAILEN_W<5> {
        KEYOUTAVAILEN_W::new(self)
    }
    #[doc = "Bit 6 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    #[must_use]
    pub fn codeinreqen(&mut self) -> CODEINREQEN_W<6> {
        CODEINREQEN_W::new(self)
    }
    #[doc = "Bit 7 - Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)"]
    #[inline(always)]
    #[must_use]
    pub fn codeoutavailen(&mut self) -> CODEOUTAVAILEN_W<7> {
        CODEOUTAVAILEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

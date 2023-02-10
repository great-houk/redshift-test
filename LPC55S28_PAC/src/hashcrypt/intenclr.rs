#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITING` reader - Write 1 to clear mask."]
pub type WAITING_R = crate::BitReader<bool>;
#[doc = "Field `WAITING` writer - Write 1 to clear mask."]
pub type WAITING_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `DIGEST` reader - Write 1 to clear mask."]
pub type DIGEST_R = crate::BitReader<bool>;
#[doc = "Field `DIGEST` writer - Write 1 to clear mask."]
pub type DIGEST_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTENCLR_SPEC, bool, O>;
#[doc = "Field `ERROR` reader - Write 1 to clear mask."]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - Write 1 to clear mask."]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, INTENCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write 1 to clear mask."]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to clear mask."]
    #[inline(always)]
    pub fn digest(&self) -> DIGEST_R {
        DIGEST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to clear mask."]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear mask."]
    #[inline(always)]
    #[must_use]
    pub fn waiting(&mut self) -> WAITING_W<0> {
        WAITING_W::new(self)
    }
    #[doc = "Bit 1 - Write 1 to clear mask."]
    #[inline(always)]
    #[must_use]
    pub fn digest(&mut self) -> DIGEST_W<1> {
        DIGEST_W::new(self)
    }
    #[doc = "Bit 2 - Write 1 to clear mask."]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<2> {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write 1 to clear interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x07;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

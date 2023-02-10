#[doc = "Register `RST_N` reader"]
pub struct R(crate::R<RST_N_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_N_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_N_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_N_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST_N` writer"]
pub struct W(crate::W<RST_N_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_N_SPEC>;
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
impl From<crate::W<RST_N_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_N_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARD_RESET` reader - Hardware reset."]
pub type CARD_RESET_R = crate::BitReader<bool>;
#[doc = "Field `CARD_RESET` writer - Hardware reset."]
pub type CARD_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, RST_N_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Hardware reset."]
    #[inline(always)]
    pub fn card_reset(&self) -> CARD_RESET_R {
        CARD_RESET_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware reset."]
    #[inline(always)]
    #[must_use]
    pub fn card_reset(&mut self) -> CARD_RESET_W<0> {
        CARD_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_n](index.html) module"]
pub struct RST_N_SPEC;
impl crate::RegisterSpec for RST_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst_n::R](R) reader structure"]
impl crate::Readable for RST_N_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_n::W](W) writer structure"]
impl crate::Writable for RST_N_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RST_N to value 0x01"]
impl crate::Resettable for RST_N_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

#[doc = "Register `ENC_ENABLE` reader"]
pub struct R(crate::R<ENC_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENC_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENC_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENC_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENC_ENABLE` writer"]
pub struct W(crate::W<ENC_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENC_ENABLE_SPEC>;
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
impl From<crate::W<ENC_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENC_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Encryption Enable."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Encryption Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Encryption of writes to the flash controller DATAW* registers is disabled."]
    DISABLED = 0,
    #[doc = "1: Encryption of writes to the flash controller DATAW* registers is enabled."]
    ENABLED = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLED,
            true => EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::ENABLED
    }
}
#[doc = "Field `EN` writer - Encryption Enable."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENC_ENABLE_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Encryption of writes to the flash controller DATAW* registers is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::DISABLED)
    }
    #[doc = "Encryption of writes to the flash controller DATAW* registers is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Encryption Enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Encryption Enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Encryption Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_enable](index.html) module"]
pub struct ENC_ENABLE_SPEC;
impl crate::RegisterSpec for ENC_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enc_enable::R](R) reader structure"]
impl crate::Readable for ENC_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enc_enable::W](W) writer structure"]
impl crate::Writable for ENC_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENC_ENABLE to value 0"]
impl crate::Resettable for ENC_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

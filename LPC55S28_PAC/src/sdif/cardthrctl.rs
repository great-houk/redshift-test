#[doc = "Register `CARDTHRCTL` reader"]
pub struct R(crate::R<CARDTHRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CARDTHRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CARDTHRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CARDTHRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CARDTHRCTL` writer"]
pub struct W(crate::W<CARDTHRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CARDTHRCTL_SPEC>;
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
impl From<crate::W<CARDTHRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CARDTHRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARDRDTHREN` reader - Card Read Threshold Enable."]
pub type CARDRDTHREN_R = crate::BitReader<bool>;
#[doc = "Field `CARDRDTHREN` writer - Card Read Threshold Enable."]
pub type CARDRDTHREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CARDTHRCTL_SPEC, bool, O>;
#[doc = "Field `BSYCLRINTEN` reader - Busy Clear Interrupt Enable."]
pub type BSYCLRINTEN_R = crate::BitReader<bool>;
#[doc = "Field `BSYCLRINTEN` writer - Busy Clear Interrupt Enable."]
pub type BSYCLRINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CARDTHRCTL_SPEC, bool, O>;
#[doc = "Field `CARDTHRESHOLD` reader - Card Threshold size."]
pub type CARDTHRESHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CARDTHRESHOLD` writer - Card Threshold size."]
pub type CARDTHRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CARDTHRCTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Card Read Threshold Enable."]
    #[inline(always)]
    pub fn cardrdthren(&self) -> CARDRDTHREN_R {
        CARDRDTHREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy Clear Interrupt Enable."]
    #[inline(always)]
    pub fn bsyclrinten(&self) -> BSYCLRINTEN_R {
        BSYCLRINTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Card Threshold size."]
    #[inline(always)]
    pub fn cardthreshold(&self) -> CARDTHRESHOLD_R {
        CARDTHRESHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Card Read Threshold Enable."]
    #[inline(always)]
    #[must_use]
    pub fn cardrdthren(&mut self) -> CARDRDTHREN_W<0> {
        CARDRDTHREN_W::new(self)
    }
    #[doc = "Bit 1 - Busy Clear Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn bsyclrinten(&mut self) -> BSYCLRINTEN_W<1> {
        BSYCLRINTEN_W::new(self)
    }
    #[doc = "Bits 16:23 - Card Threshold size."]
    #[inline(always)]
    #[must_use]
    pub fn cardthreshold(&mut self) -> CARDTHRESHOLD_W<16> {
        CARDTHRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Card Threshold Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cardthrctl](index.html) module"]
pub struct CARDTHRCTL_SPEC;
impl crate::RegisterSpec for CARDTHRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cardthrctl::R](R) reader structure"]
impl crate::Readable for CARDTHRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cardthrctl::W](W) writer structure"]
impl crate::Writable for CARDTHRCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CARDTHRCTL to value 0"]
impl crate::Resettable for CARDTHRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

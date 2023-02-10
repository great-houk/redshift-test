#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLOCKENROLL_SETKEY` reader - Block enroll operation. Write 1 to set, cleared on reset."]
pub type BLOCKENROLL_SETKEY_R = crate::BitReader<bool>;
#[doc = "Field `BLOCKENROLL_SETKEY` writer - Block enroll operation. Write 1 to set, cleared on reset."]
pub type BLOCKENROLL_SETKEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `BLOCKKEYOUTPUT` reader - Block set key operation. Write 1 to set, cleared on reset."]
pub type BLOCKKEYOUTPUT_R = crate::BitReader<bool>;
#[doc = "Field `BLOCKKEYOUTPUT` writer - Block set key operation. Write 1 to set, cleared on reset."]
pub type BLOCKKEYOUTPUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Block enroll operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockenroll_setkey(&self) -> BLOCKENROLL_SETKEY_R {
        BLOCKENROLL_SETKEY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block set key operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub fn blockkeyoutput(&self) -> BLOCKKEYOUTPUT_R {
        BLOCKKEYOUTPUT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Block enroll operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    #[must_use]
    pub fn blockenroll_setkey(&mut self) -> BLOCKENROLL_SETKEY_W<0> {
        BLOCKENROLL_SETKEY_W::new(self)
    }
    #[doc = "Bit 1 - Block set key operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    #[must_use]
    pub fn blockkeyoutput(&mut self) -> BLOCKKEYOUTPUT_W<1> {
        BLOCKKEYOUTPUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF config register for block bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

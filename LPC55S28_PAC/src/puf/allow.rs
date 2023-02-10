#[doc = "Register `ALLOW` reader"]
pub struct R(crate::R<ALLOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALLOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALLOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALLOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALLOW` writer"]
pub struct W(crate::W<ALLOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALLOW_SPEC>;
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
impl From<crate::W<ALLOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALLOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALLOWENROLL` reader - Enroll operation is allowed"]
pub type ALLOWENROLL_R = crate::BitReader<bool>;
#[doc = "Field `ALLOWSTART` reader - Start operation is allowed"]
pub type ALLOWSTART_R = crate::BitReader<bool>;
#[doc = "Field `ALLOWSETKEY` reader - Set Key operations are allowed"]
pub type ALLOWSETKEY_R = crate::BitReader<bool>;
#[doc = "Field `ALLOWGETKEY` reader - Get Key operation is allowed"]
pub type ALLOWGETKEY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Enroll operation is allowed"]
    #[inline(always)]
    pub fn allowenroll(&self) -> ALLOWENROLL_R {
        ALLOWENROLL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start operation is allowed"]
    #[inline(always)]
    pub fn allowstart(&self) -> ALLOWSTART_R {
        ALLOWSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set Key operations are allowed"]
    #[inline(always)]
    pub fn allowsetkey(&self) -> ALLOWSETKEY_R {
        ALLOWSETKEY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Get Key operation is allowed"]
    #[inline(always)]
    pub fn allowgetkey(&self) -> ALLOWGETKEY_R {
        ALLOWGETKEY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Allow register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [allow](index.html) module"]
pub struct ALLOW_SPEC;
impl crate::RegisterSpec for ALLOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [allow::R](R) reader structure"]
impl crate::Readable for ALLOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [allow::W](W) writer structure"]
impl crate::Writable for ALLOW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALLOW to value 0"]
impl crate::Resettable for ALLOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

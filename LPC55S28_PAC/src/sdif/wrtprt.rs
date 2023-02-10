#[doc = "Register `WRTPRT` reader"]
pub struct R(crate::R<WRTPRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRTPRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRTPRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRTPRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRTPRT` writer"]
pub struct W(crate::W<WRTPRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRTPRT_SPEC>;
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
impl From<crate::W<WRTPRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRTPRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITE_PROTECT` reader - Write protect."]
pub type WRITE_PROTECT_R = crate::BitReader<bool>;
#[doc = "Field `WRITE_PROTECT` writer - Write protect."]
pub type WRITE_PROTECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, WRTPRT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write protect."]
    #[inline(always)]
    pub fn write_protect(&self) -> WRITE_PROTECT_R {
        WRITE_PROTECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write protect."]
    #[inline(always)]
    #[must_use]
    pub fn write_protect(&mut self) -> WRITE_PROTECT_W<0> {
        WRITE_PROTECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Protect register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrtprt](index.html) module"]
pub struct WRTPRT_SPEC;
impl crate::RegisterSpec for WRTPRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrtprt::R](R) reader structure"]
impl crate::Readable for WRTPRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrtprt::W](W) writer structure"]
impl crate::Writable for WRTPRT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRTPRT to value 0"]
impl crate::Resettable for WRTPRT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

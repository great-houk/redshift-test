#[doc = "Register `IFSTAT` reader"]
pub struct R(crate::R<IFSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFSTAT` writer"]
pub struct W(crate::W<IFSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFSTAT_SPEC>;
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
impl From<crate::W<IFSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERROR` reader - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that an APB error has occurred,Writing logic1 clears the if_error bit"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<0> {
        ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF Interface Status and clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifstat](index.html) module"]
pub struct IFSTAT_SPEC;
impl crate::RegisterSpec for IFSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifstat::R](R) reader structure"]
impl crate::Readable for IFSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifstat::W](W) writer structure"]
impl crate::Writable for IFSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFSTAT to value 0"]
impl crate::Resettable for IFSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

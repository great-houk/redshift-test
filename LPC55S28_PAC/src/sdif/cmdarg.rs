#[doc = "Register `CMDARG` reader"]
pub struct R(crate::R<CMDARG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDARG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDARG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDARG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDARG` writer"]
pub struct W(crate::W<CMDARG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDARG_SPEC>;
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
impl From<crate::W<CMDARG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDARG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_ARG` reader - Value indicates command argument to be passed to card."]
pub type CMD_ARG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CMD_ARG` writer - Value indicates command argument to be passed to card."]
pub type CMD_ARG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDARG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to card."]
    #[inline(always)]
    pub fn cmd_arg(&self) -> CMD_ARG_R {
        CMD_ARG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to card."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_arg(&mut self) -> CMD_ARG_W<0> {
        CMD_ARG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdarg](index.html) module"]
pub struct CMDARG_SPEC;
impl crate::RegisterSpec for CMDARG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdarg::R](R) reader structure"]
impl crate::Readable for CMDARG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdarg::W](W) writer structure"]
impl crate::Writable for CMDARG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDARG to value 0"]
impl crate::Resettable for CMDARG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

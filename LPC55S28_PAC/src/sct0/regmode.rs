#[doc = "Register `REGMODE` reader"]
pub struct R(crate::R<REGMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGMODE` writer"]
pub struct W(crate::W<REGMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGMODE_SPEC>;
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
impl From<crate::W<REGMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGMOD_L` reader - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
pub type REGMOD_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REGMOD_L` writer - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
pub type REGMOD_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REGMODE_SPEC, u16, u16, 16, O>;
#[doc = "Field `REGMOD_H` reader - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
pub type REGMOD_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REGMOD_H` writer - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
pub type REGMOD_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REGMODE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[inline(always)]
    pub fn regmod_l(&self) -> REGMOD_L_R {
        REGMOD_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[inline(always)]
    pub fn regmod_h(&self) -> REGMOD_H_R {
        REGMOD_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[inline(always)]
    #[must_use]
    pub fn regmod_l(&mut self) -> REGMOD_L_W<0> {
        REGMOD_L_W::new(self)
    }
    #[doc = "Bits 16:31 - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[inline(always)]
    #[must_use]
    pub fn regmod_h(&mut self) -> REGMOD_H_W<16> {
        REGMOD_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT match/capture mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regmode](index.html) module"]
pub struct REGMODE_SPEC;
impl crate::RegisterSpec for REGMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regmode::R](R) reader structure"]
impl crate::Readable for REGMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regmode::W](W) writer structure"]
impl crate::Writable for REGMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGMODE to value 0"]
impl crate::Resettable for REGMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `BREG` reader"]
pub struct R(crate::R<BREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BREG` writer"]
pub struct W(crate::W<BREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BREG_SPEC>;
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
impl From<crate::W<BREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_VALUE` reader - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
pub type REG_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REG_VALUE` writer - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
pub type REG_VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BREG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub fn reg_value(&self) -> REG_VALUE_R {
        REG_VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    #[must_use]
    pub fn reg_value(&mut self) -> REG_VALUE_W<0> {
        REG_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "B register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [breg](index.html) module"]
pub struct BREG_SPEC;
impl crate::RegisterSpec for BREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [breg::R](R) reader structure"]
impl crate::Readable for BREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [breg::W](W) writer structure"]
impl crate::Writable for BREG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BREG to value 0"]
impl crate::Resettable for BREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

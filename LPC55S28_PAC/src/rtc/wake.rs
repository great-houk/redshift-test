#[doc = "Register `WAKE` reader"]
pub struct R(crate::R<WAKE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKE` writer"]
pub struct W(crate::W<WAKE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKE_SPEC>;
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
impl From<crate::W<WAKE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
pub type VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VAL` writer - A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WAKE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High-resolution/wake-up timer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wake](index.html) module"]
pub struct WAKE_SPEC;
impl crate::RegisterSpec for WAKE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wake::R](R) reader structure"]
impl crate::Readable for WAKE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wake::W](W) writer structure"]
impl crate::Writable for WAKE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAKE to value 0"]
impl crate::Resettable for WAKE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

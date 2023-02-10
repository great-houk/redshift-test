#[doc = "Register `OUTPUT` reader"]
pub struct R(crate::R<OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTPUT` writer"]
pub struct W(crate::W<OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTPUT_SPEC>;
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
impl From<crate::W<OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT` reader - Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub type OUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OUT` writer - Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
pub type OUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTPUT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn out(&mut self) -> OUT_W<0> {
        OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT output register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [output](index.html) module"]
pub struct OUTPUT_SPEC;
impl crate::RegisterSpec for OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [output::R](R) reader structure"]
impl crate::Readable for OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [output::W](W) writer structure"]
impl crate::Writable for OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTPUT to value 0"]
impl crate::Resettable for OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

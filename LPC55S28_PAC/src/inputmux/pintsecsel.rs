#[doc = "Register `PINTSECSEL[%s]` reader"]
pub struct R(crate::R<PINTSECSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINTSECSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINTSECSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINTSECSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINTSECSEL[%s]` writer"]
pub struct W(crate::W<PINTSECSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINTSECSEL_SPEC>;
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
impl From<crate::W<PINTSECSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINTSECSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTPIN` reader - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
pub type INTPIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTPIN` writer - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
pub type INTPIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PINTSECSEL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
    #[inline(always)]
    pub fn intpin(&self) -> INTPIN_R {
        INTPIN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
    #[inline(always)]
    #[must_use]
    pub fn intpin(&mut self) -> INTPIN_W<0> {
        INTPIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin interrupt secure select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pintsecsel](index.html) module"]
pub struct PINTSECSEL_SPEC;
impl crate::RegisterSpec for PINTSECSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pintsecsel::R](R) reader structure"]
impl crate::Readable for PINTSECSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pintsecsel::W](W) writer structure"]
impl crate::Writable for PINTSECSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINTSECSEL[%s]
to value 0x3f"]
impl crate::Resettable for PINTSECSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}

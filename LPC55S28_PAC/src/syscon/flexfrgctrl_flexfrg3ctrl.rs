#[doc = "Register `FLEXFRG3CTRL` reader"]
pub struct R(crate::R<FLEXFRGCTRL_FLEXFRG3CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLEXFRGCTRL_FLEXFRG3CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLEXFRGCTRL_FLEXFRG3CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLEXFRGCTRL_FLEXFRG3CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLEXFRG3CTRL` writer"]
pub struct W(crate::W<FLEXFRGCTRL_FLEXFRG3CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLEXFRGCTRL_FLEXFRG3CTRL_SPEC>;
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
impl From<crate::W<FLEXFRGCTRL_FLEXFRG3CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLEXFRGCTRL_FLEXFRG3CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Denominator of the fractional rate divider."]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - Denominator of the fractional rate divider."]
pub type DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLEXFRGCTRL_FLEXFRG3CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `MULT` reader - Numerator of the fractional rate divider."]
pub type MULT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MULT` writer - Numerator of the fractional rate divider."]
pub type MULT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLEXFRGCTRL_FLEXFRG3CTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Denominator of the fractional rate divider."]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Numerator of the fractional rate divider."]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Denominator of the fractional rate divider."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bits 8:15 - Numerator of the fractional rate divider."]
    #[inline(always)]
    #[must_use]
    pub fn mult(&mut self) -> MULT_W<8> {
        MULT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional rate divider for flexcomm 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flexfrgctrl_flexfrg3ctrl](index.html) module"]
pub struct FLEXFRGCTRL_FLEXFRG3CTRL_SPEC;
impl crate::RegisterSpec for FLEXFRGCTRL_FLEXFRG3CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flexfrgctrl_flexfrg3ctrl::R](R) reader structure"]
impl crate::Readable for FLEXFRGCTRL_FLEXFRG3CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flexfrgctrl_flexfrg3ctrl::W](W) writer structure"]
impl crate::Writable for FLEXFRGCTRL_FLEXFRG3CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLEXFRG3CTRL to value 0xff"]
impl crate::Resettable for FLEXFRGCTRL_FLEXFRG3CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}

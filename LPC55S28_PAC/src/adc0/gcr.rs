#[doc = "Register `GCR[%s]` reader"]
pub struct R(crate::R<GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCR[%s]` writer"]
pub struct W(crate::W<GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCR_SPEC>;
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
impl From<crate::W<GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCALR` reader - Gain Calculation Result"]
pub type GCALR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GCALR` writer - Gain Calculation Result"]
pub type GCALR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCR_SPEC, u16, u16, 16, O>;
#[doc = "Field `RDY` reader - Gain Calculation Ready"]
pub type RDY_R = crate::BitReader<RDY_A>;
#[doc = "Gain Calculation Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDY_A {
    #[doc = "0: The gain offset calculation value is invalid."]
    RDY_0 = 0,
    #[doc = "1: The gain calibration value is valid."]
    RDY_1 = 1,
}
impl From<RDY_A> for bool {
    #[inline(always)]
    fn from(variant: RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDY_A {
        match self.bits {
            false => RDY_A::RDY_0,
            true => RDY_A::RDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDY_0`"]
    #[inline(always)]
    pub fn is_rdy_0(&self) -> bool {
        *self == RDY_A::RDY_0
    }
    #[doc = "Checks if the value of the field is `RDY_1`"]
    #[inline(always)]
    pub fn is_rdy_1(&self) -> bool {
        *self == RDY_A::RDY_1
    }
}
#[doc = "Field `RDY` writer - Gain Calculation Ready"]
pub type RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCR_SPEC, RDY_A, O>;
impl<'a, const O: u8> RDY_W<'a, O> {
    #[doc = "The gain offset calculation value is invalid."]
    #[inline(always)]
    pub fn rdy_0(self) -> &'a mut W {
        self.variant(RDY_A::RDY_0)
    }
    #[doc = "The gain calibration value is valid."]
    #[inline(always)]
    pub fn rdy_1(self) -> &'a mut W {
        self.variant(RDY_A::RDY_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Gain Calculation Result"]
    #[inline(always)]
    pub fn gcalr(&self) -> GCALR_R {
        GCALR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - Gain Calculation Ready"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Gain Calculation Result"]
    #[inline(always)]
    #[must_use]
    pub fn gcalr(&mut self) -> GCALR_W<0> {
        GCALR_W::new(self)
    }
    #[doc = "Bit 24 - Gain Calculation Ready"]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<24> {
        RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gain Calculation Result\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcr](index.html) module"]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcr::R](R) reader structure"]
impl crate::Readable for GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcr::W](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCR[%s]
to value 0"]
impl crate::Resettable for GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

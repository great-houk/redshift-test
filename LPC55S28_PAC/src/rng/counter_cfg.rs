#[doc = "Register `COUNTER_CFG` reader"]
pub struct R(crate::R<COUNTER_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNTER_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNTER_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNTER_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNTER_CFG` writer"]
pub struct W(crate::W<COUNTER_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNTER_CFG_SPEC>;
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
impl From<crate::W<COUNTER_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNTER_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - 00: disabled 01: update once."]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - 00: disabled 01: update once."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COUNTER_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLOCK_SEL` reader - Selects the internal clock on which to compute statistics."]
pub type CLOCK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_SEL` writer - Selects the internal clock on which to compute statistics."]
pub type CLOCK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COUNTER_CFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `SHIFT4X` reader - To be used to add precision to clock_ratio and determine 'entropy refill'."]
pub type SHIFT4X_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHIFT4X` writer - To be used to add precision to clock_ratio and determine 'entropy refill'."]
pub type SHIFT4X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COUNTER_CFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1 - 00: disabled 01: update once."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Selects the internal clock on which to compute statistics."]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - To be used to add precision to clock_ratio and determine 'entropy refill'."]
    #[inline(always)]
    pub fn shift4x(&self) -> SHIFT4X_R {
        SHIFT4X_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00: disabled 01: update once."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bits 2:4 - Selects the internal clock on which to compute statistics."]
    #[inline(always)]
    #[must_use]
    pub fn clock_sel(&mut self) -> CLOCK_SEL_W<2> {
        CLOCK_SEL_W::new(self)
    }
    #[doc = "Bits 5:7 - To be used to add precision to clock_ratio and determine 'entropy refill'."]
    #[inline(always)]
    #[must_use]
    pub fn shift4x(&mut self) -> SHIFT4X_W<5> {
        SHIFT4X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter_cfg](index.html) module"]
pub struct COUNTER_CFG_SPEC;
impl crate::RegisterSpec for COUNTER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [counter_cfg::R](R) reader structure"]
impl crate::Readable for COUNTER_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [counter_cfg::W](W) writer structure"]
impl crate::Writable for COUNTER_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COUNTER_CFG to value 0"]
impl crate::Resettable for COUNTER_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

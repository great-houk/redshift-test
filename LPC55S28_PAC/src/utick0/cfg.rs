#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPEN0` reader - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
pub type CAPEN0_R = crate::BitReader<bool>;
#[doc = "Field `CAPEN0` writer - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
pub type CAPEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CAPEN1` reader - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
pub type CAPEN1_R = crate::BitReader<bool>;
#[doc = "Field `CAPEN1` writer - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
pub type CAPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CAPEN2` reader - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
pub type CAPEN2_R = crate::BitReader<bool>;
#[doc = "Field `CAPEN2` writer - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
pub type CAPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CAPEN3` reader - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
pub type CAPEN3_R = crate::BitReader<bool>;
#[doc = "Field `CAPEN3` writer - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
pub type CAPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CAPPOL0` reader - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type CAPPOL0_R = crate::BitReader<bool>;
#[doc = "Field `CAPPOL0` writer - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type CAPPOL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CAPPOL1` reader - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type CAPPOL1_R = crate::BitReader<bool>;
#[doc = "Field `CAPPOL1` writer - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type CAPPOL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CAPPOL2` reader - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type CAPPOL2_R = crate::BitReader<bool>;
#[doc = "Field `CAPPOL2` writer - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type CAPPOL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `CAPPOL3` reader - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type CAPPOL3_R = crate::BitReader<bool>;
#[doc = "Field `CAPPOL3` writer - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
pub type CAPPOL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen0(&self) -> CAPEN0_R {
        CAPEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen1(&self) -> CAPEN1_R {
        CAPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen2(&self) -> CAPEN2_R {
        CAPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn capen3(&self) -> CAPEN3_R {
        CAPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol0(&self) -> CAPPOL0_R {
        CAPPOL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol1(&self) -> CAPPOL1_R {
        CAPPOL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol2(&self) -> CAPPOL2_R {
        CAPPOL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn cappol3(&self) -> CAPPOL3_R {
        CAPPOL3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    #[must_use]
    pub fn capen0(&mut self) -> CAPEN0_W<0> {
        CAPEN0_W::new(self)
    }
    #[doc = "Bit 1 - Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    #[must_use]
    pub fn capen1(&mut self) -> CAPEN1_W<1> {
        CAPEN1_W::new(self)
    }
    #[doc = "Bit 2 - Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    #[must_use]
    pub fn capen2(&mut self) -> CAPEN2_W<2> {
        CAPEN2_W::new(self)
    }
    #[doc = "Bit 3 - Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    #[must_use]
    pub fn capen3(&mut self) -> CAPEN3_W<3> {
        CAPEN3_W::new(self)
    }
    #[doc = "Bit 8 - Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    #[must_use]
    pub fn cappol0(&mut self) -> CAPPOL0_W<8> {
        CAPPOL0_W::new(self)
    }
    #[doc = "Bit 9 - Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    #[must_use]
    pub fn cappol1(&mut self) -> CAPPOL1_W<9> {
        CAPPOL1_W::new(self)
    }
    #[doc = "Bit 10 - Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    #[must_use]
    pub fn cappol2(&mut self) -> CAPPOL2_W<10> {
        CAPPOL2_W::new(self)
    }
    #[doc = "Bit 11 - Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    #[must_use]
    pub fn cappol3(&mut self) -> CAPPOL3_W<11> {
        CAPPOL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR0I` reader - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
pub type MR0I_R = crate::BitReader<bool>;
#[doc = "Field `MR0I` writer - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
pub type MR0I_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR0R` reader - Reset on MR0: the TC will be reset if MR0 matches it."]
pub type MR0R_R = crate::BitReader<bool>;
#[doc = "Field `MR0R` writer - Reset on MR0: the TC will be reset if MR0 matches it."]
pub type MR0R_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR0S` reader - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
pub type MR0S_R = crate::BitReader<bool>;
#[doc = "Field `MR0S` writer - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
pub type MR0S_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR1I` reader - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
pub type MR1I_R = crate::BitReader<bool>;
#[doc = "Field `MR1I` writer - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
pub type MR1I_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR1R` reader - Reset on MR1: the TC will be reset if MR1 matches it."]
pub type MR1R_R = crate::BitReader<bool>;
#[doc = "Field `MR1R` writer - Reset on MR1: the TC will be reset if MR1 matches it."]
pub type MR1R_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR1S` reader - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
pub type MR1S_R = crate::BitReader<bool>;
#[doc = "Field `MR1S` writer - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
pub type MR1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR2I` reader - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
pub type MR2I_R = crate::BitReader<bool>;
#[doc = "Field `MR2I` writer - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
pub type MR2I_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR2R` reader - Reset on MR2: the TC will be reset if MR2 matches it."]
pub type MR2R_R = crate::BitReader<bool>;
#[doc = "Field `MR2R` writer - Reset on MR2: the TC will be reset if MR2 matches it."]
pub type MR2R_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR2S` reader - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC."]
pub type MR2S_R = crate::BitReader<bool>;
#[doc = "Field `MR2S` writer - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC."]
pub type MR2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR3I` reader - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
pub type MR3I_R = crate::BitReader<bool>;
#[doc = "Field `MR3I` writer - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
pub type MR3I_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR3R` reader - Reset on MR3: the TC will be reset if MR3 matches it."]
pub type MR3R_R = crate::BitReader<bool>;
#[doc = "Field `MR3R` writer - Reset on MR3: the TC will be reset if MR3 matches it."]
pub type MR3R_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR3S` reader - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
pub type MR3S_R = crate::BitReader<bool>;
#[doc = "Field `MR3S` writer - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
pub type MR3S_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR0RL` reader - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type MR0RL_R = crate::BitReader<bool>;
#[doc = "Field `MR0RL` writer - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type MR0RL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR1RL` reader - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type MR1RL_R = crate::BitReader<bool>;
#[doc = "Field `MR1RL` writer - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type MR1RL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR2RL` reader - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type MR2RL_R = crate::BitReader<bool>;
#[doc = "Field `MR2RL` writer - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type MR2RL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
#[doc = "Field `MR3RL` reader - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type MR3RL_R = crate::BitReader<bool>;
#[doc = "Field `MR3RL` writer - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
pub type MR3RL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn mr0i(&self) -> MR0I_R {
        MR0I_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn mr0r(&self) -> MR0R_R {
        MR0R_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn mr0s(&self) -> MR0S_R {
        MR0S_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn mr1i(&self) -> MR1I_R {
        MR1I_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn mr1r(&self) -> MR1R_R {
        MR1R_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn mr1s(&self) -> MR1S_R {
        MR1S_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn mr2i(&self) -> MR2I_R {
        MR2I_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn mr2r(&self) -> MR2R_R {
        MR2R_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    pub fn mr2s(&self) -> MR2S_R {
        MR2S_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn mr3i(&self) -> MR3I_R {
        MR3I_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn mr3r(&self) -> MR3R_R {
        MR3R_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn mr3s(&self) -> MR3S_R {
        MR3S_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 24 - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr0rl(&self) -> MR0RL_R {
        MR0RL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr1rl(&self) -> MR1RL_R {
        MR1RL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr2rl(&self) -> MR2RL_R {
        MR2RL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub fn mr3rl(&self) -> MR3RL_R {
        MR3RL_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr0i(&mut self) -> MR0I_W<0> {
        MR0I_W::new(self)
    }
    #[doc = "Bit 1 - Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    #[must_use]
    pub fn mr0r(&mut self) -> MR0R_W<1> {
        MR0R_W::new(self)
    }
    #[doc = "Bit 2 - Stop on MR0: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr0s(&mut self) -> MR0S_W<2> {
        MR0S_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr1i(&mut self) -> MR1I_W<3> {
        MR1I_W::new(self)
    }
    #[doc = "Bit 4 - Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    #[must_use]
    pub fn mr1r(&mut self) -> MR1R_W<4> {
        MR1R_W::new(self)
    }
    #[doc = "Bit 5 - Stop on MR1: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr1s(&mut self) -> MR1S_W<5> {
        MR1S_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr2i(&mut self) -> MR2I_W<6> {
        MR2I_W::new(self)
    }
    #[doc = "Bit 7 - Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    #[must_use]
    pub fn mr2r(&mut self) -> MR2R_W<7> {
        MR2R_W::new(self)
    }
    #[doc = "Bit 8 - Stop on MR2: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr2s(&mut self) -> MR2S_W<8> {
        MR2S_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr3i(&mut self) -> MR3I_W<9> {
        MR3I_W::new(self)
    }
    #[doc = "Bit 10 - Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    #[must_use]
    pub fn mr3r(&mut self) -> MR3R_W<10> {
        MR3R_W::new(self)
    }
    #[doc = "Bit 11 - Stop on MR3: the TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    #[must_use]
    pub fn mr3s(&mut self) -> MR3S_W<11> {
        MR3S_W::new(self)
    }
    #[doc = "Bit 24 - Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    #[must_use]
    pub fn mr0rl(&mut self) -> MR0RL_W<24> {
        MR0RL_W::new(self)
    }
    #[doc = "Bit 25 - Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    #[must_use]
    pub fn mr1rl(&mut self) -> MR1RL_W<25> {
        MR1RL_W::new(self)
    }
    #[doc = "Bit 26 - Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    #[must_use]
    pub fn mr2rl(&mut self) -> MR2RL_W<26> {
        MR2RL_W::new(self)
    }
    #[doc = "Bit 27 - Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    #[must_use]
    pub fn mr3rl(&mut self) -> MR3RL_W<27> {
        MR3RL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Match Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

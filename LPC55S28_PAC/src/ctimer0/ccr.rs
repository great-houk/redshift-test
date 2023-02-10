#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAP0RE` reader - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP0RE_R = crate::BitReader<bool>;
#[doc = "Field `CAP0RE` writer - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP0RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CAP0FE` reader - Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP0FE_R = crate::BitReader<bool>;
#[doc = "Field `CAP0FE` writer - Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP0FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CAP0I` reader - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
pub type CAP0I_R = crate::BitReader<bool>;
#[doc = "Field `CAP0I` writer - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
pub type CAP0I_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CAP1RE` reader - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP1RE_R = crate::BitReader<bool>;
#[doc = "Field `CAP1RE` writer - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP1RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CAP1FE` reader - Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP1FE_R = crate::BitReader<bool>;
#[doc = "Field `CAP1FE` writer - Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP1FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CAP1I` reader - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
pub type CAP1I_R = crate::BitReader<bool>;
#[doc = "Field `CAP1I` writer - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
pub type CAP1I_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CAP2RE` reader - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP2RE_R = crate::BitReader<bool>;
#[doc = "Field `CAP2RE` writer - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP2RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CAP2FE` reader - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP2FE_R = crate::BitReader<bool>;
#[doc = "Field `CAP2FE` writer - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP2FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CAP2I` reader - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
pub type CAP2I_R = crate::BitReader<bool>;
#[doc = "Field `CAP2I` writer - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
pub type CAP2I_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CAP3RE` reader - Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP3RE_R = crate::BitReader<bool>;
#[doc = "Field `CAP3RE` writer - Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP3RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CAP3FE` reader - Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP3FE_R = crate::BitReader<bool>;
#[doc = "Field `CAP3FE` writer - Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
pub type CAP3FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
#[doc = "Field `CAP3I` reader - Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
pub type CAP3I_R = crate::BitReader<bool>;
#[doc = "Field `CAP3I` writer - Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
pub type CAP3I_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap0re(&self) -> CAP0RE_R {
        CAP0RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap0fe(&self) -> CAP0FE_R {
        CAP0FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
    #[inline(always)]
    pub fn cap0i(&self) -> CAP0I_R {
        CAP0I_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap1re(&self) -> CAP1RE_R {
        CAP1RE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap1fe(&self) -> CAP1FE_R {
        CAP1FE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
    #[inline(always)]
    pub fn cap1i(&self) -> CAP1I_R {
        CAP1I_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap2re(&self) -> CAP2RE_R {
        CAP2RE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap2fe(&self) -> CAP2FE_R {
        CAP2FE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
    #[inline(always)]
    pub fn cap2i(&self) -> CAP2I_R {
        CAP2I_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap3re(&self) -> CAP3RE_R {
        CAP3RE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub fn cap3fe(&self) -> CAP3FE_R {
        CAP3FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
    #[inline(always)]
    pub fn cap3i(&self) -> CAP3I_R {
        CAP3I_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap0re(&mut self) -> CAP0RE_W<0> {
        CAP0RE_W::new(self)
    }
    #[doc = "Bit 1 - Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap0fe(&mut self) -> CAP0FE_W<1> {
        CAP0FE_W::new(self)
    }
    #[doc = "Bit 2 - Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cap0i(&mut self) -> CAP0I_W<2> {
        CAP0I_W::new(self)
    }
    #[doc = "Bit 3 - Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap1re(&mut self) -> CAP1RE_W<3> {
        CAP1RE_W::new(self)
    }
    #[doc = "Bit 4 - Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap1fe(&mut self) -> CAP1FE_W<4> {
        CAP1FE_W::new(self)
    }
    #[doc = "Bit 5 - Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cap1i(&mut self) -> CAP1I_W<5> {
        CAP1I_W::new(self)
    }
    #[doc = "Bit 6 - Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap2re(&mut self) -> CAP2RE_W<6> {
        CAP2RE_W::new(self)
    }
    #[doc = "Bit 7 - Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap2fe(&mut self) -> CAP2FE_W<7> {
        CAP2FE_W::new(self)
    }
    #[doc = "Bit 8 - Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cap2i(&mut self) -> CAP2I_W<8> {
        CAP2I_W::new(self)
    }
    #[doc = "Bit 9 - Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap3re(&mut self) -> CAP3RE_W<9> {
        CAP3RE_W::new(self)
    }
    #[doc = "Bit 10 - Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap3fe(&mut self) -> CAP3FE_W<10> {
        CAP3FE_W::new(self)
    }
    #[doc = "Bit 11 - Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cap3i(&mut self) -> CAP3I_W<11> {
        CAP3I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

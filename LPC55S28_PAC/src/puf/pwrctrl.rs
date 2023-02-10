#[doc = "Register `PWRCTRL` reader"]
pub struct R(crate::R<PWRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCTRL` writer"]
pub struct W(crate::W<PWRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCTRL_SPEC>;
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
impl From<crate::W<PWRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMON` reader - Power on the PUF RAM."]
pub type RAMON_R = crate::BitReader<bool>;
#[doc = "Field `RAMON` writer - Power on the PUF RAM."]
pub type RAMON_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCTRL_SPEC, bool, O>;
#[doc = "Field `RAMSTAT` reader - PUF RAM status."]
pub type RAMSTAT_R = crate::BitReader<bool>;
#[doc = "Field `RAMSTAT` writer - PUF RAM status."]
pub type RAMSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWRCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Power on the PUF RAM."]
    #[inline(always)]
    pub fn ramon(&self) -> RAMON_R {
        RAMON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PUF RAM status."]
    #[inline(always)]
    pub fn ramstat(&self) -> RAMSTAT_R {
        RAMSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power on the PUF RAM."]
    #[inline(always)]
    #[must_use]
    pub fn ramon(&mut self) -> RAMON_W<0> {
        RAMON_W::new(self)
    }
    #[doc = "Bit 1 - PUF RAM status."]
    #[inline(always)]
    #[must_use]
    pub fn ramstat(&mut self) -> RAMSTAT_W<1> {
        RAMSTAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUF RAM Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrctrl](index.html) module"]
pub struct PWRCTRL_SPEC;
impl crate::RegisterSpec for PWRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrctrl::R](R) reader structure"]
impl crate::Readable for PWRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrctrl::W](W) writer structure"]
impl crate::Writable for PWRCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCTRL to value 0xf8"]
impl crate::Resettable for PWRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xf8;
}

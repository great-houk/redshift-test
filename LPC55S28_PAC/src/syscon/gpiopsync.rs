#[doc = "Register `GPIOPSYNC` reader"]
pub struct R(crate::R<GPIOPSYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOPSYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOPSYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOPSYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOPSYNC` writer"]
pub struct W(crate::W<GPIOPSYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOPSYNC_SPEC>;
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
impl From<crate::W<GPIOPSYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOPSYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSYNC` reader - Enable bypass of the first stage of synchonization inside GPIO_INT module."]
pub type PSYNC_R = crate::BitReader<PSYNC_A>;
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSYNC_A {
    #[doc = "0: use the first stage of synchonization inside GPIO_INT module."]
    USED = 0,
    #[doc = "1: bypass of the first stage of synchonization inside GPIO_INT module."]
    BYPASS = 1,
}
impl From<PSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl PSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSYNC_A {
        match self.bits {
            false => PSYNC_A::USED,
            true => PSYNC_A::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == PSYNC_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == PSYNC_A::BYPASS
    }
}
#[doc = "Field `PSYNC` writer - Enable bypass of the first stage of synchonization inside GPIO_INT module."]
pub type PSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOPSYNC_SPEC, PSYNC_A, O>;
impl<'a, const O: u8> PSYNC_W<'a, O> {
    #[doc = "use the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub fn used(self) -> &'a mut W {
        self.variant(PSYNC_A::USED)
    }
    #[doc = "bypass of the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(PSYNC_A::BYPASS)
    }
}
impl R {
    #[doc = "Bit 0 - Enable bypass of the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    pub fn psync(&self) -> PSYNC_R {
        PSYNC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bypass of the first stage of synchonization inside GPIO_INT module."]
    #[inline(always)]
    #[must_use]
    pub fn psync(&mut self) -> PSYNC_W<0> {
        PSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiopsync](index.html) module"]
pub struct GPIOPSYNC_SPEC;
impl crate::RegisterSpec for GPIOPSYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiopsync::R](R) reader structure"]
impl crate::Readable for GPIOPSYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiopsync::W](W) writer structure"]
impl crate::Writable for GPIOPSYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOPSYNC to value 0"]
impl crate::Resettable for GPIOPSYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

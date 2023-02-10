#[doc = "Register `SLVADR2` reader"]
pub struct R(crate::R<SLVADR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLVADR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLVADR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLVADR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLVADR2` writer"]
pub struct W(crate::W<SLVADR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLVADR2_SPEC>;
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
impl From<crate::W<SLVADR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLVADR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADISABLE` reader - Slave Address n Disable."]
pub type SADISABLE_R = crate::BitReader<SADISABLE_A>;
#[doc = "Slave Address n Disable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SADISABLE_A {
    #[doc = "0: Enabled. Slave Address n is enabled."]
    ENABLED = 0,
    #[doc = "1: Ignored Slave Address n is ignored."]
    DISABLED = 1,
}
impl From<SADISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: SADISABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl SADISABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SADISABLE_A {
        match self.bits {
            false => SADISABLE_A::ENABLED,
            true => SADISABLE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SADISABLE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SADISABLE_A::DISABLED
    }
}
#[doc = "Field `SADISABLE` writer - Slave Address n Disable."]
pub type SADISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLVADR2_SPEC, SADISABLE_A, O>;
impl<'a, const O: u8> SADISABLE_W<'a, O> {
    #[doc = "Enabled. Slave Address n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SADISABLE_A::ENABLED)
    }
    #[doc = "Ignored Slave Address n is ignored."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SADISABLE_A::DISABLED)
    }
}
#[doc = "Field `SLVADR` reader - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
pub type SLVADR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLVADR` writer - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
pub type SLVADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLVADR2_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    pub fn sadisable(&self) -> SADISABLE_R {
        SADISABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub fn slvadr(&self) -> SLVADR_R {
        SLVADR_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    #[must_use]
    pub fn sadisable(&mut self) -> SADISABLE_W<0> {
        SADISABLE_W::new(self)
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    #[must_use]
    pub fn slvadr(&mut self) -> SLVADR_W<1> {
        SLVADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvadr2](index.html) module"]
pub struct SLVADR2_SPEC;
impl crate::RegisterSpec for SLVADR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slvadr2::R](R) reader structure"]
impl crate::Readable for SLVADR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slvadr2::W](W) writer structure"]
impl crate::Writable for SLVADR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLVADR2 to value 0x01"]
impl crate::Resettable for SLVADR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

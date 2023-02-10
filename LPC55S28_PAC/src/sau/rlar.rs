#[doc = "Register `RLAR` reader"]
pub struct R(crate::R<RLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RLAR` writer"]
pub struct W(crate::W<RLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RLAR_SPEC>;
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
impl From<crate::W<RLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable. SAU region enable."]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "Enable. SAU region enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: SAU region is enabled."]
    ENABLED = 0,
    #[doc = "1: SAU region is disabled."]
    DISABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::ENABLED,
            true => ENABLE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
}
#[doc = "Field `ENABLE` writer - Enable. SAU region enable."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RLAR_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "SAU region is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
    }
    #[doc = "SAU region is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
}
#[doc = "Field `NSC` reader - Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
pub type NSC_R = crate::BitReader<NSC_A>;
#[doc = "Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSC_A {
    #[doc = "0: Region is not Non-secure callable."]
    NOT_NON_SECURE_CALLABLE = 0,
    #[doc = "1: Region is Non-secure callable."]
    NON_SECURE_CALLABLE = 1,
}
impl From<NSC_A> for bool {
    #[inline(always)]
    fn from(variant: NSC_A) -> Self {
        variant as u8 != 0
    }
}
impl NSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSC_A {
        match self.bits {
            false => NSC_A::NOT_NON_SECURE_CALLABLE,
            true => NSC_A::NON_SECURE_CALLABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_NON_SECURE_CALLABLE`"]
    #[inline(always)]
    pub fn is_not_non_secure_callable(&self) -> bool {
        *self == NSC_A::NOT_NON_SECURE_CALLABLE
    }
    #[doc = "Checks if the value of the field is `NON_SECURE_CALLABLE`"]
    #[inline(always)]
    pub fn is_non_secure_callable(&self) -> bool {
        *self == NSC_A::NON_SECURE_CALLABLE
    }
}
#[doc = "Field `NSC` writer - Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
pub type NSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RLAR_SPEC, NSC_A, O>;
impl<'a, const O: u8> NSC_W<'a, O> {
    #[doc = "Region is not Non-secure callable."]
    #[inline(always)]
    pub fn not_non_secure_callable(self) -> &'a mut W {
        self.variant(NSC_A::NOT_NON_SECURE_CALLABLE)
    }
    #[doc = "Region is Non-secure callable."]
    #[inline(always)]
    pub fn non_secure_callable(self) -> &'a mut W {
        self.variant(NSC_A::NON_SECURE_CALLABLE)
    }
}
#[doc = "Field `LADDR` reader - Limit address. Holds bits\\[31:5\\]
of the limit address for the selected SAU region. Bits\\[4:0\\]
of the limit address are defined as 0x1F."]
pub type LADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LADDR` writer - Limit address. Holds bits\\[31:5\\]
of the limit address for the selected SAU region. Bits\\[4:0\\]
of the limit address are defined as 0x1F."]
pub type LADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLAR_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bit 0 - Enable. SAU region enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[inline(always)]
    pub fn nsc(&self) -> NSC_R {
        NSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:31 - Limit address. Holds bits\\[31:5\\]
of the limit address for the selected SAU region. Bits\\[4:0\\]
of the limit address are defined as 0x1F."]
    #[inline(always)]
    pub fn laddr(&self) -> LADDR_R {
        LADDR_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Enable. SAU region enable."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Non-secure callable. Controls whether Non-secure state is permitted to execute an SG instruction from this region."]
    #[inline(always)]
    #[must_use]
    pub fn nsc(&mut self) -> NSC_W<1> {
        NSC_W::new(self)
    }
    #[doc = "Bits 5:31 - Limit address. Holds bits\\[31:5\\]
of the limit address for the selected SAU region. Bits\\[4:0\\]
of the limit address are defined as 0x1F."]
    #[inline(always)]
    #[must_use]
    pub fn laddr(&mut self) -> LADDR_W<5> {
        LADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security Attribution Unit Region Limit Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlar](index.html) module"]
pub struct RLAR_SPEC;
impl crate::RegisterSpec for RLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rlar::R](R) reader structure"]
impl crate::Readable for RLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rlar::W](W) writer structure"]
impl crate::Writable for RLAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RLAR to value 0"]
impl crate::Resettable for RLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

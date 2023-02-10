#[doc = "Register `PLL1CTRL` reader"]
pub struct R(crate::R<PLL1CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL1CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL1CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL1CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL1CTRL` writer"]
pub struct W(crate::W<PLL1CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL1CTRL_SPEC>;
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
impl From<crate::W<PLL1CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL1CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELR` reader - Bandwidth select R value."]
pub type SELR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELR` writer - Bandwidth select R value."]
pub type SELR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SELI` reader - Bandwidth select I value."]
pub type SELI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELI` writer - Bandwidth select I value."]
pub type SELI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1CTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `SELP` reader - Bandwidth select P value."]
pub type SELP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELP` writer - Bandwidth select P value."]
pub type SELP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL1CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `BYPASSPLL` reader - Bypass PLL input clock is sent directly to the PLL output (default)."]
pub type BYPASSPLL_R = crate::BitReader<BYPASSPLL_A>;
#[doc = "Bypass PLL input clock is sent directly to the PLL output (default).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASSPLL_A {
    #[doc = "0: use PLL."]
    USED = 0,
    #[doc = "1: PLL input clock is sent directly to the PLL output."]
    BYPASSED = 1,
}
impl From<BYPASSPLL_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSPLL_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASSPLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASSPLL_A {
        match self.bits {
            false => BYPASSPLL_A::USED,
            true => BYPASSPLL_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == BYPASSPLL_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASSPLL_A::BYPASSED
    }
}
#[doc = "Field `BYPASSPLL` writer - Bypass PLL input clock is sent directly to the PLL output (default)."]
pub type BYPASSPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1CTRL_SPEC, BYPASSPLL_A, O>;
impl<'a, const O: u8> BYPASSPLL_W<'a, O> {
    #[doc = "use PLL."]
    #[inline(always)]
    pub fn used(self) -> &'a mut W {
        self.variant(BYPASSPLL_A::USED)
    }
    #[doc = "PLL input clock is sent directly to the PLL output."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(BYPASSPLL_A::BYPASSED)
    }
}
#[doc = "Field `BYPASSPOSTDIV2` reader - bypass of the divide-by-2 divider in the post-divider."]
pub type BYPASSPOSTDIV2_R = crate::BitReader<BYPASSPOSTDIV2_A>;
#[doc = "bypass of the divide-by-2 divider in the post-divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASSPOSTDIV2_A {
    #[doc = "0: use the divide-by-2 divider in the post-divider."]
    USED = 0,
    #[doc = "1: bypass of the divide-by-2 divider in the post-divider."]
    BYPASSED = 1,
}
impl From<BYPASSPOSTDIV2_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSPOSTDIV2_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASSPOSTDIV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASSPOSTDIV2_A {
        match self.bits {
            false => BYPASSPOSTDIV2_A::USED,
            true => BYPASSPOSTDIV2_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == BYPASSPOSTDIV2_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASSPOSTDIV2_A::BYPASSED
    }
}
#[doc = "Field `BYPASSPOSTDIV2` writer - bypass of the divide-by-2 divider in the post-divider."]
pub type BYPASSPOSTDIV2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL1CTRL_SPEC, BYPASSPOSTDIV2_A, O>;
impl<'a, const O: u8> BYPASSPOSTDIV2_W<'a, O> {
    #[doc = "use the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub fn used(self) -> &'a mut W {
        self.variant(BYPASSPOSTDIV2_A::USED)
    }
    #[doc = "bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(BYPASSPOSTDIV2_A::BYPASSED)
    }
}
#[doc = "Field `LIMUPOFF` reader - limup_off = 1 in spread spectrum and fractional PLL applications."]
pub type LIMUPOFF_R = crate::BitReader<bool>;
#[doc = "Field `LIMUPOFF` writer - limup_off = 1 in spread spectrum and fractional PLL applications."]
pub type LIMUPOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1CTRL_SPEC, bool, O>;
#[doc = "Field `BWDIRECT` reader - control of the bandwidth of the PLL."]
pub type BWDIRECT_R = crate::BitReader<BWDIRECT_A>;
#[doc = "control of the bandwidth of the PLL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWDIRECT_A {
    #[doc = "0: the bandwidth is changed synchronously with the feedback-divider."]
    SYNC = 0,
    #[doc = "1: modify the bandwidth of the PLL directly."]
    DIRECT = 1,
}
impl From<BWDIRECT_A> for bool {
    #[inline(always)]
    fn from(variant: BWDIRECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BWDIRECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWDIRECT_A {
        match self.bits {
            false => BWDIRECT_A::SYNC,
            true => BWDIRECT_A::DIRECT,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == BWDIRECT_A::SYNC
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == BWDIRECT_A::DIRECT
    }
}
#[doc = "Field `BWDIRECT` writer - control of the bandwidth of the PLL."]
pub type BWDIRECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1CTRL_SPEC, BWDIRECT_A, O>;
impl<'a, const O: u8> BWDIRECT_W<'a, O> {
    #[doc = "the bandwidth is changed synchronously with the feedback-divider."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(BWDIRECT_A::SYNC)
    }
    #[doc = "modify the bandwidth of the PLL directly."]
    #[inline(always)]
    pub fn direct(self) -> &'a mut W {
        self.variant(BWDIRECT_A::DIRECT)
    }
}
#[doc = "Field `BYPASSPREDIV` reader - bypass of the pre-divider."]
pub type BYPASSPREDIV_R = crate::BitReader<BYPASSPREDIV_A>;
#[doc = "bypass of the pre-divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASSPREDIV_A {
    #[doc = "0: use the pre-divider."]
    USED = 0,
    #[doc = "1: bypass of the pre-divider."]
    BYPASSED = 1,
}
impl From<BYPASSPREDIV_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSPREDIV_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASSPREDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASSPREDIV_A {
        match self.bits {
            false => BYPASSPREDIV_A::USED,
            true => BYPASSPREDIV_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == BYPASSPREDIV_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASSPREDIV_A::BYPASSED
    }
}
#[doc = "Field `BYPASSPREDIV` writer - bypass of the pre-divider."]
pub type BYPASSPREDIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL1CTRL_SPEC, BYPASSPREDIV_A, O>;
impl<'a, const O: u8> BYPASSPREDIV_W<'a, O> {
    #[doc = "use the pre-divider."]
    #[inline(always)]
    pub fn used(self) -> &'a mut W {
        self.variant(BYPASSPREDIV_A::USED)
    }
    #[doc = "bypass of the pre-divider."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(BYPASSPREDIV_A::BYPASSED)
    }
}
#[doc = "Field `BYPASSPOSTDIV` reader - bypass of the post-divider."]
pub type BYPASSPOSTDIV_R = crate::BitReader<BYPASSPOSTDIV_A>;
#[doc = "bypass of the post-divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASSPOSTDIV_A {
    #[doc = "0: use the post-divider."]
    USED = 0,
    #[doc = "1: bypass of the post-divider."]
    BYPASSED = 1,
}
impl From<BYPASSPOSTDIV_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASSPOSTDIV_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASSPOSTDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASSPOSTDIV_A {
        match self.bits {
            false => BYPASSPOSTDIV_A::USED,
            true => BYPASSPOSTDIV_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `USED`"]
    #[inline(always)]
    pub fn is_used(&self) -> bool {
        *self == BYPASSPOSTDIV_A::USED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == BYPASSPOSTDIV_A::BYPASSED
    }
}
#[doc = "Field `BYPASSPOSTDIV` writer - bypass of the post-divider."]
pub type BYPASSPOSTDIV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL1CTRL_SPEC, BYPASSPOSTDIV_A, O>;
impl<'a, const O: u8> BYPASSPOSTDIV_W<'a, O> {
    #[doc = "use the post-divider."]
    #[inline(always)]
    pub fn used(self) -> &'a mut W {
        self.variant(BYPASSPOSTDIV_A::USED)
    }
    #[doc = "bypass of the post-divider."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(BYPASSPOSTDIV_A::BYPASSED)
    }
}
#[doc = "Field `CLKEN` reader - enable the output clock."]
pub type CLKEN_R = crate::BitReader<CLKEN_A>;
#[doc = "enable the output clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN_A {
    #[doc = "0: Disable the output clock."]
    DISABLE = 0,
    #[doc = "1: Enable the output clock."]
    ENABLE = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::DISABLE,
            true => CLKEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLKEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLKEN_A::ENABLE
    }
}
#[doc = "Field `CLKEN` writer - enable the output clock."]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1CTRL_SPEC, CLKEN_A, O>;
impl<'a, const O: u8> CLKEN_W<'a, O> {
    #[doc = "Disable the output clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLKEN_A::DISABLE)
    }
    #[doc = "Enable the output clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLKEN_A::ENABLE)
    }
}
#[doc = "Field `FRMEN` reader - 1: free running mode."]
pub type FRMEN_R = crate::BitReader<bool>;
#[doc = "Field `FRMEN` writer - 1: free running mode."]
pub type FRMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1CTRL_SPEC, bool, O>;
#[doc = "Field `FRMCLKSTABLE` reader - free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
pub type FRMCLKSTABLE_R = crate::BitReader<bool>;
#[doc = "Field `FRMCLKSTABLE` writer - free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
pub type FRMCLKSTABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1CTRL_SPEC, bool, O>;
#[doc = "Field `SKEWEN` reader - Skew mode."]
pub type SKEWEN_R = crate::BitReader<SKEWEN_A>;
#[doc = "Skew mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SKEWEN_A {
    #[doc = "0: skewmode is disable."]
    DISABLE = 0,
    #[doc = "1: skewmode is enable."]
    ENABLE = 1,
}
impl From<SKEWEN_A> for bool {
    #[inline(always)]
    fn from(variant: SKEWEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SKEWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKEWEN_A {
        match self.bits {
            false => SKEWEN_A::DISABLE,
            true => SKEWEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SKEWEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SKEWEN_A::ENABLE
    }
}
#[doc = "Field `SKEWEN` writer - Skew mode."]
pub type SKEWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL1CTRL_SPEC, SKEWEN_A, O>;
impl<'a, const O: u8> SKEWEN_W<'a, O> {
    #[doc = "skewmode is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SKEWEN_A::DISABLE)
    }
    #[doc = "skewmode is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SKEWEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline(always)]
    pub fn selr(&self) -> SELR_R {
        SELR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline(always)]
    pub fn seli(&self) -> SELI_R {
        SELI_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:14 - Bandwidth select P value."]
    #[inline(always)]
    pub fn selp(&self) -> SELP_R {
        SELP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[inline(always)]
    pub fn bypasspll(&self) -> BYPASSPLL_R {
        BYPASSPLL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    pub fn bypasspostdiv2(&self) -> BYPASSPOSTDIV2_R {
        BYPASSPOSTDIV2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[inline(always)]
    pub fn limupoff(&self) -> LIMUPOFF_R {
        LIMUPOFF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - control of the bandwidth of the PLL."]
    #[inline(always)]
    pub fn bwdirect(&self) -> BWDIRECT_R {
        BWDIRECT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - bypass of the pre-divider."]
    #[inline(always)]
    pub fn bypassprediv(&self) -> BYPASSPREDIV_R {
        BYPASSPREDIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - bypass of the post-divider."]
    #[inline(always)]
    pub fn bypasspostdiv(&self) -> BYPASSPOSTDIV_R {
        BYPASSPOSTDIV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - enable the output clock."]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: free running mode."]
    #[inline(always)]
    pub fn frmen(&self) -> FRMEN_R {
        FRMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
    #[inline(always)]
    pub fn frmclkstable(&self) -> FRMCLKSTABLE_R {
        FRMCLKSTABLE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Skew mode."]
    #[inline(always)]
    pub fn skewen(&self) -> SKEWEN_R {
        SKEWEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline(always)]
    #[must_use]
    pub fn selr(&mut self) -> SELR_W<0> {
        SELR_W::new(self)
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline(always)]
    #[must_use]
    pub fn seli(&mut self) -> SELI_W<4> {
        SELI_W::new(self)
    }
    #[doc = "Bits 10:14 - Bandwidth select P value."]
    #[inline(always)]
    #[must_use]
    pub fn selp(&mut self) -> SELP_W<10> {
        SELP_W::new(self)
    }
    #[doc = "Bit 15 - Bypass PLL input clock is sent directly to the PLL output (default)."]
    #[inline(always)]
    #[must_use]
    pub fn bypasspll(&mut self) -> BYPASSPLL_W<15> {
        BYPASSPLL_W::new(self)
    }
    #[doc = "Bit 16 - bypass of the divide-by-2 divider in the post-divider."]
    #[inline(always)]
    #[must_use]
    pub fn bypasspostdiv2(&mut self) -> BYPASSPOSTDIV2_W<16> {
        BYPASSPOSTDIV2_W::new(self)
    }
    #[doc = "Bit 17 - limup_off = 1 in spread spectrum and fractional PLL applications."]
    #[inline(always)]
    #[must_use]
    pub fn limupoff(&mut self) -> LIMUPOFF_W<17> {
        LIMUPOFF_W::new(self)
    }
    #[doc = "Bit 18 - control of the bandwidth of the PLL."]
    #[inline(always)]
    #[must_use]
    pub fn bwdirect(&mut self) -> BWDIRECT_W<18> {
        BWDIRECT_W::new(self)
    }
    #[doc = "Bit 19 - bypass of the pre-divider."]
    #[inline(always)]
    #[must_use]
    pub fn bypassprediv(&mut self) -> BYPASSPREDIV_W<19> {
        BYPASSPREDIV_W::new(self)
    }
    #[doc = "Bit 20 - bypass of the post-divider."]
    #[inline(always)]
    #[must_use]
    pub fn bypasspostdiv(&mut self) -> BYPASSPOSTDIV_W<20> {
        BYPASSPOSTDIV_W::new(self)
    }
    #[doc = "Bit 21 - enable the output clock."]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<21> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 22 - 1: free running mode."]
    #[inline(always)]
    #[must_use]
    pub fn frmen(&mut self) -> FRMEN_W<22> {
        FRMEN_W::new(self)
    }
    #[doc = "Bit 23 - free running mode clockstable: Warning: Only make frm_clockstable = 1 after the PLL output frequency is stable."]
    #[inline(always)]
    #[must_use]
    pub fn frmclkstable(&mut self) -> FRMCLKSTABLE_W<23> {
        FRMCLKSTABLE_W::new(self)
    }
    #[doc = "Bit 24 - Skew mode."]
    #[inline(always)]
    #[must_use]
    pub fn skewen(&mut self) -> SKEWEN_W<24> {
        SKEWEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL1 550m control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1ctrl](index.html) module"]
pub struct PLL1CTRL_SPEC;
impl crate::RegisterSpec for PLL1CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll1ctrl::R](R) reader structure"]
impl crate::Readable for PLL1CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll1ctrl::W](W) writer structure"]
impl crate::Writable for PLL1CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL1CTRL to value 0"]
impl crate::Resettable for PLL1CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

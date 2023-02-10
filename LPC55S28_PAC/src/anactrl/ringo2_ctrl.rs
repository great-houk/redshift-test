#[doc = "Register `RINGO2_CTRL` reader"]
pub struct R(crate::R<RINGO2_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RINGO2_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RINGO2_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RINGO2_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RINGO2_CTRL` writer"]
pub struct W(crate::W<RINGO2_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RINGO2_CTRL_SPEC>;
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
impl From<crate::W<RINGO2_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RINGO2_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S` reader - Select short or long ringo (for all ringos types)."]
pub type S_R = crate::BitReader<S_A>;
#[doc = "Select short or long ringo (for all ringos types).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S_A {
    #[doc = "0: Select short ringo (few elements)."]
    SHORT = 0,
    #[doc = "1: Select long ringo (many elements)."]
    LONG = 1,
}
impl From<S_A> for bool {
    #[inline(always)]
    fn from(variant: S_A) -> Self {
        variant as u8 != 0
    }
}
impl S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S_A {
        match self.bits {
            false => S_A::SHORT,
            true => S_A::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == S_A::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == S_A::LONG
    }
}
#[doc = "Field `S` writer - Select short or long ringo (for all ringos types)."]
pub type S_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO2_CTRL_SPEC, S_A, O>;
impl<'a, const O: u8> S_W<'a, O> {
    #[doc = "Select short ringo (few elements)."]
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(S_A::SHORT)
    }
    #[doc = "Select long ringo (many elements)."]
    #[inline(always)]
    pub fn long(self) -> &'a mut W {
        self.variant(S_A::LONG)
    }
}
#[doc = "Field `FS` reader - Ringo frequency output divider."]
pub type FS_R = crate::BitReader<FS_A>;
#[doc = "Ringo frequency output divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FS_A {
    #[doc = "0: High frequency output (frequency lower than 100 MHz)."]
    FAST = 0,
    #[doc = "1: Low frequency output (frequency lower than 10 MHz)."]
    SLOW = 1,
}
impl From<FS_A> for bool {
    #[inline(always)]
    fn from(variant: FS_A) -> Self {
        variant as u8 != 0
    }
}
impl FS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FS_A {
        match self.bits {
            false => FS_A::FAST,
            true => FS_A::SLOW,
        }
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == FS_A::FAST
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == FS_A::SLOW
    }
}
#[doc = "Field `FS` writer - Ringo frequency output divider."]
pub type FS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO2_CTRL_SPEC, FS_A, O>;
impl<'a, const O: u8> FS_W<'a, O> {
    #[doc = "High frequency output (frequency lower than 100 MHz)."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(FS_A::FAST)
    }
    #[doc = "Low frequency output (frequency lower than 10 MHz)."]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(FS_A::SLOW)
    }
}
#[doc = "Field `PD` reader - Ringo module Power control."]
pub type PD_R = crate::BitReader<PD_A>;
#[doc = "Ringo module Power control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD_A {
    #[doc = "0: The Ringo module is enabled."]
    POWERED_ON = 0,
    #[doc = "1: The Ringo module is disabled."]
    POWERED_DOWN = 1,
}
impl From<PD_A> for bool {
    #[inline(always)]
    fn from(variant: PD_A) -> Self {
        variant as u8 != 0
    }
}
impl PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD_A {
        match self.bits {
            false => PD_A::POWERED_ON,
            true => PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED_ON`"]
    #[inline(always)]
    pub fn is_powered_on(&self) -> bool {
        *self == PD_A::POWERED_ON
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == PD_A::POWERED_DOWN
    }
}
#[doc = "Field `PD` writer - Ringo module Power control."]
pub type PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO2_CTRL_SPEC, PD_A, O>;
impl<'a, const O: u8> PD_W<'a, O> {
    #[doc = "The Ringo module is enabled."]
    #[inline(always)]
    pub fn powered_on(self) -> &'a mut W {
        self.variant(PD_A::POWERED_ON)
    }
    #[doc = "The Ringo module is disabled."]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(PD_A::POWERED_DOWN)
    }
}
#[doc = "Field `E_R24` reader - ."]
pub type E_R24_R = crate::BitReader<E_R24_A>;
#[doc = ".\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_R24_A {
    #[doc = "0: Ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Ringo is enabled."]
    ENABLE = 1,
}
impl From<E_R24_A> for bool {
    #[inline(always)]
    fn from(variant: E_R24_A) -> Self {
        variant as u8 != 0
    }
}
impl E_R24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_R24_A {
        match self.bits {
            false => E_R24_A::DISABLE,
            true => E_R24_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_R24_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_R24_A::ENABLE
    }
}
#[doc = "Field `E_R24` writer - ."]
pub type E_R24_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO2_CTRL_SPEC, E_R24_A, O>;
impl<'a, const O: u8> E_R24_W<'a, O> {
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_R24_A::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_R24_A::ENABLE)
    }
}
#[doc = "Field `E_R35` reader - ."]
pub type E_R35_R = crate::BitReader<E_R35_A>;
#[doc = ".\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_R35_A {
    #[doc = "0: Ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Ringo is enabled."]
    ENABLE = 1,
}
impl From<E_R35_A> for bool {
    #[inline(always)]
    fn from(variant: E_R35_A) -> Self {
        variant as u8 != 0
    }
}
impl E_R35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_R35_A {
        match self.bits {
            false => E_R35_A::DISABLE,
            true => E_R35_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_R35_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_R35_A::ENABLE
    }
}
#[doc = "Field `E_R35` writer - ."]
pub type E_R35_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO2_CTRL_SPEC, E_R35_A, O>;
impl<'a, const O: u8> E_R35_W<'a, O> {
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_R35_A::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_R35_A::ENABLE)
    }
}
#[doc = "Field `E_M2` reader - Metal 2 (M2) monitor control."]
pub type E_M2_R = crate::BitReader<E_M2_A>;
#[doc = "Metal 2 (M2) monitor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_M2_A {
    #[doc = "0: Ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Ringo is enabled."]
    ENABLE = 1,
}
impl From<E_M2_A> for bool {
    #[inline(always)]
    fn from(variant: E_M2_A) -> Self {
        variant as u8 != 0
    }
}
impl E_M2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_M2_A {
        match self.bits {
            false => E_M2_A::DISABLE,
            true => E_M2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_M2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_M2_A::ENABLE
    }
}
#[doc = "Field `E_M2` writer - Metal 2 (M2) monitor control."]
pub type E_M2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO2_CTRL_SPEC, E_M2_A, O>;
impl<'a, const O: u8> E_M2_W<'a, O> {
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_M2_A::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_M2_A::ENABLE)
    }
}
#[doc = "Field `E_M3` reader - Metal 3 (M3) monitor control."]
pub type E_M3_R = crate::BitReader<E_M3_A>;
#[doc = "Metal 3 (M3) monitor control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_M3_A {
    #[doc = "0: Ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Ringo is enabled."]
    ENABLE = 1,
}
impl From<E_M3_A> for bool {
    #[inline(always)]
    fn from(variant: E_M3_A) -> Self {
        variant as u8 != 0
    }
}
impl E_M3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_M3_A {
        match self.bits {
            false => E_M3_A::DISABLE,
            true => E_M3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_M3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_M3_A::ENABLE
    }
}
#[doc = "Field `E_M3` writer - Metal 3 (M3) monitor control."]
pub type E_M3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO2_CTRL_SPEC, E_M3_A, O>;
impl<'a, const O: u8> E_M3_W<'a, O> {
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_M3_A::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_M3_A::ENABLE)
    }
}
#[doc = "Field `E_M4` reader - Metal 4 (M4) monitor control."]
pub type E_M4_R = crate::BitReader<E_M4_A>;
#[doc = "Metal 4 (M4) monitor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_M4_A {
    #[doc = "0: Ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Ringo is enabled."]
    ENABLE = 1,
}
impl From<E_M4_A> for bool {
    #[inline(always)]
    fn from(variant: E_M4_A) -> Self {
        variant as u8 != 0
    }
}
impl E_M4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_M4_A {
        match self.bits {
            false => E_M4_A::DISABLE,
            true => E_M4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_M4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_M4_A::ENABLE
    }
}
#[doc = "Field `E_M4` writer - Metal 4 (M4) monitor control."]
pub type E_M4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO2_CTRL_SPEC, E_M4_A, O>;
impl<'a, const O: u8> E_M4_W<'a, O> {
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_M4_A::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_M4_A::ENABLE)
    }
}
#[doc = "Field `E_M5` reader - Metal 5 (M5) monitor control."]
pub type E_M5_R = crate::BitReader<E_M5_A>;
#[doc = "Metal 5 (M5) monitor control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum E_M5_A {
    #[doc = "0: Ringo is disabled."]
    DISABLE = 0,
    #[doc = "1: Ringo is enabled."]
    ENABLE = 1,
}
impl From<E_M5_A> for bool {
    #[inline(always)]
    fn from(variant: E_M5_A) -> Self {
        variant as u8 != 0
    }
}
impl E_M5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> E_M5_A {
        match self.bits {
            false => E_M5_A::DISABLE,
            true => E_M5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == E_M5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == E_M5_A::ENABLE
    }
}
#[doc = "Field `E_M5` writer - Metal 5 (M5) monitor control."]
pub type E_M5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RINGO2_CTRL_SPEC, E_M5_A, O>;
impl<'a, const O: u8> E_M5_W<'a, O> {
    #[doc = "Ringo is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(E_M5_A::DISABLE)
    }
    #[doc = "Ringo is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(E_M5_A::ENABLE)
    }
}
#[doc = "Field `DIVISOR` reader - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
pub type DIVISOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVISOR` writer - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
pub type DIVISOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RINGO2_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `DIV_UPDATE_REQ` reader - Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
pub type DIV_UPDATE_REQ_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ringo module Power control."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn e_r24(&self) -> E_R24_R {
        E_R24_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ."]
    #[inline(always)]
    pub fn e_r35(&self) -> E_R35_R {
        E_R35_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Metal 2 (M2) monitor control."]
    #[inline(always)]
    pub fn e_m2(&self) -> E_M2_R {
        E_M2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Metal 3 (M3) monitor control."]
    #[inline(always)]
    pub fn e_m3(&self) -> E_M3_R {
        E_M3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Metal 4 (M4) monitor control."]
    #[inline(always)]
    pub fn e_m4(&self) -> E_M4_R {
        E_M4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Metal 5 (M5) monitor control."]
    #[inline(always)]
    pub fn e_m5(&self) -> E_M5_R {
        E_M5_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline(always)]
    pub fn divisor(&self) -> DIVISOR_R {
        DIVISOR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn div_update_req(&self) -> DIV_UPDATE_REQ_R {
        DIV_UPDATE_REQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<0> {
        S_W::new(self)
    }
    #[doc = "Bit 1 - Ringo frequency output divider."]
    #[inline(always)]
    #[must_use]
    pub fn fs(&mut self) -> FS_W<1> {
        FS_W::new(self)
    }
    #[doc = "Bit 2 - Ringo module Power control."]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<2> {
        PD_W::new(self)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    #[must_use]
    pub fn e_r24(&mut self) -> E_R24_W<3> {
        E_R24_W::new(self)
    }
    #[doc = "Bit 4 - ."]
    #[inline(always)]
    #[must_use]
    pub fn e_r35(&mut self) -> E_R35_W<4> {
        E_R35_W::new(self)
    }
    #[doc = "Bit 5 - Metal 2 (M2) monitor control."]
    #[inline(always)]
    #[must_use]
    pub fn e_m2(&mut self) -> E_M2_W<5> {
        E_M2_W::new(self)
    }
    #[doc = "Bit 6 - Metal 3 (M3) monitor control."]
    #[inline(always)]
    #[must_use]
    pub fn e_m3(&mut self) -> E_M3_W<6> {
        E_M3_W::new(self)
    }
    #[doc = "Bit 7 - Metal 4 (M4) monitor control."]
    #[inline(always)]
    #[must_use]
    pub fn e_m4(&mut self) -> E_M4_W<7> {
        E_M4_W::new(self)
    }
    #[doc = "Bit 8 - Metal 5 (M5) monitor control."]
    #[inline(always)]
    #[must_use]
    pub fn e_m5(&mut self) -> E_M5_W<8> {
        E_M5_W::new(self)
    }
    #[doc = "Bits 16:19 - Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline(always)]
    #[must_use]
    pub fn divisor(&mut self) -> DIVISOR_W<16> {
        DIVISOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Third Ring Oscillator module control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ringo2_ctrl](index.html) module"]
pub struct RINGO2_CTRL_SPEC;
impl crate::RegisterSpec for RINGO2_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ringo2_ctrl::R](R) reader structure"]
impl crate::Readable for RINGO2_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ringo2_ctrl::W](W) writer structure"]
impl crate::Writable for RINGO2_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RINGO2_CTRL to value 0x40"]
impl crate::Resettable for RINGO2_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}

#[doc = "Register `SDIOCLKCTRL` reader"]
pub struct R(crate::R<SDIOCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIOCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIOCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIOCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIOCLKCTRL` writer"]
pub struct W(crate::W<SDIOCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIOCLKCTRL_SPEC>;
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
impl From<crate::W<SDIOCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIOCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCLK_DRV_PHASE` reader - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
pub type CCLK_DRV_PHASE_R = crate::FieldReader<u8, CCLK_DRV_PHASE_A>;
#[doc = "Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCLK_DRV_PHASE_A {
    #[doc = "0: 0 degree shift."]
    ENUM_0_DEG = 0,
    #[doc = "1: 90 degree shift."]
    ENUM_90_DEG = 1,
    #[doc = "2: 180 degree shift."]
    ENUM_180_DEG = 2,
    #[doc = "3: 270 degree shift."]
    ENUM_270_DEG = 3,
}
impl From<CCLK_DRV_PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: CCLK_DRV_PHASE_A) -> Self {
        variant as _
    }
}
impl CCLK_DRV_PHASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLK_DRV_PHASE_A {
        match self.bits {
            0 => CCLK_DRV_PHASE_A::ENUM_0_DEG,
            1 => CCLK_DRV_PHASE_A::ENUM_90_DEG,
            2 => CCLK_DRV_PHASE_A::ENUM_180_DEG,
            3 => CCLK_DRV_PHASE_A::ENUM_270_DEG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_0_DEG`"]
    #[inline(always)]
    pub fn is_enum_0_deg(&self) -> bool {
        *self == CCLK_DRV_PHASE_A::ENUM_0_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_90_DEG`"]
    #[inline(always)]
    pub fn is_enum_90_deg(&self) -> bool {
        *self == CCLK_DRV_PHASE_A::ENUM_90_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_180_DEG`"]
    #[inline(always)]
    pub fn is_enum_180_deg(&self) -> bool {
        *self == CCLK_DRV_PHASE_A::ENUM_180_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_270_DEG`"]
    #[inline(always)]
    pub fn is_enum_270_deg(&self) -> bool {
        *self == CCLK_DRV_PHASE_A::ENUM_270_DEG
    }
}
#[doc = "Field `CCLK_DRV_PHASE` writer - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
pub type CCLK_DRV_PHASE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SDIOCLKCTRL_SPEC, u8, CCLK_DRV_PHASE_A, 2, O>;
impl<'a, const O: u8> CCLK_DRV_PHASE_W<'a, O> {
    #[doc = "0 degree shift."]
    #[inline(always)]
    pub fn enum_0_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASE_A::ENUM_0_DEG)
    }
    #[doc = "90 degree shift."]
    #[inline(always)]
    pub fn enum_90_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASE_A::ENUM_90_DEG)
    }
    #[doc = "180 degree shift."]
    #[inline(always)]
    pub fn enum_180_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASE_A::ENUM_180_DEG)
    }
    #[doc = "270 degree shift."]
    #[inline(always)]
    pub fn enum_270_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASE_A::ENUM_270_DEG)
    }
}
#[doc = "Field `CCLK_SAMPLE_PHASE` reader - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub type CCLK_SAMPLE_PHASE_R = crate::FieldReader<u8, CCLK_SAMPLE_PHASE_A>;
#[doc = "Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCLK_SAMPLE_PHASE_A {
    #[doc = "0: 0 degree shift."]
    ENUM_0_DEG = 0,
    #[doc = "1: 90 degree shift."]
    ENUM_90_DEG = 1,
    #[doc = "2: 180 degree shift."]
    ENUM_180_DEG = 2,
    #[doc = "3: 270 degree shift."]
    ENUM_270_DEG = 3,
}
impl From<CCLK_SAMPLE_PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: CCLK_SAMPLE_PHASE_A) -> Self {
        variant as _
    }
}
impl CCLK_SAMPLE_PHASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLK_SAMPLE_PHASE_A {
        match self.bits {
            0 => CCLK_SAMPLE_PHASE_A::ENUM_0_DEG,
            1 => CCLK_SAMPLE_PHASE_A::ENUM_90_DEG,
            2 => CCLK_SAMPLE_PHASE_A::ENUM_180_DEG,
            3 => CCLK_SAMPLE_PHASE_A::ENUM_270_DEG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_0_DEG`"]
    #[inline(always)]
    pub fn is_enum_0_deg(&self) -> bool {
        *self == CCLK_SAMPLE_PHASE_A::ENUM_0_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_90_DEG`"]
    #[inline(always)]
    pub fn is_enum_90_deg(&self) -> bool {
        *self == CCLK_SAMPLE_PHASE_A::ENUM_90_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_180_DEG`"]
    #[inline(always)]
    pub fn is_enum_180_deg(&self) -> bool {
        *self == CCLK_SAMPLE_PHASE_A::ENUM_180_DEG
    }
    #[doc = "Checks if the value of the field is `ENUM_270_DEG`"]
    #[inline(always)]
    pub fn is_enum_270_deg(&self) -> bool {
        *self == CCLK_SAMPLE_PHASE_A::ENUM_270_DEG
    }
}
#[doc = "Field `CCLK_SAMPLE_PHASE` writer - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub type CCLK_SAMPLE_PHASE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SDIOCLKCTRL_SPEC, u8, CCLK_SAMPLE_PHASE_A, 2, O>;
impl<'a, const O: u8> CCLK_SAMPLE_PHASE_W<'a, O> {
    #[doc = "0 degree shift."]
    #[inline(always)]
    pub fn enum_0_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASE_A::ENUM_0_DEG)
    }
    #[doc = "90 degree shift."]
    #[inline(always)]
    pub fn enum_90_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASE_A::ENUM_90_DEG)
    }
    #[doc = "180 degree shift."]
    #[inline(always)]
    pub fn enum_180_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASE_A::ENUM_180_DEG)
    }
    #[doc = "270 degree shift."]
    #[inline(always)]
    pub fn enum_270_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASE_A::ENUM_270_DEG)
    }
}
#[doc = "Field `PHASE_ACTIVE` reader - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
pub type PHASE_ACTIVE_R = crate::BitReader<PHASE_ACTIVE_A>;
#[doc = "Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHASE_ACTIVE_A {
    #[doc = "0: Bypassed."]
    BYPASSED = 0,
    #[doc = "1: Activates phase shift logic. When active, the clock divider is active and phase delays are enabled."]
    PH_SHIFT = 1,
}
impl From<PHASE_ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: PHASE_ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl PHASE_ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHASE_ACTIVE_A {
        match self.bits {
            false => PHASE_ACTIVE_A::BYPASSED,
            true => PHASE_ACTIVE_A::PH_SHIFT,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == PHASE_ACTIVE_A::BYPASSED
    }
    #[doc = "Checks if the value of the field is `PH_SHIFT`"]
    #[inline(always)]
    pub fn is_ph_shift(&self) -> bool {
        *self == PHASE_ACTIVE_A::PH_SHIFT
    }
}
#[doc = "Field `PHASE_ACTIVE` writer - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
pub type PHASE_ACTIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SDIOCLKCTRL_SPEC, PHASE_ACTIVE_A, O>;
impl<'a, const O: u8> PHASE_ACTIVE_W<'a, O> {
    #[doc = "Bypassed."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(PHASE_ACTIVE_A::BYPASSED)
    }
    #[doc = "Activates phase shift logic. When active, the clock divider is active and phase delays are enabled."]
    #[inline(always)]
    pub fn ph_shift(self) -> &'a mut W {
        self.variant(PHASE_ACTIVE_A::PH_SHIFT)
    }
}
#[doc = "Field `CCLK_DRV_DELAY` reader - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
pub type CCLK_DRV_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCLK_DRV_DELAY` writer - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
pub type CCLK_DRV_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDIOCLKCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `CCLK_DRV_DELAY_ACTIVE` reader - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
pub type CCLK_DRV_DELAY_ACTIVE_R = crate::BitReader<CCLK_DRV_DELAY_ACTIVE_A>;
#[doc = "Enables drive delay, as controlled by the CCLK_DRV_DELAY field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLK_DRV_DELAY_ACTIVE_A {
    #[doc = "0: Disable drive delay."]
    DISABLE = 0,
    #[doc = "1: Enable drive delay."]
    ENABLE = 1,
}
impl From<CCLK_DRV_DELAY_ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: CCLK_DRV_DELAY_ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCLK_DRV_DELAY_ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLK_DRV_DELAY_ACTIVE_A {
        match self.bits {
            false => CCLK_DRV_DELAY_ACTIVE_A::DISABLE,
            true => CCLK_DRV_DELAY_ACTIVE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CCLK_DRV_DELAY_ACTIVE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CCLK_DRV_DELAY_ACTIVE_A::ENABLE
    }
}
#[doc = "Field `CCLK_DRV_DELAY_ACTIVE` writer - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
pub type CCLK_DRV_DELAY_ACTIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SDIOCLKCTRL_SPEC, CCLK_DRV_DELAY_ACTIVE_A, O>;
impl<'a, const O: u8> CCLK_DRV_DELAY_ACTIVE_W<'a, O> {
    #[doc = "Disable drive delay."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CCLK_DRV_DELAY_ACTIVE_A::DISABLE)
    }
    #[doc = "Enable drive delay."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CCLK_DRV_DELAY_ACTIVE_A::ENABLE)
    }
}
#[doc = "Field `CCLK_SAMPLE_DELAY` reader - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub type CCLK_SAMPLE_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCLK_SAMPLE_DELAY` writer - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub type CCLK_SAMPLE_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SDIOCLKCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `CCLK_SAMPLE_DELAY_ACTIVE` reader - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
pub type CCLK_SAMPLE_DELAY_ACTIVE_R = crate::BitReader<CCLK_SAMPLE_DELAY_ACTIVE_A>;
#[doc = "Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCLK_SAMPLE_DELAY_ACTIVE_A {
    #[doc = "0: Disables sample delay."]
    DISABLE = 0,
    #[doc = "1: Enables sample delay."]
    ENABLE = 1,
}
impl From<CCLK_SAMPLE_DELAY_ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: CCLK_SAMPLE_DELAY_ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCLK_SAMPLE_DELAY_ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLK_SAMPLE_DELAY_ACTIVE_A {
        match self.bits {
            false => CCLK_SAMPLE_DELAY_ACTIVE_A::DISABLE,
            true => CCLK_SAMPLE_DELAY_ACTIVE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CCLK_SAMPLE_DELAY_ACTIVE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CCLK_SAMPLE_DELAY_ACTIVE_A::ENABLE
    }
}
#[doc = "Field `CCLK_SAMPLE_DELAY_ACTIVE` writer - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
pub type CCLK_SAMPLE_DELAY_ACTIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SDIOCLKCTRL_SPEC, CCLK_SAMPLE_DELAY_ACTIVE_A, O>;
impl<'a, const O: u8> CCLK_SAMPLE_DELAY_ACTIVE_W<'a, O> {
    #[doc = "Disables sample delay."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_DELAY_ACTIVE_A::DISABLE)
    }
    #[doc = "Enables sample delay."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_DELAY_ACTIVE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_phase(&self) -> CCLK_DRV_PHASE_R {
        CCLK_DRV_PHASE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_phase(&self) -> CCLK_SAMPLE_PHASE_R {
        CCLK_SAMPLE_PHASE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
    #[inline(always)]
    pub fn phase_active(&self) -> PHASE_ACTIVE_R {
        PHASE_ACTIVE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_delay(&self) -> CCLK_DRV_DELAY_R {
        CCLK_DRV_DELAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline(always)]
    pub fn cclk_drv_delay_active(&self) -> CCLK_DRV_DELAY_ACTIVE_R {
        CCLK_DRV_DELAY_ACTIVE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_delay(&self) -> CCLK_SAMPLE_DELAY_R {
        CCLK_SAMPLE_DELAY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline(always)]
    pub fn cclk_sample_delay_active(&self) -> CCLK_SAMPLE_DELAY_ACTIVE_R {
        CCLK_SAMPLE_DELAY_ACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline(always)]
    #[must_use]
    pub fn cclk_drv_phase(&mut self) -> CCLK_DRV_PHASE_W<0> {
        CCLK_DRV_PHASE_W::new(self)
    }
    #[doc = "Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    #[must_use]
    pub fn cclk_sample_phase(&mut self) -> CCLK_SAMPLE_PHASE_W<2> {
        CCLK_SAMPLE_PHASE_W::new(self)
    }
    #[doc = "Bit 7 - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
    #[inline(always)]
    #[must_use]
    pub fn phase_active(&mut self) -> PHASE_ACTIVE_W<7> {
        PHASE_ACTIVE_W::new(self)
    }
    #[doc = "Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline(always)]
    #[must_use]
    pub fn cclk_drv_delay(&mut self) -> CCLK_DRV_DELAY_W<16> {
        CCLK_DRV_DELAY_W::new(self)
    }
    #[doc = "Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline(always)]
    #[must_use]
    pub fn cclk_drv_delay_active(&mut self) -> CCLK_DRV_DELAY_ACTIVE_W<23> {
        CCLK_DRV_DELAY_ACTIVE_W::new(self)
    }
    #[doc = "Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    #[must_use]
    pub fn cclk_sample_delay(&mut self) -> CCLK_SAMPLE_DELAY_W<24> {
        CCLK_SAMPLE_DELAY_W::new(self)
    }
    #[doc = "Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline(always)]
    #[must_use]
    pub fn cclk_sample_delay_active(&mut self) -> CCLK_SAMPLE_DELAY_ACTIVE_W<31> {
        CCLK_SAMPLE_DELAY_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO CCLKIN phase and delay control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdioclkctrl](index.html) module"]
pub struct SDIOCLKCTRL_SPEC;
impl crate::RegisterSpec for SDIOCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdioclkctrl::R](R) reader structure"]
impl crate::Readable for SDIOCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdioclkctrl::W](W) writer structure"]
impl crate::Writable for SDIOCLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIOCLKCTRL to value 0"]
impl crate::Resettable for SDIOCLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

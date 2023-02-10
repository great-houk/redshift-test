#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTPENDINGEN` reader - Master Pending interrupt Enable."]
pub type MSTPENDINGEN_R = crate::BitReader<MSTPENDINGEN_A>;
#[doc = "Master Pending interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTPENDINGEN_A {
    #[doc = "0: Disabled. The MstPending interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The MstPending interrupt is enabled."]
    ENABLED = 1,
}
impl From<MSTPENDINGEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSTPENDINGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTPENDINGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTPENDINGEN_A {
        match self.bits {
            false => MSTPENDINGEN_A::DISABLED,
            true => MSTPENDINGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTPENDINGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTPENDINGEN_A::ENABLED
    }
}
#[doc = "Field `MSTPENDINGEN` writer - Master Pending interrupt Enable."]
pub type MSTPENDINGEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, MSTPENDINGEN_A, O>;
impl<'a, const O: u8> MSTPENDINGEN_W<'a, O> {
    #[doc = "Disabled. The MstPending interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTPENDINGEN_A::DISABLED)
    }
    #[doc = "Enabled. The MstPending interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTPENDINGEN_A::ENABLED)
    }
}
#[doc = "Field `MSTARBLOSSEN` reader - Master Arbitration Loss interrupt Enable."]
pub type MSTARBLOSSEN_R = crate::BitReader<MSTARBLOSSEN_A>;
#[doc = "Master Arbitration Loss interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTARBLOSSEN_A {
    #[doc = "0: Disabled. The MstArbLoss interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The MstArbLoss interrupt is enabled."]
    ENABLED = 1,
}
impl From<MSTARBLOSSEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSTARBLOSSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTARBLOSSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTARBLOSSEN_A {
        match self.bits {
            false => MSTARBLOSSEN_A::DISABLED,
            true => MSTARBLOSSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTARBLOSSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTARBLOSSEN_A::ENABLED
    }
}
#[doc = "Field `MSTARBLOSSEN` writer - Master Arbitration Loss interrupt Enable."]
pub type MSTARBLOSSEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, MSTARBLOSSEN_A, O>;
impl<'a, const O: u8> MSTARBLOSSEN_W<'a, O> {
    #[doc = "Disabled. The MstArbLoss interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTARBLOSSEN_A::DISABLED)
    }
    #[doc = "Enabled. The MstArbLoss interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTARBLOSSEN_A::ENABLED)
    }
}
#[doc = "Field `MSTSTSTPERREN` reader - Master Start/Stop Error interrupt Enable."]
pub type MSTSTSTPERREN_R = crate::BitReader<MSTSTSTPERREN_A>;
#[doc = "Master Start/Stop Error interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTSTSTPERREN_A {
    #[doc = "0: Disabled. The MstStStpErr interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The MstStStpErr interrupt is enabled."]
    ENABLED = 1,
}
impl From<MSTSTSTPERREN_A> for bool {
    #[inline(always)]
    fn from(variant: MSTSTSTPERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTSTSTPERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSTSTPERREN_A {
        match self.bits {
            false => MSTSTSTPERREN_A::DISABLED,
            true => MSTSTSTPERREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTSTSTPERREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTSTSTPERREN_A::ENABLED
    }
}
#[doc = "Field `MSTSTSTPERREN` writer - Master Start/Stop Error interrupt Enable."]
pub type MSTSTSTPERREN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, MSTSTSTPERREN_A, O>;
impl<'a, const O: u8> MSTSTSTPERREN_W<'a, O> {
    #[doc = "Disabled. The MstStStpErr interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTSTSTPERREN_A::DISABLED)
    }
    #[doc = "Enabled. The MstStStpErr interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTSTSTPERREN_A::ENABLED)
    }
}
#[doc = "Field `SLVPENDINGEN` reader - Slave Pending interrupt Enable."]
pub type SLVPENDINGEN_R = crate::BitReader<SLVPENDINGEN_A>;
#[doc = "Slave Pending interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVPENDINGEN_A {
    #[doc = "0: Disabled. The SlvPending interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The SlvPending interrupt is enabled."]
    ENABLED = 1,
}
impl From<SLVPENDINGEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVPENDINGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SLVPENDINGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVPENDINGEN_A {
        match self.bits {
            false => SLVPENDINGEN_A::DISABLED,
            true => SLVPENDINGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVPENDINGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVPENDINGEN_A::ENABLED
    }
}
#[doc = "Field `SLVPENDINGEN` writer - Slave Pending interrupt Enable."]
pub type SLVPENDINGEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, SLVPENDINGEN_A, O>;
impl<'a, const O: u8> SLVPENDINGEN_W<'a, O> {
    #[doc = "Disabled. The SlvPending interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVPENDINGEN_A::DISABLED)
    }
    #[doc = "Enabled. The SlvPending interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVPENDINGEN_A::ENABLED)
    }
}
#[doc = "Field `SLVNOTSTREN` reader - Slave Not Stretching interrupt Enable."]
pub type SLVNOTSTREN_R = crate::BitReader<SLVNOTSTREN_A>;
#[doc = "Slave Not Stretching interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVNOTSTREN_A {
    #[doc = "0: Disabled. The SlvNotStr interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The SlvNotStr interrupt is enabled."]
    ENABLED = 1,
}
impl From<SLVNOTSTREN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVNOTSTREN_A) -> Self {
        variant as u8 != 0
    }
}
impl SLVNOTSTREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVNOTSTREN_A {
        match self.bits {
            false => SLVNOTSTREN_A::DISABLED,
            true => SLVNOTSTREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVNOTSTREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVNOTSTREN_A::ENABLED
    }
}
#[doc = "Field `SLVNOTSTREN` writer - Slave Not Stretching interrupt Enable."]
pub type SLVNOTSTREN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, SLVNOTSTREN_A, O>;
impl<'a, const O: u8> SLVNOTSTREN_W<'a, O> {
    #[doc = "Disabled. The SlvNotStr interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVNOTSTREN_A::DISABLED)
    }
    #[doc = "Enabled. The SlvNotStr interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVNOTSTREN_A::ENABLED)
    }
}
#[doc = "Field `SLVDESELEN` reader - Slave Deselect interrupt Enable."]
pub type SLVDESELEN_R = crate::BitReader<SLVDESELEN_A>;
#[doc = "Slave Deselect interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVDESELEN_A {
    #[doc = "0: Disabled. The SlvDeSel interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The SlvDeSel interrupt is enabled."]
    ENABLED = 1,
}
impl From<SLVDESELEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVDESELEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SLVDESELEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVDESELEN_A {
        match self.bits {
            false => SLVDESELEN_A::DISABLED,
            true => SLVDESELEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVDESELEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVDESELEN_A::ENABLED
    }
}
#[doc = "Field `SLVDESELEN` writer - Slave Deselect interrupt Enable."]
pub type SLVDESELEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, SLVDESELEN_A, O>;
impl<'a, const O: u8> SLVDESELEN_W<'a, O> {
    #[doc = "Disabled. The SlvDeSel interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVDESELEN_A::DISABLED)
    }
    #[doc = "Enabled. The SlvDeSel interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVDESELEN_A::ENABLED)
    }
}
#[doc = "Field `MONRDYEN` reader - Monitor data Ready interrupt Enable."]
pub type MONRDYEN_R = crate::BitReader<MONRDYEN_A>;
#[doc = "Monitor data Ready interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONRDYEN_A {
    #[doc = "0: Disabled. The MonRdy interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The MonRdy interrupt is enabled."]
    ENABLED = 1,
}
impl From<MONRDYEN_A> for bool {
    #[inline(always)]
    fn from(variant: MONRDYEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MONRDYEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONRDYEN_A {
        match self.bits {
            false => MONRDYEN_A::DISABLED,
            true => MONRDYEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONRDYEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONRDYEN_A::ENABLED
    }
}
#[doc = "Field `MONRDYEN` writer - Monitor data Ready interrupt Enable."]
pub type MONRDYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, MONRDYEN_A, O>;
impl<'a, const O: u8> MONRDYEN_W<'a, O> {
    #[doc = "Disabled. The MonRdy interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONRDYEN_A::DISABLED)
    }
    #[doc = "Enabled. The MonRdy interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONRDYEN_A::ENABLED)
    }
}
#[doc = "Field `MONOVEN` reader - Monitor Overrun interrupt Enable."]
pub type MONOVEN_R = crate::BitReader<MONOVEN_A>;
#[doc = "Monitor Overrun interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONOVEN_A {
    #[doc = "0: Disabled. The MonOv interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The MonOv interrupt is enabled."]
    ENABLED = 1,
}
impl From<MONOVEN_A> for bool {
    #[inline(always)]
    fn from(variant: MONOVEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MONOVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONOVEN_A {
        match self.bits {
            false => MONOVEN_A::DISABLED,
            true => MONOVEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONOVEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONOVEN_A::ENABLED
    }
}
#[doc = "Field `MONOVEN` writer - Monitor Overrun interrupt Enable."]
pub type MONOVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, MONOVEN_A, O>;
impl<'a, const O: u8> MONOVEN_W<'a, O> {
    #[doc = "Disabled. The MonOv interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONOVEN_A::DISABLED)
    }
    #[doc = "Enabled. The MonOv interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONOVEN_A::ENABLED)
    }
}
#[doc = "Field `MONIDLEEN` reader - Monitor Idle interrupt Enable."]
pub type MONIDLEEN_R = crate::BitReader<MONIDLEEN_A>;
#[doc = "Monitor Idle interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONIDLEEN_A {
    #[doc = "0: Disabled. The MonIdle interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The MonIdle interrupt is enabled."]
    ENABLED = 1,
}
impl From<MONIDLEEN_A> for bool {
    #[inline(always)]
    fn from(variant: MONIDLEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MONIDLEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONIDLEEN_A {
        match self.bits {
            false => MONIDLEEN_A::DISABLED,
            true => MONIDLEEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONIDLEEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONIDLEEN_A::ENABLED
    }
}
#[doc = "Field `MONIDLEEN` writer - Monitor Idle interrupt Enable."]
pub type MONIDLEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, MONIDLEEN_A, O>;
impl<'a, const O: u8> MONIDLEEN_W<'a, O> {
    #[doc = "Disabled. The MonIdle interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONIDLEEN_A::DISABLED)
    }
    #[doc = "Enabled. The MonIdle interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONIDLEEN_A::ENABLED)
    }
}
#[doc = "Field `EVENTTIMEOUTEN` reader - Event time-out interrupt Enable."]
pub type EVENTTIMEOUTEN_R = crate::BitReader<EVENTTIMEOUTEN_A>;
#[doc = "Event time-out interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTTIMEOUTEN_A {
    #[doc = "0: Disabled. The Event time-out interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The Event time-out interrupt is enabled."]
    ENABLED = 1,
}
impl From<EVENTTIMEOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTTIMEOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTTIMEOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTTIMEOUTEN_A {
        match self.bits {
            false => EVENTTIMEOUTEN_A::DISABLED,
            true => EVENTTIMEOUTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EVENTTIMEOUTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EVENTTIMEOUTEN_A::ENABLED
    }
}
#[doc = "Field `EVENTTIMEOUTEN` writer - Event time-out interrupt Enable."]
pub type EVENTTIMEOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, EVENTTIMEOUTEN_A, O>;
impl<'a, const O: u8> EVENTTIMEOUTEN_W<'a, O> {
    #[doc = "Disabled. The Event time-out interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTEN_A::DISABLED)
    }
    #[doc = "Enabled. The Event time-out interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EVENTTIMEOUTEN_A::ENABLED)
    }
}
#[doc = "Field `SCLTIMEOUTEN` reader - SCL time-out interrupt Enable."]
pub type SCLTIMEOUTEN_R = crate::BitReader<SCLTIMEOUTEN_A>;
#[doc = "SCL time-out interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLTIMEOUTEN_A {
    #[doc = "0: Disabled. The SCL time-out interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The SCL time-out interrupt is enabled."]
    ENABLED = 1,
}
impl From<SCLTIMEOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCLTIMEOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCLTIMEOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLTIMEOUTEN_A {
        match self.bits {
            false => SCLTIMEOUTEN_A::DISABLED,
            true => SCLTIMEOUTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCLTIMEOUTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCLTIMEOUTEN_A::ENABLED
    }
}
#[doc = "Field `SCLTIMEOUTEN` writer - SCL time-out interrupt Enable."]
pub type SCLTIMEOUTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, SCLTIMEOUTEN_A, O>;
impl<'a, const O: u8> SCLTIMEOUTEN_W<'a, O> {
    #[doc = "Disabled. The SCL time-out interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCLTIMEOUTEN_A::DISABLED)
    }
    #[doc = "Enabled. The SCL time-out interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCLTIMEOUTEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Master Pending interrupt Enable."]
    #[inline(always)]
    pub fn mstpendingen(&self) -> MSTPENDINGEN_R {
        MSTPENDINGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt Enable."]
    #[inline(always)]
    pub fn mstarblossen(&self) -> MSTARBLOSSEN_R {
        MSTARBLOSSEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt Enable."]
    #[inline(always)]
    pub fn mstststperren(&self) -> MSTSTSTPERREN_R {
        MSTSTSTPERREN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave Pending interrupt Enable."]
    #[inline(always)]
    pub fn slvpendingen(&self) -> SLVPENDINGEN_R {
        SLVPENDINGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt Enable."]
    #[inline(always)]
    pub fn slvnotstren(&self) -> SLVNOTSTREN_R {
        SLVNOTSTREN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Slave Deselect interrupt Enable."]
    #[inline(always)]
    pub fn slvdeselen(&self) -> SLVDESELEN_R {
        SLVDESELEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt Enable."]
    #[inline(always)]
    pub fn monrdyen(&self) -> MONRDYEN_R {
        MONRDYEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt Enable."]
    #[inline(always)]
    pub fn monoven(&self) -> MONOVEN_R {
        MONOVEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Monitor Idle interrupt Enable."]
    #[inline(always)]
    pub fn monidleen(&self) -> MONIDLEEN_R {
        MONIDLEEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Event time-out interrupt Enable."]
    #[inline(always)]
    pub fn eventtimeouten(&self) -> EVENTTIMEOUTEN_R {
        EVENTTIMEOUTEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCL time-out interrupt Enable."]
    #[inline(always)]
    pub fn scltimeouten(&self) -> SCLTIMEOUTEN_R {
        SCLTIMEOUTEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Pending interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn mstpendingen(&mut self) -> MSTPENDINGEN_W<0> {
        MSTPENDINGEN_W::new(self)
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn mstarblossen(&mut self) -> MSTARBLOSSEN_W<4> {
        MSTARBLOSSEN_W::new(self)
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn mstststperren(&mut self) -> MSTSTSTPERREN_W<6> {
        MSTSTSTPERREN_W::new(self)
    }
    #[doc = "Bit 8 - Slave Pending interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn slvpendingen(&mut self) -> SLVPENDINGEN_W<8> {
        SLVPENDINGEN_W::new(self)
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn slvnotstren(&mut self) -> SLVNOTSTREN_W<11> {
        SLVNOTSTREN_W::new(self)
    }
    #[doc = "Bit 15 - Slave Deselect interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn slvdeselen(&mut self) -> SLVDESELEN_W<15> {
        SLVDESELEN_W::new(self)
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn monrdyen(&mut self) -> MONRDYEN_W<16> {
        MONRDYEN_W::new(self)
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn monoven(&mut self) -> MONOVEN_W<17> {
        MONOVEN_W::new(self)
    }
    #[doc = "Bit 19 - Monitor Idle interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn monidleen(&mut self) -> MONIDLEEN_W<19> {
        MONIDLEEN_W::new(self)
    }
    #[doc = "Bit 24 - Event time-out interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn eventtimeouten(&mut self) -> EVENTTIMEOUTEN_W<24> {
        EVENTTIMEOUTEN_W::new(self)
    }
    #[doc = "Bit 25 - SCL time-out interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn scltimeouten(&mut self) -> SCLTIMEOUTEN_W<25> {
        SCLTIMEOUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Set and read register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

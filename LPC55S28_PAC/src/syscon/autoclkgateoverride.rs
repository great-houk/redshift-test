#[doc = "Register `AUTOCLKGATEOVERRIDE` reader"]
pub struct R(crate::R<AUTOCLKGATEOVERRIDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOCLKGATEOVERRIDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOCLKGATEOVERRIDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOCLKGATEOVERRIDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOCLKGATEOVERRIDE` writer"]
pub struct W(crate::W<AUTOCLKGATEOVERRIDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOCLKGATEOVERRIDE_SPEC>;
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
impl From<crate::W<AUTOCLKGATEOVERRIDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOCLKGATEOVERRIDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM` reader - Control automatic clock gating of ROM controller."]
pub type ROM_R = crate::BitReader<ROM_A>;
#[doc = "Control automatic clock gating of ROM controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROM_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<ROM_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_A) -> Self {
        variant as u8 != 0
    }
}
impl ROM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_A {
        match self.bits {
            false => ROM_A::DISABLE,
            true => ROM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ROM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ROM_A::ENABLE
    }
}
#[doc = "Field `ROM` writer - Control automatic clock gating of ROM controller."]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, ROM_A, O>;
impl<'a, const O: u8> ROM_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROM_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROM_A::ENABLE)
    }
}
#[doc = "Field `RAMX_CTRL` reader - Control automatic clock gating of RAMX controller."]
pub type RAMX_CTRL_R = crate::BitReader<RAMX_CTRL_A>;
#[doc = "Control automatic clock gating of RAMX controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMX_CTRL_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<RAMX_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RAMX_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMX_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMX_CTRL_A {
        match self.bits {
            false => RAMX_CTRL_A::DISABLE,
            true => RAMX_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAMX_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAMX_CTRL_A::ENABLE
    }
}
#[doc = "Field `RAMX_CTRL` writer - Control automatic clock gating of RAMX controller."]
pub type RAMX_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, RAMX_CTRL_A, O>;
impl<'a, const O: u8> RAMX_CTRL_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAMX_CTRL_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAMX_CTRL_A::ENABLE)
    }
}
#[doc = "Field `RAM0_CTRL` reader - Control automatic clock gating of RAM0 controller."]
pub type RAM0_CTRL_R = crate::BitReader<RAM0_CTRL_A>;
#[doc = "Control automatic clock gating of RAM0 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM0_CTRL_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<RAM0_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RAM0_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM0_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM0_CTRL_A {
        match self.bits {
            false => RAM0_CTRL_A::DISABLE,
            true => RAM0_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAM0_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAM0_CTRL_A::ENABLE
    }
}
#[doc = "Field `RAM0_CTRL` writer - Control automatic clock gating of RAM0 controller."]
pub type RAM0_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, RAM0_CTRL_A, O>;
impl<'a, const O: u8> RAM0_CTRL_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM0_CTRL_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM0_CTRL_A::ENABLE)
    }
}
#[doc = "Field `RAM1_CTRL` reader - Control automatic clock gating of RAM1 controller."]
pub type RAM1_CTRL_R = crate::BitReader<RAM1_CTRL_A>;
#[doc = "Control automatic clock gating of RAM1 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM1_CTRL_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<RAM1_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RAM1_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM1_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM1_CTRL_A {
        match self.bits {
            false => RAM1_CTRL_A::DISABLE,
            true => RAM1_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAM1_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAM1_CTRL_A::ENABLE
    }
}
#[doc = "Field `RAM1_CTRL` writer - Control automatic clock gating of RAM1 controller."]
pub type RAM1_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, RAM1_CTRL_A, O>;
impl<'a, const O: u8> RAM1_CTRL_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM1_CTRL_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM1_CTRL_A::ENABLE)
    }
}
#[doc = "Field `RAM2_CTRL` reader - Control automatic clock gating of RAM2 controller."]
pub type RAM2_CTRL_R = crate::BitReader<RAM2_CTRL_A>;
#[doc = "Control automatic clock gating of RAM2 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM2_CTRL_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<RAM2_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RAM2_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM2_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM2_CTRL_A {
        match self.bits {
            false => RAM2_CTRL_A::DISABLE,
            true => RAM2_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAM2_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAM2_CTRL_A::ENABLE
    }
}
#[doc = "Field `RAM2_CTRL` writer - Control automatic clock gating of RAM2 controller."]
pub type RAM2_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, RAM2_CTRL_A, O>;
impl<'a, const O: u8> RAM2_CTRL_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM2_CTRL_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM2_CTRL_A::ENABLE)
    }
}
#[doc = "Field `RAM3_CTRL` reader - Control automatic clock gating of RAM3 controller."]
pub type RAM3_CTRL_R = crate::BitReader<RAM3_CTRL_A>;
#[doc = "Control automatic clock gating of RAM3 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM3_CTRL_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<RAM3_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RAM3_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM3_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM3_CTRL_A {
        match self.bits {
            false => RAM3_CTRL_A::DISABLE,
            true => RAM3_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAM3_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAM3_CTRL_A::ENABLE
    }
}
#[doc = "Field `RAM3_CTRL` writer - Control automatic clock gating of RAM3 controller."]
pub type RAM3_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, RAM3_CTRL_A, O>;
impl<'a, const O: u8> RAM3_CTRL_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM3_CTRL_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM3_CTRL_A::ENABLE)
    }
}
#[doc = "Field `RAM4_CTRL` reader - Control automatic clock gating of RAM4 controller."]
pub type RAM4_CTRL_R = crate::BitReader<RAM4_CTRL_A>;
#[doc = "Control automatic clock gating of RAM4 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM4_CTRL_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<RAM4_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: RAM4_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM4_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM4_CTRL_A {
        match self.bits {
            false => RAM4_CTRL_A::DISABLE,
            true => RAM4_CTRL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RAM4_CTRL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RAM4_CTRL_A::ENABLE
    }
}
#[doc = "Field `RAM4_CTRL` writer - Control automatic clock gating of RAM4 controller."]
pub type RAM4_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, RAM4_CTRL_A, O>;
impl<'a, const O: u8> RAM4_CTRL_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM4_CTRL_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM4_CTRL_A::ENABLE)
    }
}
#[doc = "Field `SYNC0_APB` reader - Control automatic clock gating of synchronous bridge controller 0."]
pub type SYNC0_APB_R = crate::BitReader<SYNC0_APB_A>;
#[doc = "Control automatic clock gating of synchronous bridge controller 0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC0_APB_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<SYNC0_APB_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC0_APB_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC0_APB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC0_APB_A {
        match self.bits {
            false => SYNC0_APB_A::DISABLE,
            true => SYNC0_APB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SYNC0_APB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SYNC0_APB_A::ENABLE
    }
}
#[doc = "Field `SYNC0_APB` writer - Control automatic clock gating of synchronous bridge controller 0."]
pub type SYNC0_APB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, SYNC0_APB_A, O>;
impl<'a, const O: u8> SYNC0_APB_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYNC0_APB_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYNC0_APB_A::ENABLE)
    }
}
#[doc = "Field `SYNC1_APB` reader - Control automatic clock gating of synchronous bridge controller 1."]
pub type SYNC1_APB_R = crate::BitReader<SYNC1_APB_A>;
#[doc = "Control automatic clock gating of synchronous bridge controller 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC1_APB_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<SYNC1_APB_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC1_APB_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC1_APB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC1_APB_A {
        match self.bits {
            false => SYNC1_APB_A::DISABLE,
            true => SYNC1_APB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SYNC1_APB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SYNC1_APB_A::ENABLE
    }
}
#[doc = "Field `SYNC1_APB` writer - Control automatic clock gating of synchronous bridge controller 1."]
pub type SYNC1_APB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, SYNC1_APB_A, O>;
impl<'a, const O: u8> SYNC1_APB_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYNC1_APB_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYNC1_APB_A::ENABLE)
    }
}
#[doc = "Field `CRCGEN` reader - Control automatic clock gating of CRCGEN controller."]
pub type CRCGEN_R = crate::BitReader<CRCGEN_A>;
#[doc = "Control automatic clock gating of CRCGEN controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCGEN_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<CRCGEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCGEN_A {
        match self.bits {
            false => CRCGEN_A::DISABLE,
            true => CRCGEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CRCGEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CRCGEN_A::ENABLE
    }
}
#[doc = "Field `CRCGEN` writer - Control automatic clock gating of CRCGEN controller."]
pub type CRCGEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, CRCGEN_A, O>;
impl<'a, const O: u8> CRCGEN_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRCGEN_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CRCGEN_A::ENABLE)
    }
}
#[doc = "Field `SDMA0` reader - Control automatic clock gating of DMA0 controller."]
pub type SDMA0_R = crate::BitReader<SDMA0_A>;
#[doc = "Control automatic clock gating of DMA0 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMA0_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<SDMA0_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA0_A) -> Self {
        variant as u8 != 0
    }
}
impl SDMA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA0_A {
        match self.bits {
            false => SDMA0_A::DISABLE,
            true => SDMA0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDMA0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDMA0_A::ENABLE
    }
}
#[doc = "Field `SDMA0` writer - Control automatic clock gating of DMA0 controller."]
pub type SDMA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, SDMA0_A, O>;
impl<'a, const O: u8> SDMA0_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA0_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDMA0_A::ENABLE)
    }
}
#[doc = "Field `SDMA1` reader - Control automatic clock gating of DMA1 controller."]
pub type SDMA1_R = crate::BitReader<SDMA1_A>;
#[doc = "Control automatic clock gating of DMA1 controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMA1_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<SDMA1_A> for bool {
    #[inline(always)]
    fn from(variant: SDMA1_A) -> Self {
        variant as u8 != 0
    }
}
impl SDMA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMA1_A {
        match self.bits {
            false => SDMA1_A::DISABLE,
            true => SDMA1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SDMA1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SDMA1_A::ENABLE
    }
}
#[doc = "Field `SDMA1` writer - Control automatic clock gating of DMA1 controller."]
pub type SDMA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, SDMA1_A, O>;
impl<'a, const O: u8> SDMA1_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDMA1_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDMA1_A::ENABLE)
    }
}
#[doc = "Field `USB0` reader - Control automatic clock gating of USB controller."]
pub type USB0_R = crate::BitReader<USB0_A>;
#[doc = "Control automatic clock gating of USB controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB0_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<USB0_A> for bool {
    #[inline(always)]
    fn from(variant: USB0_A) -> Self {
        variant as u8 != 0
    }
}
impl USB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB0_A {
        match self.bits {
            false => USB0_A::DISABLE,
            true => USB0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB0_A::ENABLE
    }
}
#[doc = "Field `USB0` writer - Control automatic clock gating of USB controller."]
pub type USB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, USB0_A, O>;
impl<'a, const O: u8> USB0_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB0_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB0_A::ENABLE)
    }
}
#[doc = "Field `SYSCON` reader - Control automatic clock gating of synchronous system controller registers bank."]
pub type SYSCON_R = crate::BitReader<SYSCON_A>;
#[doc = "Control automatic clock gating of synchronous system controller registers bank.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCON_A {
    #[doc = "0: Automatic clock gating is not overridden."]
    DISABLE = 0,
    #[doc = "1: Automatic clock gating is overridden (Clock gating is disabled)."]
    ENABLE = 1,
}
impl From<SYSCON_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCON_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCON_A {
        match self.bits {
            false => SYSCON_A::DISABLE,
            true => SYSCON_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SYSCON_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SYSCON_A::ENABLE
    }
}
#[doc = "Field `SYSCON` writer - Control automatic clock gating of synchronous system controller registers bank."]
pub type SYSCON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, SYSCON_A, O>;
impl<'a, const O: u8> SYSCON_W<'a, O> {
    #[doc = "Automatic clock gating is not overridden."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SYSCON_A::DISABLE)
    }
    #[doc = "Automatic clock gating is overridden (Clock gating is disabled)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SYSCON_A::ENABLE)
    }
}
#[doc = "The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ENABLEUPDATE_AW {
    #[doc = "0: Bit Fields 0 - 15 of this register are not updated"]
    DISABLE = 0,
    #[doc = "49374: Bit Fields 0 - 15 of this register are updated"]
    ENABLE = 49374,
}
impl From<ENABLEUPDATE_AW> for u16 {
    #[inline(always)]
    fn from(variant: ENABLEUPDATE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `ENABLEUPDATE` writer - The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect."]
pub type ENABLEUPDATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUTOCLKGATEOVERRIDE_SPEC, u16, ENABLEUPDATE_AW, 16, O>;
impl<'a, const O: u8> ENABLEUPDATE_W<'a, O> {
    #[doc = "Bit Fields 0 - 15 of this register are not updated"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLEUPDATE_AW::DISABLE)
    }
    #[doc = "Bit Fields 0 - 15 of this register are updated"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLEUPDATE_AW::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Control automatic clock gating of ROM controller."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control automatic clock gating of RAMX controller."]
    #[inline(always)]
    pub fn ramx_ctrl(&self) -> RAMX_CTRL_R {
        RAMX_CTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control automatic clock gating of RAM0 controller."]
    #[inline(always)]
    pub fn ram0_ctrl(&self) -> RAM0_CTRL_R {
        RAM0_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control automatic clock gating of RAM1 controller."]
    #[inline(always)]
    pub fn ram1_ctrl(&self) -> RAM1_CTRL_R {
        RAM1_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control automatic clock gating of RAM2 controller."]
    #[inline(always)]
    pub fn ram2_ctrl(&self) -> RAM2_CTRL_R {
        RAM2_CTRL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Control automatic clock gating of RAM3 controller."]
    #[inline(always)]
    pub fn ram3_ctrl(&self) -> RAM3_CTRL_R {
        RAM3_CTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Control automatic clock gating of RAM4 controller."]
    #[inline(always)]
    pub fn ram4_ctrl(&self) -> RAM4_CTRL_R {
        RAM4_CTRL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Control automatic clock gating of synchronous bridge controller 0."]
    #[inline(always)]
    pub fn sync0_apb(&self) -> SYNC0_APB_R {
        SYNC0_APB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Control automatic clock gating of synchronous bridge controller 1."]
    #[inline(always)]
    pub fn sync1_apb(&self) -> SYNC1_APB_R {
        SYNC1_APB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Control automatic clock gating of CRCGEN controller."]
    #[inline(always)]
    pub fn crcgen(&self) -> CRCGEN_R {
        CRCGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Control automatic clock gating of DMA0 controller."]
    #[inline(always)]
    pub fn sdma0(&self) -> SDMA0_R {
        SDMA0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Control automatic clock gating of DMA1 controller."]
    #[inline(always)]
    pub fn sdma1(&self) -> SDMA1_R {
        SDMA1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Control automatic clock gating of USB controller."]
    #[inline(always)]
    pub fn usb0(&self) -> USB0_R {
        USB0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Control automatic clock gating of synchronous system controller registers bank."]
    #[inline(always)]
    pub fn syscon(&self) -> SYSCON_R {
        SYSCON_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control automatic clock gating of ROM controller."]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<0> {
        ROM_W::new(self)
    }
    #[doc = "Bit 1 - Control automatic clock gating of RAMX controller."]
    #[inline(always)]
    #[must_use]
    pub fn ramx_ctrl(&mut self) -> RAMX_CTRL_W<1> {
        RAMX_CTRL_W::new(self)
    }
    #[doc = "Bit 2 - Control automatic clock gating of RAM0 controller."]
    #[inline(always)]
    #[must_use]
    pub fn ram0_ctrl(&mut self) -> RAM0_CTRL_W<2> {
        RAM0_CTRL_W::new(self)
    }
    #[doc = "Bit 3 - Control automatic clock gating of RAM1 controller."]
    #[inline(always)]
    #[must_use]
    pub fn ram1_ctrl(&mut self) -> RAM1_CTRL_W<3> {
        RAM1_CTRL_W::new(self)
    }
    #[doc = "Bit 4 - Control automatic clock gating of RAM2 controller."]
    #[inline(always)]
    #[must_use]
    pub fn ram2_ctrl(&mut self) -> RAM2_CTRL_W<4> {
        RAM2_CTRL_W::new(self)
    }
    #[doc = "Bit 5 - Control automatic clock gating of RAM3 controller."]
    #[inline(always)]
    #[must_use]
    pub fn ram3_ctrl(&mut self) -> RAM3_CTRL_W<5> {
        RAM3_CTRL_W::new(self)
    }
    #[doc = "Bit 6 - Control automatic clock gating of RAM4 controller."]
    #[inline(always)]
    #[must_use]
    pub fn ram4_ctrl(&mut self) -> RAM4_CTRL_W<6> {
        RAM4_CTRL_W::new(self)
    }
    #[doc = "Bit 7 - Control automatic clock gating of synchronous bridge controller 0."]
    #[inline(always)]
    #[must_use]
    pub fn sync0_apb(&mut self) -> SYNC0_APB_W<7> {
        SYNC0_APB_W::new(self)
    }
    #[doc = "Bit 8 - Control automatic clock gating of synchronous bridge controller 1."]
    #[inline(always)]
    #[must_use]
    pub fn sync1_apb(&mut self) -> SYNC1_APB_W<8> {
        SYNC1_APB_W::new(self)
    }
    #[doc = "Bit 11 - Control automatic clock gating of CRCGEN controller."]
    #[inline(always)]
    #[must_use]
    pub fn crcgen(&mut self) -> CRCGEN_W<11> {
        CRCGEN_W::new(self)
    }
    #[doc = "Bit 12 - Control automatic clock gating of DMA0 controller."]
    #[inline(always)]
    #[must_use]
    pub fn sdma0(&mut self) -> SDMA0_W<12> {
        SDMA0_W::new(self)
    }
    #[doc = "Bit 13 - Control automatic clock gating of DMA1 controller."]
    #[inline(always)]
    #[must_use]
    pub fn sdma1(&mut self) -> SDMA1_W<13> {
        SDMA1_W::new(self)
    }
    #[doc = "Bit 14 - Control automatic clock gating of USB controller."]
    #[inline(always)]
    #[must_use]
    pub fn usb0(&mut self) -> USB0_W<14> {
        USB0_W::new(self)
    }
    #[doc = "Bit 15 - Control automatic clock gating of synchronous system controller registers bank."]
    #[inline(always)]
    #[must_use]
    pub fn syscon(&mut self) -> SYSCON_W<15> {
        SYSCON_W::new(self)
    }
    #[doc = "Bits 16:31 - The value 0xC0DE must be written for AUTOCLKGATEOVERRIDE registers fields updates to have effect."]
    #[inline(always)]
    #[must_use]
    pub fn enableupdate(&mut self) -> ENABLEUPDATE_W<16> {
        ENABLEUPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control automatic clock gating\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autoclkgateoverride](index.html) module"]
pub struct AUTOCLKGATEOVERRIDE_SPEC;
impl crate::RegisterSpec for AUTOCLKGATEOVERRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autoclkgateoverride::R](R) reader structure"]
impl crate::Readable for AUTOCLKGATEOVERRIDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autoclkgateoverride::W](W) writer structure"]
impl crate::Writable for AUTOCLKGATEOVERRIDE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUTOCLKGATEOVERRIDE to value 0xffff"]
impl crate::Resettable for AUTOCLKGATEOVERRIDE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}

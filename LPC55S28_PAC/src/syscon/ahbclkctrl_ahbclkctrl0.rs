#[doc = "Register `AHBCLKCTRL0` reader"]
pub struct R(crate::R<AHBCLKCTRL_AHBCLKCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKCTRL_AHBCLKCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKCTRL_AHBCLKCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKCTRL_AHBCLKCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKCTRL0` writer"]
pub struct W(crate::W<AHBCLKCTRL_AHBCLKCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRL_AHBCLKCTRL0_SPEC>;
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
impl From<crate::W<AHBCLKCTRL_AHBCLKCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRL_AHBCLKCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM` reader - Enables the clock for the ROM."]
pub type ROM_R = crate::BitReader<ROM_A>;
#[doc = "Enables the clock for the ROM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROM_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
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
#[doc = "Field `ROM` writer - Enables the clock for the ROM."]
pub type ROM_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, ROM_A, O>;
impl<'a, const O: u8> ROM_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROM_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROM_A::ENABLE)
    }
}
#[doc = "Field `SRAM_CTRL1` reader - Enables the clock for the SRAM Controller 1."]
pub type SRAM_CTRL1_R = crate::BitReader<SRAM_CTRL1_A>;
#[doc = "Enables the clock for the SRAM Controller 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_CTRL1_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SRAM_CTRL1_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL1_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_CTRL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL1_A {
        match self.bits {
            false => SRAM_CTRL1_A::DISABLE,
            true => SRAM_CTRL1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SRAM_CTRL1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SRAM_CTRL1_A::ENABLE
    }
}
#[doc = "Field `SRAM_CTRL1` writer - Enables the clock for the SRAM Controller 1."]
pub type SRAM_CTRL1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, SRAM_CTRL1_A, O>;
impl<'a, const O: u8> SRAM_CTRL1_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL1_A::ENABLE)
    }
}
#[doc = "Field `SRAM_CTRL2` reader - Enables the clock for the SRAM Controller 2."]
pub type SRAM_CTRL2_R = crate::BitReader<SRAM_CTRL2_A>;
#[doc = "Enables the clock for the SRAM Controller 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_CTRL2_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SRAM_CTRL2_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL2_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_CTRL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL2_A {
        match self.bits {
            false => SRAM_CTRL2_A::DISABLE,
            true => SRAM_CTRL2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SRAM_CTRL2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SRAM_CTRL2_A::ENABLE
    }
}
#[doc = "Field `SRAM_CTRL2` writer - Enables the clock for the SRAM Controller 2."]
pub type SRAM_CTRL2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, SRAM_CTRL2_A, O>;
impl<'a, const O: u8> SRAM_CTRL2_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL2_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL2_A::ENABLE)
    }
}
#[doc = "Field `SRAM_CTRL3` reader - Enables the clock for the SRAM Controller 3."]
pub type SRAM_CTRL3_R = crate::BitReader<SRAM_CTRL3_A>;
#[doc = "Enables the clock for the SRAM Controller 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_CTRL3_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SRAM_CTRL3_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL3_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_CTRL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL3_A {
        match self.bits {
            false => SRAM_CTRL3_A::DISABLE,
            true => SRAM_CTRL3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SRAM_CTRL3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SRAM_CTRL3_A::ENABLE
    }
}
#[doc = "Field `SRAM_CTRL3` writer - Enables the clock for the SRAM Controller 3."]
pub type SRAM_CTRL3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, SRAM_CTRL3_A, O>;
impl<'a, const O: u8> SRAM_CTRL3_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL3_A::ENABLE)
    }
}
#[doc = "Field `SRAM_CTRL4` reader - Enables the clock for the SRAM Controller 4."]
pub type SRAM_CTRL4_R = crate::BitReader<SRAM_CTRL4_A>;
#[doc = "Enables the clock for the SRAM Controller 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_CTRL4_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<SRAM_CTRL4_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_CTRL4_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_CTRL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_CTRL4_A {
        match self.bits {
            false => SRAM_CTRL4_A::DISABLE,
            true => SRAM_CTRL4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SRAM_CTRL4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SRAM_CTRL4_A::ENABLE
    }
}
#[doc = "Field `SRAM_CTRL4` writer - Enables the clock for the SRAM Controller 4."]
pub type SRAM_CTRL4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, SRAM_CTRL4_A, O>;
impl<'a, const O: u8> SRAM_CTRL4_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRAM_CTRL4_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRAM_CTRL4_A::ENABLE)
    }
}
#[doc = "Field `FLASH` reader - Enables the clock for the Flash controller."]
pub type FLASH_R = crate::BitReader<FLASH_A>;
#[doc = "Enables the clock for the Flash controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASH_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FLASH_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_A {
        match self.bits {
            false => FLASH_A::DISABLE,
            true => FLASH_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLASH_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLASH_A::ENABLE
    }
}
#[doc = "Field `FLASH` writer - Enables the clock for the Flash controller."]
pub type FLASH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, FLASH_A, O>;
impl<'a, const O: u8> FLASH_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLASH_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLASH_A::ENABLE)
    }
}
#[doc = "Field `FMC` reader - Enables the clock for the FMC controller."]
pub type FMC_R = crate::BitReader<FMC_A>;
#[doc = "Enables the clock for the FMC controller.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMC_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<FMC_A> for bool {
    #[inline(always)]
    fn from(variant: FMC_A) -> Self {
        variant as u8 != 0
    }
}
impl FMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMC_A {
        match self.bits {
            false => FMC_A::DISABLE,
            true => FMC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FMC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FMC_A::ENABLE
    }
}
#[doc = "Field `FMC` writer - Enables the clock for the FMC controller."]
pub type FMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, FMC_A, O>;
impl<'a, const O: u8> FMC_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FMC_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FMC_A::ENABLE)
    }
}
#[doc = "Field `MUX` reader - Enables the clock for the Input Mux."]
pub type MUX_R = crate::BitReader<MUX_A>;
#[doc = "Enables the clock for the Input Mux.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUX_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<MUX_A> for bool {
    #[inline(always)]
    fn from(variant: MUX_A) -> Self {
        variant as u8 != 0
    }
}
impl MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_A {
        match self.bits {
            false => MUX_A::DISABLE,
            true => MUX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MUX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MUX_A::ENABLE
    }
}
#[doc = "Field `MUX` writer - Enables the clock for the Input Mux."]
pub type MUX_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, MUX_A, O>;
impl<'a, const O: u8> MUX_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MUX_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MUX_A::ENABLE)
    }
}
#[doc = "Field `IOCON` reader - Enables the clock for the I/O controller."]
pub type IOCON_R = crate::BitReader<IOCON_A>;
#[doc = "Enables the clock for the I/O controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOCON_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<IOCON_A> for bool {
    #[inline(always)]
    fn from(variant: IOCON_A) -> Self {
        variant as u8 != 0
    }
}
impl IOCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_A {
        match self.bits {
            false => IOCON_A::DISABLE,
            true => IOCON_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IOCON_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IOCON_A::ENABLE
    }
}
#[doc = "Field `IOCON` writer - Enables the clock for the I/O controller."]
pub type IOCON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, IOCON_A, O>;
impl<'a, const O: u8> IOCON_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IOCON_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IOCON_A::ENABLE)
    }
}
#[doc = "Field `GPIO0` reader - Enables the clock for the GPIO0."]
pub type GPIO0_R = crate::BitReader<GPIO0_A>;
#[doc = "Enables the clock for the GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO0_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GPIO0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0_A {
        match self.bits {
            false => GPIO0_A::DISABLE,
            true => GPIO0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO0_A::ENABLE
    }
}
#[doc = "Field `GPIO0` writer - Enables the clock for the GPIO0."]
pub type GPIO0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, GPIO0_A, O>;
impl<'a, const O: u8> GPIO0_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO0_A::ENABLE)
    }
}
#[doc = "Field `GPIO1` reader - Enables the clock for the GPIO1."]
pub type GPIO1_R = crate::BitReader<GPIO1_A>;
#[doc = "Enables the clock for the GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO1_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GPIO1_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1_A {
        match self.bits {
            false => GPIO1_A::DISABLE,
            true => GPIO1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO1_A::ENABLE
    }
}
#[doc = "Field `GPIO1` writer - Enables the clock for the GPIO1."]
pub type GPIO1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, GPIO1_A, O>;
impl<'a, const O: u8> GPIO1_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO1_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO1_A::ENABLE)
    }
}
#[doc = "Field `GPIO2` reader - Enables the clock for the GPIO2."]
pub type GPIO2_R = crate::BitReader<GPIO2_A>;
#[doc = "Enables the clock for the GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO2_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GPIO2_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO2_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO2_A {
        match self.bits {
            false => GPIO2_A::DISABLE,
            true => GPIO2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO2_A::ENABLE
    }
}
#[doc = "Field `GPIO2` writer - Enables the clock for the GPIO2."]
pub type GPIO2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, GPIO2_A, O>;
impl<'a, const O: u8> GPIO2_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO2_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO2_A::ENABLE)
    }
}
#[doc = "Field `GPIO3` reader - Enables the clock for the GPIO3."]
pub type GPIO3_R = crate::BitReader<GPIO3_A>;
#[doc = "Enables the clock for the GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO3_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GPIO3_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO3_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO3_A {
        match self.bits {
            false => GPIO3_A::DISABLE,
            true => GPIO3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GPIO3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GPIO3_A::ENABLE
    }
}
#[doc = "Field `GPIO3` writer - Enables the clock for the GPIO3."]
pub type GPIO3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, GPIO3_A, O>;
impl<'a, const O: u8> GPIO3_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO3_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO3_A::ENABLE)
    }
}
#[doc = "Field `PINT` reader - Enables the clock for the Pin interrupt (PINT)."]
pub type PINT_R = crate::BitReader<PINT_A>;
#[doc = "Enables the clock for the Pin interrupt (PINT).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<PINT_A> for bool {
    #[inline(always)]
    fn from(variant: PINT_A) -> Self {
        variant as u8 != 0
    }
}
impl PINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT_A {
        match self.bits {
            false => PINT_A::DISABLE,
            true => PINT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PINT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PINT_A::ENABLE
    }
}
#[doc = "Field `PINT` writer - Enables the clock for the Pin interrupt (PINT)."]
pub type PINT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, PINT_A, O>;
impl<'a, const O: u8> PINT_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PINT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PINT_A::ENABLE)
    }
}
#[doc = "Field `GINT` reader - Enables the clock for the Group interrupt (GINT)."]
pub type GINT_R = crate::BitReader<GINT_A>;
#[doc = "Enables the clock for the Group interrupt (GINT).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GINT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<GINT_A> for bool {
    #[inline(always)]
    fn from(variant: GINT_A) -> Self {
        variant as u8 != 0
    }
}
impl GINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GINT_A {
        match self.bits {
            false => GINT_A::DISABLE,
            true => GINT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GINT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GINT_A::ENABLE
    }
}
#[doc = "Field `GINT` writer - Enables the clock for the Group interrupt (GINT)."]
pub type GINT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, GINT_A, O>;
impl<'a, const O: u8> GINT_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GINT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GINT_A::ENABLE)
    }
}
#[doc = "Field `DMA0` reader - Enables the clock for the DMA0."]
pub type DMA0_R = crate::BitReader<DMA0_A>;
#[doc = "Enables the clock for the DMA0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<DMA0_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0_A {
        match self.bits {
            false => DMA0_A::DISABLE,
            true => DMA0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMA0_A::ENABLE
    }
}
#[doc = "Field `DMA0` writer - Enables the clock for the DMA0."]
pub type DMA0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, DMA0_A, O>;
impl<'a, const O: u8> DMA0_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA0_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA0_A::ENABLE)
    }
}
#[doc = "Field `CRCGEN` reader - Enables the clock for the CRCGEN."]
pub type CRCGEN_R = crate::BitReader<CRCGEN_A>;
#[doc = "Enables the clock for the CRCGEN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCGEN_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
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
#[doc = "Field `CRCGEN` writer - Enables the clock for the CRCGEN."]
pub type CRCGEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, CRCGEN_A, O>;
impl<'a, const O: u8> CRCGEN_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRCGEN_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CRCGEN_A::ENABLE)
    }
}
#[doc = "Field `WWDT` reader - Enables the clock for the Watchdog Timer."]
pub type WWDT_R = crate::BitReader<WWDT_A>;
#[doc = "Enables the clock for the Watchdog Timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WWDT_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<WWDT_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT_A) -> Self {
        variant as u8 != 0
    }
}
impl WWDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT_A {
        match self.bits {
            false => WWDT_A::DISABLE,
            true => WWDT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WWDT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WWDT_A::ENABLE
    }
}
#[doc = "Field `WWDT` writer - Enables the clock for the Watchdog Timer."]
pub type WWDT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, WWDT_A, O>;
impl<'a, const O: u8> WWDT_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WWDT_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WWDT_A::ENABLE)
    }
}
#[doc = "Field `RTC` reader - Enables the clock for the Real Time Clock (RTC)."]
pub type RTC_R = crate::BitReader<RTC_A>;
#[doc = "Enables the clock for the Real Time Clock (RTC).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<RTC_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            false => RTC_A::DISABLE,
            true => RTC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RTC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTC_A::ENABLE
    }
}
#[doc = "Field `RTC` writer - Enables the clock for the Real Time Clock (RTC)."]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, RTC_A, O>;
impl<'a, const O: u8> RTC_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RTC_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTC_A::ENABLE)
    }
}
#[doc = "Field `MAILBOX` reader - Enables the clock for the Inter CPU communication Mailbox."]
pub type MAILBOX_R = crate::BitReader<MAILBOX_A>;
#[doc = "Enables the clock for the Inter CPU communication Mailbox.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAILBOX_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<MAILBOX_A> for bool {
    #[inline(always)]
    fn from(variant: MAILBOX_A) -> Self {
        variant as u8 != 0
    }
}
impl MAILBOX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAILBOX_A {
        match self.bits {
            false => MAILBOX_A::DISABLE,
            true => MAILBOX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MAILBOX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MAILBOX_A::ENABLE
    }
}
#[doc = "Field `MAILBOX` writer - Enables the clock for the Inter CPU communication Mailbox."]
pub type MAILBOX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, MAILBOX_A, O>;
impl<'a, const O: u8> MAILBOX_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MAILBOX_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MAILBOX_A::ENABLE)
    }
}
#[doc = "Field `ADC` reader - Enables the clock for the ADC."]
pub type ADC_R = crate::BitReader<ADC_A>;
#[doc = "Enables the clock for the ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC_A {
    #[doc = "0: Disable Clock."]
    DISABLE = 0,
    #[doc = "1: Enable Clock."]
    ENABLE = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::DISABLE,
            true => ADC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADC_A::ENABLE
    }
}
#[doc = "Field `ADC` writer - Enables the clock for the ADC."]
pub type ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHBCLKCTRL_AHBCLKCTRL0_SPEC, ADC_A, O>;
impl<'a, const O: u8> ADC_W<'a, O> {
    #[doc = "Disable Clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_A::DISABLE)
    }
    #[doc = "Enable Clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 1 - Enables the clock for the ROM."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the clock for the SRAM Controller 1."]
    #[inline(always)]
    pub fn sram_ctrl1(&self) -> SRAM_CTRL1_R {
        SRAM_CTRL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables the clock for the SRAM Controller 2."]
    #[inline(always)]
    pub fn sram_ctrl2(&self) -> SRAM_CTRL2_R {
        SRAM_CTRL2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the clock for the SRAM Controller 3."]
    #[inline(always)]
    pub fn sram_ctrl3(&self) -> SRAM_CTRL3_R {
        SRAM_CTRL3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables the clock for the SRAM Controller 4."]
    #[inline(always)]
    pub fn sram_ctrl4(&self) -> SRAM_CTRL4_R {
        SRAM_CTRL4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for the Flash controller."]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for the FMC controller."]
    #[inline(always)]
    pub fn fmc(&self) -> FMC_R {
        FMC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for the Input Mux."]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for the I/O controller."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for the Pin interrupt (PINT)."]
    #[inline(always)]
    pub fn pint(&self) -> PINT_R {
        PINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the Group interrupt (GINT)."]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables the clock for the DMA0."]
    #[inline(always)]
    pub fn dma0(&self) -> DMA0_R {
        DMA0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enables the clock for the CRCGEN."]
    #[inline(always)]
    pub fn crcgen(&self) -> CRCGEN_R {
        CRCGEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer."]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enables the clock for the Real Time Clock (RTC)."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 26 - Enables the clock for the Inter CPU communication Mailbox."]
    #[inline(always)]
    pub fn mailbox(&self) -> MAILBOX_R {
        MAILBOX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for the ADC."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enables the clock for the ROM."]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<1> {
        ROM_W::new(self)
    }
    #[doc = "Bit 3 - Enables the clock for the SRAM Controller 1."]
    #[inline(always)]
    #[must_use]
    pub fn sram_ctrl1(&mut self) -> SRAM_CTRL1_W<3> {
        SRAM_CTRL1_W::new(self)
    }
    #[doc = "Bit 4 - Enables the clock for the SRAM Controller 2."]
    #[inline(always)]
    #[must_use]
    pub fn sram_ctrl2(&mut self) -> SRAM_CTRL2_W<4> {
        SRAM_CTRL2_W::new(self)
    }
    #[doc = "Bit 5 - Enables the clock for the SRAM Controller 3."]
    #[inline(always)]
    #[must_use]
    pub fn sram_ctrl3(&mut self) -> SRAM_CTRL3_W<5> {
        SRAM_CTRL3_W::new(self)
    }
    #[doc = "Bit 6 - Enables the clock for the SRAM Controller 4."]
    #[inline(always)]
    #[must_use]
    pub fn sram_ctrl4(&mut self) -> SRAM_CTRL4_W<6> {
        SRAM_CTRL4_W::new(self)
    }
    #[doc = "Bit 7 - Enables the clock for the Flash controller."]
    #[inline(always)]
    #[must_use]
    pub fn flash(&mut self) -> FLASH_W<7> {
        FLASH_W::new(self)
    }
    #[doc = "Bit 8 - Enables the clock for the FMC controller."]
    #[inline(always)]
    #[must_use]
    pub fn fmc(&mut self) -> FMC_W<8> {
        FMC_W::new(self)
    }
    #[doc = "Bit 11 - Enables the clock for the Input Mux."]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MUX_W<11> {
        MUX_W::new(self)
    }
    #[doc = "Bit 13 - Enables the clock for the I/O controller."]
    #[inline(always)]
    #[must_use]
    pub fn iocon(&mut self) -> IOCON_W<13> {
        IOCON_W::new(self)
    }
    #[doc = "Bit 14 - Enables the clock for the GPIO0."]
    #[inline(always)]
    #[must_use]
    pub fn gpio0(&mut self) -> GPIO0_W<14> {
        GPIO0_W::new(self)
    }
    #[doc = "Bit 15 - Enables the clock for the GPIO1."]
    #[inline(always)]
    #[must_use]
    pub fn gpio1(&mut self) -> GPIO1_W<15> {
        GPIO1_W::new(self)
    }
    #[doc = "Bit 16 - Enables the clock for the GPIO2."]
    #[inline(always)]
    #[must_use]
    pub fn gpio2(&mut self) -> GPIO2_W<16> {
        GPIO2_W::new(self)
    }
    #[doc = "Bit 17 - Enables the clock for the GPIO3."]
    #[inline(always)]
    #[must_use]
    pub fn gpio3(&mut self) -> GPIO3_W<17> {
        GPIO3_W::new(self)
    }
    #[doc = "Bit 18 - Enables the clock for the Pin interrupt (PINT)."]
    #[inline(always)]
    #[must_use]
    pub fn pint(&mut self) -> PINT_W<18> {
        PINT_W::new(self)
    }
    #[doc = "Bit 19 - Enables the clock for the Group interrupt (GINT)."]
    #[inline(always)]
    #[must_use]
    pub fn gint(&mut self) -> GINT_W<19> {
        GINT_W::new(self)
    }
    #[doc = "Bit 20 - Enables the clock for the DMA0."]
    #[inline(always)]
    #[must_use]
    pub fn dma0(&mut self) -> DMA0_W<20> {
        DMA0_W::new(self)
    }
    #[doc = "Bit 21 - Enables the clock for the CRCGEN."]
    #[inline(always)]
    #[must_use]
    pub fn crcgen(&mut self) -> CRCGEN_W<21> {
        CRCGEN_W::new(self)
    }
    #[doc = "Bit 22 - Enables the clock for the Watchdog Timer."]
    #[inline(always)]
    #[must_use]
    pub fn wwdt(&mut self) -> WWDT_W<22> {
        WWDT_W::new(self)
    }
    #[doc = "Bit 23 - Enables the clock for the Real Time Clock (RTC)."]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<23> {
        RTC_W::new(self)
    }
    #[doc = "Bit 26 - Enables the clock for the Inter CPU communication Mailbox."]
    #[inline(always)]
    #[must_use]
    pub fn mailbox(&mut self) -> MAILBOX_W<26> {
        MAILBOX_W::new(self)
    }
    #[doc = "Bit 27 - Enables the clock for the ADC."]
    #[inline(always)]
    #[must_use]
    pub fn adc(&mut self) -> ADC_W<27> {
        ADC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Clock control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl_ahbclkctrl0](index.html) module"]
pub struct AHBCLKCTRL_AHBCLKCTRL0_SPEC;
impl crate::RegisterSpec for AHBCLKCTRL_AHBCLKCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclkctrl_ahbclkctrl0::R](R) reader structure"]
impl crate::Readable for AHBCLKCTRL_AHBCLKCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl_ahbclkctrl0::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRL_AHBCLKCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBCLKCTRL0 to value 0x0180"]
impl crate::Resettable for AHBCLKCTRL_AHBCLKCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0180;
}

#[doc = "Register `CLOCK_CTRL` reader"]
pub struct R(crate::R<CLOCK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_CTRL` writer"]
pub struct W(crate::W<CLOCK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_CTRL_SPEC>;
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
impl From<crate::W<CLOCK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32MHZ_FREQM_ENA` reader - Enable XTAL32MHz clock for Frequency Measure module."]
pub type XTAL32MHZ_FREQM_ENA_R = crate::BitReader<XTAL32MHZ_FREQM_ENA_A>;
#[doc = "Enable XTAL32MHz clock for Frequency Measure module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XTAL32MHZ_FREQM_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<XTAL32MHZ_FREQM_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: XTAL32MHZ_FREQM_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl XTAL32MHZ_FREQM_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XTAL32MHZ_FREQM_ENA_A {
        match self.bits {
            false => XTAL32MHZ_FREQM_ENA_A::DISABLE,
            true => XTAL32MHZ_FREQM_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == XTAL32MHZ_FREQM_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == XTAL32MHZ_FREQM_ENA_A::ENABLE
    }
}
#[doc = "Field `XTAL32MHZ_FREQM_ENA` writer - Enable XTAL32MHz clock for Frequency Measure module."]
pub type XTAL32MHZ_FREQM_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLOCK_CTRL_SPEC, XTAL32MHZ_FREQM_ENA_A, O>;
impl<'a, const O: u8> XTAL32MHZ_FREQM_ENA_W<'a, O> {
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(XTAL32MHZ_FREQM_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(XTAL32MHZ_FREQM_ENA_A::ENABLE)
    }
}
#[doc = "Field `FRO1MHZ_UTICK_ENA` reader - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
pub type FRO1MHZ_UTICK_ENA_R = crate::BitReader<FRO1MHZ_UTICK_ENA_A>;
#[doc = "Enable FRO 1MHz clock for Frequency Measure module and for UTICK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRO1MHZ_UTICK_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<FRO1MHZ_UTICK_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: FRO1MHZ_UTICK_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl FRO1MHZ_UTICK_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO1MHZ_UTICK_ENA_A {
        match self.bits {
            false => FRO1MHZ_UTICK_ENA_A::DISABLE,
            true => FRO1MHZ_UTICK_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRO1MHZ_UTICK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRO1MHZ_UTICK_ENA_A::ENABLE
    }
}
#[doc = "Field `FRO1MHZ_UTICK_ENA` writer - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
pub type FRO1MHZ_UTICK_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLOCK_CTRL_SPEC, FRO1MHZ_UTICK_ENA_A, O>;
impl<'a, const O: u8> FRO1MHZ_UTICK_ENA_W<'a, O> {
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRO1MHZ_UTICK_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRO1MHZ_UTICK_ENA_A::ENABLE)
    }
}
#[doc = "Field `FRO12MHZ_FREQM_ENA` reader - Enable FRO 12MHz clock for Frequency Measure module."]
pub type FRO12MHZ_FREQM_ENA_R = crate::BitReader<FRO12MHZ_FREQM_ENA_A>;
#[doc = "Enable FRO 12MHz clock for Frequency Measure module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRO12MHZ_FREQM_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<FRO12MHZ_FREQM_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: FRO12MHZ_FREQM_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl FRO12MHZ_FREQM_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO12MHZ_FREQM_ENA_A {
        match self.bits {
            false => FRO12MHZ_FREQM_ENA_A::DISABLE,
            true => FRO12MHZ_FREQM_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRO12MHZ_FREQM_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRO12MHZ_FREQM_ENA_A::ENABLE
    }
}
#[doc = "Field `FRO12MHZ_FREQM_ENA` writer - Enable FRO 12MHz clock for Frequency Measure module."]
pub type FRO12MHZ_FREQM_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLOCK_CTRL_SPEC, FRO12MHZ_FREQM_ENA_A, O>;
impl<'a, const O: u8> FRO12MHZ_FREQM_ENA_W<'a, O> {
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRO12MHZ_FREQM_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRO12MHZ_FREQM_ENA_A::ENABLE)
    }
}
#[doc = "Field `FRO_HF_FREQM_ENA` reader - Enable FRO 96MHz clock for Frequency Measure module."]
pub type FRO_HF_FREQM_ENA_R = crate::BitReader<FRO_HF_FREQM_ENA_A>;
#[doc = "Enable FRO 96MHz clock for Frequency Measure module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRO_HF_FREQM_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<FRO_HF_FREQM_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: FRO_HF_FREQM_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl FRO_HF_FREQM_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO_HF_FREQM_ENA_A {
        match self.bits {
            false => FRO_HF_FREQM_ENA_A::DISABLE,
            true => FRO_HF_FREQM_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRO_HF_FREQM_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRO_HF_FREQM_ENA_A::ENABLE
    }
}
#[doc = "Field `FRO_HF_FREQM_ENA` writer - Enable FRO 96MHz clock for Frequency Measure module."]
pub type FRO_HF_FREQM_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLOCK_CTRL_SPEC, FRO_HF_FREQM_ENA_A, O>;
impl<'a, const O: u8> FRO_HF_FREQM_ENA_W<'a, O> {
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRO_HF_FREQM_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRO_HF_FREQM_ENA_A::ENABLE)
    }
}
#[doc = "Field `CLKIN_ENA` reader - Enable clock_in clock for clock module."]
pub type CLKIN_ENA_R = crate::BitReader<CLKIN_ENA_A>;
#[doc = "Enable clock_in clock for clock module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKIN_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<CLKIN_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: CLKIN_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKIN_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKIN_ENA_A {
        match self.bits {
            false => CLKIN_ENA_A::DISABLE,
            true => CLKIN_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLKIN_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLKIN_ENA_A::ENABLE
    }
}
#[doc = "Field `CLKIN_ENA` writer - Enable clock_in clock for clock module."]
pub type CLKIN_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLOCK_CTRL_SPEC, CLKIN_ENA_A, O>;
impl<'a, const O: u8> CLKIN_ENA_W<'a, O> {
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLKIN_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLKIN_ENA_A::ENABLE)
    }
}
#[doc = "Field `FRO1MHZ_CLK_ENA` reader - Enable FRO 1MHz clock for clock muxing in clock gen."]
pub type FRO1MHZ_CLK_ENA_R = crate::BitReader<FRO1MHZ_CLK_ENA_A>;
#[doc = "Enable FRO 1MHz clock for clock muxing in clock gen.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRO1MHZ_CLK_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<FRO1MHZ_CLK_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: FRO1MHZ_CLK_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl FRO1MHZ_CLK_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO1MHZ_CLK_ENA_A {
        match self.bits {
            false => FRO1MHZ_CLK_ENA_A::DISABLE,
            true => FRO1MHZ_CLK_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FRO1MHZ_CLK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FRO1MHZ_CLK_ENA_A::ENABLE
    }
}
#[doc = "Field `FRO1MHZ_CLK_ENA` writer - Enable FRO 1MHz clock for clock muxing in clock gen."]
pub type FRO1MHZ_CLK_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLOCK_CTRL_SPEC, FRO1MHZ_CLK_ENA_A, O>;
impl<'a, const O: u8> FRO1MHZ_CLK_ENA_W<'a, O> {
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FRO1MHZ_CLK_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FRO1MHZ_CLK_ENA_A::ENABLE)
    }
}
#[doc = "Field `ANA_FRO12M_CLK_ENA` reader - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
pub type ANA_FRO12M_CLK_ENA_R = crate::BitReader<ANA_FRO12M_CLK_ENA_A>;
#[doc = "Enable FRO 12MHz clock for analog control of the FRO 192MHz.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANA_FRO12M_CLK_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<ANA_FRO12M_CLK_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: ANA_FRO12M_CLK_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl ANA_FRO12M_CLK_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANA_FRO12M_CLK_ENA_A {
        match self.bits {
            false => ANA_FRO12M_CLK_ENA_A::DISABLE,
            true => ANA_FRO12M_CLK_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ANA_FRO12M_CLK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ANA_FRO12M_CLK_ENA_A::ENABLE
    }
}
#[doc = "Field `ANA_FRO12M_CLK_ENA` writer - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
pub type ANA_FRO12M_CLK_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLOCK_CTRL_SPEC, ANA_FRO12M_CLK_ENA_A, O>;
impl<'a, const O: u8> ANA_FRO12M_CLK_ENA_W<'a, O> {
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ANA_FRO12M_CLK_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ANA_FRO12M_CLK_ENA_A::ENABLE)
    }
}
#[doc = "Field `XO_CAL_CLK_ENA` reader - Enable clock for cristal oscilator calibration."]
pub type XO_CAL_CLK_ENA_R = crate::BitReader<XO_CAL_CLK_ENA_A>;
#[doc = "Enable clock for cristal oscilator calibration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XO_CAL_CLK_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<XO_CAL_CLK_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: XO_CAL_CLK_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl XO_CAL_CLK_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XO_CAL_CLK_ENA_A {
        match self.bits {
            false => XO_CAL_CLK_ENA_A::DISABLE,
            true => XO_CAL_CLK_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == XO_CAL_CLK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == XO_CAL_CLK_ENA_A::ENABLE
    }
}
#[doc = "Field `XO_CAL_CLK_ENA` writer - Enable clock for cristal oscilator calibration."]
pub type XO_CAL_CLK_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLOCK_CTRL_SPEC, XO_CAL_CLK_ENA_A, O>;
impl<'a, const O: u8> XO_CAL_CLK_ENA_W<'a, O> {
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(XO_CAL_CLK_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(XO_CAL_CLK_ENA_A::ENABLE)
    }
}
#[doc = "Field `PLU_DEGLITCH_CLK_ENA` reader - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
pub type PLU_DEGLITCH_CLK_ENA_R = crate::BitReader<PLU_DEGLITCH_CLK_ENA_A>;
#[doc = "Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLU_DEGLITCH_CLK_ENA_A {
    #[doc = "0: The clock is not enabled."]
    DISABLE = 0,
    #[doc = "1: The clock is enabled."]
    ENABLE = 1,
}
impl From<PLU_DEGLITCH_CLK_ENA_A> for bool {
    #[inline(always)]
    fn from(variant: PLU_DEGLITCH_CLK_ENA_A) -> Self {
        variant as u8 != 0
    }
}
impl PLU_DEGLITCH_CLK_ENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLU_DEGLITCH_CLK_ENA_A {
        match self.bits {
            false => PLU_DEGLITCH_CLK_ENA_A::DISABLE,
            true => PLU_DEGLITCH_CLK_ENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PLU_DEGLITCH_CLK_ENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PLU_DEGLITCH_CLK_ENA_A::ENABLE
    }
}
#[doc = "Field `PLU_DEGLITCH_CLK_ENA` writer - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
pub type PLU_DEGLITCH_CLK_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLOCK_CTRL_SPEC, PLU_DEGLITCH_CLK_ENA_A, O>;
impl<'a, const O: u8> PLU_DEGLITCH_CLK_ENA_W<'a, O> {
    #[doc = "The clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PLU_DEGLITCH_CLK_ENA_A::DISABLE)
    }
    #[doc = "The clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PLU_DEGLITCH_CLK_ENA_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 1 - Enable XTAL32MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn xtal32mhz_freqm_ena(&self) -> XTAL32MHZ_FREQM_ENA_R {
        XTAL32MHZ_FREQM_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
    #[inline(always)]
    pub fn fro1mhz_utick_ena(&self) -> FRO1MHZ_UTICK_ENA_R {
        FRO1MHZ_UTICK_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable FRO 12MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn fro12mhz_freqm_ena(&self) -> FRO12MHZ_FREQM_ENA_R {
        FRO12MHZ_FREQM_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable FRO 96MHz clock for Frequency Measure module."]
    #[inline(always)]
    pub fn fro_hf_freqm_ena(&self) -> FRO_HF_FREQM_ENA_R {
        FRO_HF_FREQM_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable clock_in clock for clock module."]
    #[inline(always)]
    pub fn clkin_ena(&self) -> CLKIN_ENA_R {
        CLKIN_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable FRO 1MHz clock for clock muxing in clock gen."]
    #[inline(always)]
    pub fn fro1mhz_clk_ena(&self) -> FRO1MHZ_CLK_ENA_R {
        FRO1MHZ_CLK_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
    #[inline(always)]
    pub fn ana_fro12m_clk_ena(&self) -> ANA_FRO12M_CLK_ENA_R {
        ANA_FRO12M_CLK_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable clock for cristal oscilator calibration."]
    #[inline(always)]
    pub fn xo_cal_clk_ena(&self) -> XO_CAL_CLK_ENA_R {
        XO_CAL_CLK_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[inline(always)]
    pub fn plu_deglitch_clk_ena(&self) -> PLU_DEGLITCH_CLK_ENA_R {
        PLU_DEGLITCH_CLK_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable XTAL32MHz clock for Frequency Measure module."]
    #[inline(always)]
    #[must_use]
    pub fn xtal32mhz_freqm_ena(&mut self) -> XTAL32MHZ_FREQM_ENA_W<1> {
        XTAL32MHZ_FREQM_ENA_W::new(self)
    }
    #[doc = "Bit 2 - Enable FRO 1MHz clock for Frequency Measure module and for UTICK."]
    #[inline(always)]
    #[must_use]
    pub fn fro1mhz_utick_ena(&mut self) -> FRO1MHZ_UTICK_ENA_W<2> {
        FRO1MHZ_UTICK_ENA_W::new(self)
    }
    #[doc = "Bit 3 - Enable FRO 12MHz clock for Frequency Measure module."]
    #[inline(always)]
    #[must_use]
    pub fn fro12mhz_freqm_ena(&mut self) -> FRO12MHZ_FREQM_ENA_W<3> {
        FRO12MHZ_FREQM_ENA_W::new(self)
    }
    #[doc = "Bit 4 - Enable FRO 96MHz clock for Frequency Measure module."]
    #[inline(always)]
    #[must_use]
    pub fn fro_hf_freqm_ena(&mut self) -> FRO_HF_FREQM_ENA_W<4> {
        FRO_HF_FREQM_ENA_W::new(self)
    }
    #[doc = "Bit 5 - Enable clock_in clock for clock module."]
    #[inline(always)]
    #[must_use]
    pub fn clkin_ena(&mut self) -> CLKIN_ENA_W<5> {
        CLKIN_ENA_W::new(self)
    }
    #[doc = "Bit 6 - Enable FRO 1MHz clock for clock muxing in clock gen."]
    #[inline(always)]
    #[must_use]
    pub fn fro1mhz_clk_ena(&mut self) -> FRO1MHZ_CLK_ENA_W<6> {
        FRO1MHZ_CLK_ENA_W::new(self)
    }
    #[doc = "Bit 7 - Enable FRO 12MHz clock for analog control of the FRO 192MHz."]
    #[inline(always)]
    #[must_use]
    pub fn ana_fro12m_clk_ena(&mut self) -> ANA_FRO12M_CLK_ENA_W<7> {
        ANA_FRO12M_CLK_ENA_W::new(self)
    }
    #[doc = "Bit 8 - Enable clock for cristal oscilator calibration."]
    #[inline(always)]
    #[must_use]
    pub fn xo_cal_clk_ena(&mut self) -> XO_CAL_CLK_ENA_W<8> {
        XO_CAL_CLK_ENA_W::new(self)
    }
    #[doc = "Bit 9 - Enable clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[inline(always)]
    #[must_use]
    pub fn plu_deglitch_clk_ena(&mut self) -> PLU_DEGLITCH_CLK_ENA_W<9> {
        PLU_DEGLITCH_CLK_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_ctrl](index.html) module"]
pub struct CLOCK_CTRL_SPEC;
impl crate::RegisterSpec for CLOCK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_ctrl::R](R) reader structure"]
impl crate::Readable for CLOCK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_ctrl::W](W) writer structure"]
impl crate::Writable for CLOCK_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCK_CTRL to value 0x01"]
impl crate::Resettable for CLOCK_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}

#[doc = "Register `FRO192M_CTRL` reader"]
pub struct R(crate::R<FRO192M_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRO192M_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRO192M_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRO192M_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRO192M_CTRL` writer"]
pub struct W(crate::W<FRO192M_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRO192M_CTRL_SPEC>;
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
impl From<crate::W<FRO192M_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRO192M_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA_12MHZCLK` reader - 12 MHz clock control."]
pub type ENA_12MHZCLK_R = crate::BitReader<ENA_12MHZCLK_A>;
#[doc = "12 MHz clock control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA_12MHZCLK_A {
    #[doc = "0: 12 MHz clock is disabled."]
    DISABLE = 0,
    #[doc = "1: 12 MHz clock is enabled."]
    ENABLE = 1,
}
impl From<ENA_12MHZCLK_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_12MHZCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA_12MHZCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_12MHZCLK_A {
        match self.bits {
            false => ENA_12MHZCLK_A::DISABLE,
            true => ENA_12MHZCLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENA_12MHZCLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENA_12MHZCLK_A::ENABLE
    }
}
#[doc = "Field `ENA_12MHZCLK` writer - 12 MHz clock control."]
pub type ENA_12MHZCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRO192M_CTRL_SPEC, ENA_12MHZCLK_A, O>;
impl<'a, const O: u8> ENA_12MHZCLK_W<'a, O> {
    #[doc = "12 MHz clock is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENA_12MHZCLK_A::DISABLE)
    }
    #[doc = "12 MHz clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_12MHZCLK_A::ENABLE)
    }
}
#[doc = "Field `ENA_48MHZCLK` reader - 48 MHz clock control."]
pub type ENA_48MHZCLK_R = crate::BitReader<ENA_48MHZCLK_A>;
#[doc = "48 MHz clock control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA_48MHZCLK_A {
    #[doc = "1: 48 MHz clock is enabled."]
    ENABLE = 1,
}
impl From<ENA_48MHZCLK_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_48MHZCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA_48MHZCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENA_48MHZCLK_A> {
        match self.bits {
            true => Some(ENA_48MHZCLK_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENA_48MHZCLK_A::ENABLE
    }
}
#[doc = "Field `ENA_48MHZCLK` writer - 48 MHz clock control."]
pub type ENA_48MHZCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRO192M_CTRL_SPEC, ENA_48MHZCLK_A, O>;
impl<'a, const O: u8> ENA_48MHZCLK_W<'a, O> {
    #[doc = "48 MHz clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_48MHZCLK_A::ENABLE)
    }
}
#[doc = "Field `DAC_TRIM` reader - Frequency trim."]
pub type DAC_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_TRIM` writer - Frequency trim."]
pub type DAC_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRO192M_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `USBCLKADJ` reader - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
pub type USBCLKADJ_R = crate::BitReader<bool>;
#[doc = "Field `USBCLKADJ` writer - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
pub type USBCLKADJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRO192M_CTRL_SPEC, bool, O>;
#[doc = "Field `USBMODCHG` reader - If it reads as 1 when reading the DAC_TRIM field and USBCLKADJ=1, it should be re-read until it is 0."]
pub type USBMODCHG_R = crate::BitReader<bool>;
#[doc = "Field `ENA_96MHZCLK` reader - 96 MHz clock control."]
pub type ENA_96MHZCLK_R = crate::BitReader<ENA_96MHZCLK_A>;
#[doc = "96 MHz clock control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA_96MHZCLK_A {
    #[doc = "0: 96 MHz clock is disabled."]
    DISABLE = 0,
    #[doc = "1: 96 MHz clock is enabled."]
    ENABLE = 1,
}
impl From<ENA_96MHZCLK_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_96MHZCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA_96MHZCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_96MHZCLK_A {
        match self.bits {
            false => ENA_96MHZCLK_A::DISABLE,
            true => ENA_96MHZCLK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENA_96MHZCLK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENA_96MHZCLK_A::ENABLE
    }
}
#[doc = "Field `ENA_96MHZCLK` writer - 96 MHz clock control."]
pub type ENA_96MHZCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRO192M_CTRL_SPEC, ENA_96MHZCLK_A, O>;
impl<'a, const O: u8> ENA_96MHZCLK_W<'a, O> {
    #[doc = "96 MHz clock is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENA_96MHZCLK_A::DISABLE)
    }
    #[doc = "96 MHz clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENA_96MHZCLK_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 14 - 12 MHz clock control."]
    #[inline(always)]
    pub fn ena_12mhzclk(&self) -> ENA_12MHZCLK_R {
        ENA_12MHZCLK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 48 MHz clock control."]
    #[inline(always)]
    pub fn ena_48mhzclk(&self) -> ENA_48MHZCLK_R {
        ENA_48MHZCLK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn dac_trim(&self) -> DAC_TRIM_R {
        DAC_TRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline(always)]
    pub fn usbclkadj(&self) -> USBCLKADJ_R {
        USBCLKADJ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - If it reads as 1 when reading the DAC_TRIM field and USBCLKADJ=1, it should be re-read until it is 0."]
    #[inline(always)]
    pub fn usbmodchg(&self) -> USBMODCHG_R {
        USBMODCHG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline(always)]
    pub fn ena_96mhzclk(&self) -> ENA_96MHZCLK_R {
        ENA_96MHZCLK_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - 12 MHz clock control."]
    #[inline(always)]
    #[must_use]
    pub fn ena_12mhzclk(&mut self) -> ENA_12MHZCLK_W<14> {
        ENA_12MHZCLK_W::new(self)
    }
    #[doc = "Bit 15 - 48 MHz clock control."]
    #[inline(always)]
    #[must_use]
    pub fn ena_48mhzclk(&mut self) -> ENA_48MHZCLK_W<15> {
        ENA_48MHZCLK_W::new(self)
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    #[must_use]
    pub fn dac_trim(&mut self) -> DAC_TRIM_W<16> {
        DAC_TRIM_W::new(self)
    }
    #[doc = "Bit 24 - If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline(always)]
    #[must_use]
    pub fn usbclkadj(&mut self) -> USBCLKADJ_W<24> {
        USBCLKADJ_W::new(self)
    }
    #[doc = "Bit 30 - 96 MHz clock control."]
    #[inline(always)]
    #[must_use]
    pub fn ena_96mhzclk(&mut self) -> ENA_96MHZCLK_W<30> {
        ENA_96MHZCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "192MHz Free Running OScillator (FRO) Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fro192m_ctrl](index.html) module"]
pub struct FRO192M_CTRL_SPEC;
impl crate::RegisterSpec for FRO192M_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fro192m_ctrl::R](R) reader structure"]
impl crate::Readable for FRO192M_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fro192m_ctrl::W](W) writer structure"]
impl crate::Writable for FRO192M_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRO192M_CTRL to value 0x0080_d01a"]
impl crate::Resettable for FRO192M_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_d01a;
}

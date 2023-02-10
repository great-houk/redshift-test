#[doc = "Register `FIFOTRIG` reader"]
pub struct R(crate::R<FIFOTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOTRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOTRIG` writer"]
pub struct W(crate::W<FIFOTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOTRIG_SPEC>;
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
impl From<crate::W<FIFOTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXLVLENA` reader - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
pub type TXLVLENA_R = crate::BitReader<TXLVLENA_A>;
#[doc = "Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXLVLENA_A {
    #[doc = "0: Transmit FIFO level does not generate a FIFO level trigger."]
    DISABLED = 0,
    #[doc = "1: An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    ENABLED = 1,
}
impl From<TXLVLENA_A> for bool {
    #[inline(always)]
    fn from(variant: TXLVLENA_A) -> Self {
        variant as u8 != 0
    }
}
impl TXLVLENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXLVLENA_A {
        match self.bits {
            false => TXLVLENA_A::DISABLED,
            true => TXLVLENA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXLVLENA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXLVLENA_A::ENABLED
    }
}
#[doc = "Field `TXLVLENA` writer - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
pub type TXLVLENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOTRIG_SPEC, TXLVLENA_A, O>;
impl<'a, const O: u8> TXLVLENA_W<'a, O> {
    #[doc = "Transmit FIFO level does not generate a FIFO level trigger."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXLVLENA_A::DISABLED)
    }
    #[doc = "An trigger will be generated if the transmit FIFO level reaches the value specified by the TXLVL field in this register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXLVLENA_A::ENABLED)
    }
}
#[doc = "Field `RXLVLENA` reader - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
pub type RXLVLENA_R = crate::BitReader<RXLVLENA_A>;
#[doc = "Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXLVLENA_A {
    #[doc = "0: Receive FIFO level does not generate a FIFO level trigger."]
    DISABLED = 0,
    #[doc = "1: An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    ENABLED = 1,
}
impl From<RXLVLENA_A> for bool {
    #[inline(always)]
    fn from(variant: RXLVLENA_A) -> Self {
        variant as u8 != 0
    }
}
impl RXLVLENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXLVLENA_A {
        match self.bits {
            false => RXLVLENA_A::DISABLED,
            true => RXLVLENA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXLVLENA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXLVLENA_A::ENABLED
    }
}
#[doc = "Field `RXLVLENA` writer - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
pub type RXLVLENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOTRIG_SPEC, RXLVLENA_A, O>;
impl<'a, const O: u8> RXLVLENA_W<'a, O> {
    #[doc = "Receive FIFO level does not generate a FIFO level trigger."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXLVLENA_A::DISABLED)
    }
    #[doc = "An trigger will be generated if the receive FIFO level reaches the value specified by the RXLVL field in this register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXLVLENA_A::ENABLED)
    }
}
#[doc = "Field `TXLVL` reader - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
pub type TXLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXLVL` writer - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
pub type TXLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOTRIG_SPEC, u8, u8, 4, O>;
#[doc = "Field `RXLVL` reader - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
pub type RXLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXLVL` writer - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
pub type RXLVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFOTRIG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline(always)]
    pub fn txlvlena(&self) -> TXLVLENA_R {
        TXLVLENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline(always)]
    pub fn rxlvlena(&self) -> RXLVLENA_R {
        RXLVLENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline(always)]
    #[must_use]
    pub fn txlvlena(&mut self) -> TXLVLENA_W<0> {
        TXLVLENA_W::new(self)
    }
    #[doc = "Bit 1 - Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline(always)]
    #[must_use]
    pub fn rxlvlena(&mut self) -> RXLVLENA_W<1> {
        RXLVLENA_W::new(self)
    }
    #[doc = "Bits 8:11 - Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline(always)]
    #[must_use]
    pub fn txlvl(&mut self) -> TXLVL_W<8> {
        TXLVL_W::new(self)
    }
    #[doc = "Bits 16:19 - Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    #[must_use]
    pub fn rxlvl(&mut self) -> RXLVL_W<16> {
        RXLVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO trigger settings for interrupt and DMA request.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifotrig](index.html) module"]
pub struct FIFOTRIG_SPEC;
impl crate::RegisterSpec for FIFOTRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifotrig::R](R) reader structure"]
impl crate::Readable for FIFOTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifotrig::W](W) writer structure"]
impl crate::Writable for FIFOTRIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOTRIG to value 0"]
impl crate::Resettable for FIFOTRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXBRKEN` reader - Break Enable."]
pub type TXBRKEN_R = crate::BitReader<TXBRKEN_A>;
#[doc = "Break Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXBRKEN_A {
    #[doc = "0: Normal operation."]
    NORMAL = 0,
    #[doc = "1: Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    CONTINOUS = 1,
}
impl From<TXBRKEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXBRKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXBRKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXBRKEN_A {
        match self.bits {
            false => TXBRKEN_A::NORMAL,
            true => TXBRKEN_A::CONTINOUS,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TXBRKEN_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `CONTINOUS`"]
    #[inline(always)]
    pub fn is_continous(&self) -> bool {
        *self == TXBRKEN_A::CONTINOUS
    }
}
#[doc = "Field `TXBRKEN` writer - Break Enable."]
pub type TXBRKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, TXBRKEN_A, O>;
impl<'a, const O: u8> TXBRKEN_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TXBRKEN_A::NORMAL)
    }
    #[doc = "Continuous break. Continuous break is sent immediately when this bit is set, and remains until this bit is cleared. A break may be sent without danger of corrupting any currently transmitting character if the transmitter is first disabled (TXDIS in CTL is set) and then waiting for the transmitter to be disabled (TXDISINT in STAT = 1) before writing 1 to TXBRKEN."]
    #[inline(always)]
    pub fn continous(self) -> &'a mut W {
        self.variant(TXBRKEN_A::CONTINOUS)
    }
}
#[doc = "Field `ADDRDET` reader - Enable address detect mode."]
pub type ADDRDET_R = crate::BitReader<ADDRDET_A>;
#[doc = "Enable address detect mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRDET_A {
    #[doc = "0: Disabled. The USART presents all incoming data."]
    DISABLED = 0,
    #[doc = "1: Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    ENABLED = 1,
}
impl From<ADDRDET_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRDET_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRDET_A {
        match self.bits {
            false => ADDRDET_A::DISABLED,
            true => ADDRDET_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRDET_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRDET_A::ENABLED
    }
}
#[doc = "Field `ADDRDET` writer - Enable address detect mode."]
pub type ADDRDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, ADDRDET_A, O>;
impl<'a, const O: u8> ADDRDET_W<'a, O> {
    #[doc = "Disabled. The USART presents all incoming data."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRDET_A::DISABLED)
    }
    #[doc = "Enabled. The USART receiver ignores incoming data that does not have the most significant bit of the data (typically the 9th bit) = 1. When the data MSB bit = 1, the receiver treats the incoming data normally, generating a received data interrupt. Software can then check the data to see if this is an address that should be handled. If it is, the ADDRDET bit is cleared by software and further incoming data is handled normally."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRDET_A::ENABLED)
    }
}
#[doc = "Field `TXDIS` reader - Transmit Disable."]
pub type TXDIS_R = crate::BitReader<TXDIS_A>;
#[doc = "Transmit Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDIS_A {
    #[doc = "0: Not disabled. USART transmitter is not disabled."]
    ENABLED = 0,
    #[doc = "1: Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    DISABLED = 1,
}
impl From<TXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: TXDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDIS_A {
        match self.bits {
            false => TXDIS_A::ENABLED,
            true => TXDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDIS_A::DISABLED
    }
}
#[doc = "Field `TXDIS` writer - Transmit Disable."]
pub type TXDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, TXDIS_A, O>;
impl<'a, const O: u8> TXDIS_W<'a, O> {
    #[doc = "Not disabled. USART transmitter is not disabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDIS_A::ENABLED)
    }
    #[doc = "Disabled. USART transmitter is disabled after any character currently being transmitted is complete. This feature can be used to facilitate software flow control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDIS_A::DISABLED)
    }
}
#[doc = "Field `CC` reader - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
pub type CC_R = crate::BitReader<CC_A>;
#[doc = "Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC_A {
    #[doc = "0: Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    CLOCK_ON_CHARACTER = 0,
    #[doc = "1: Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    CONTINOUS_CLOCK = 1,
}
impl From<CC_A> for bool {
    #[inline(always)]
    fn from(variant: CC_A) -> Self {
        variant as u8 != 0
    }
}
impl CC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC_A {
        match self.bits {
            false => CC_A::CLOCK_ON_CHARACTER,
            true => CC_A::CONTINOUS_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `CLOCK_ON_CHARACTER`"]
    #[inline(always)]
    pub fn is_clock_on_character(&self) -> bool {
        *self == CC_A::CLOCK_ON_CHARACTER
    }
    #[doc = "Checks if the value of the field is `CONTINOUS_CLOCK`"]
    #[inline(always)]
    pub fn is_continous_clock(&self) -> bool {
        *self == CC_A::CONTINOUS_CLOCK
    }
}
#[doc = "Field `CC` writer - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
pub type CC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, CC_A, O>;
impl<'a, const O: u8> CC_W<'a, O> {
    #[doc = "Clock on character. In synchronous mode, SCLK cycles only when characters are being sent on Un_TXD or to complete a character that is being received."]
    #[inline(always)]
    pub fn clock_on_character(self) -> &'a mut W {
        self.variant(CC_A::CLOCK_ON_CHARACTER)
    }
    #[doc = "Continuous clock. SCLK runs continuously in synchronous mode, allowing characters to be received on Un_RxD independently from transmission on Un_TXD)."]
    #[inline(always)]
    pub fn continous_clock(self) -> &'a mut W {
        self.variant(CC_A::CONTINOUS_CLOCK)
    }
}
#[doc = "Field `CLRCCONRX` reader - Clear Continuous Clock."]
pub type CLRCCONRX_R = crate::BitReader<CLRCCONRX_A>;
#[doc = "Clear Continuous Clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRCCONRX_A {
    #[doc = "0: No effect. No effect on the CC bit."]
    NO_EFFECT = 0,
    #[doc = "1: Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    AUTO_CLEAR = 1,
}
impl From<CLRCCONRX_A> for bool {
    #[inline(always)]
    fn from(variant: CLRCCONRX_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRCCONRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRCCONRX_A {
        match self.bits {
            false => CLRCCONRX_A::NO_EFFECT,
            true => CLRCCONRX_A::AUTO_CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == CLRCCONRX_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `AUTO_CLEAR`"]
    #[inline(always)]
    pub fn is_auto_clear(&self) -> bool {
        *self == CLRCCONRX_A::AUTO_CLEAR
    }
}
#[doc = "Field `CLRCCONRX` writer - Clear Continuous Clock."]
pub type CLRCCONRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, CLRCCONRX_A, O>;
impl<'a, const O: u8> CLRCCONRX_W<'a, O> {
    #[doc = "No effect. No effect on the CC bit."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(CLRCCONRX_A::NO_EFFECT)
    }
    #[doc = "Auto-clear. The CC bit is automatically cleared when a complete character has been received. This bit is cleared at the same time."]
    #[inline(always)]
    pub fn auto_clear(self) -> &'a mut W {
        self.variant(CLRCCONRX_A::AUTO_CLEAR)
    }
}
#[doc = "Field `AUTOBAUD` reader - Autobaud enable."]
pub type AUTOBAUD_R = crate::BitReader<AUTOBAUD_A>;
#[doc = "Autobaud enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOBAUD_A {
    #[doc = "0: Disabled. USART is in normal operating mode."]
    DISABLED = 0,
    #[doc = "1: Enabled. USART is in autobaud mode. This bit should only be set when the USART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR."]
    ENABLED = 1,
}
impl From<AUTOBAUD_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOBAUD_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOBAUD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOBAUD_A {
        match self.bits {
            false => AUTOBAUD_A::DISABLED,
            true => AUTOBAUD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOBAUD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOBAUD_A::ENABLED
    }
}
#[doc = "Field `AUTOBAUD` writer - Autobaud enable."]
pub type AUTOBAUD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, AUTOBAUD_A, O>;
impl<'a, const O: u8> AUTOBAUD_W<'a, O> {
    #[doc = "Disabled. USART is in normal operating mode."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOBAUD_A::DISABLED)
    }
    #[doc = "Enabled. USART is in autobaud mode. This bit should only be set when the USART receiver is idle. The first start bit of RX is measured and used the update the BRG register to match the received data rate. AUTOBAUD is cleared once this process is complete, or if there is an AERR."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOBAUD_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 1 - Break Enable."]
    #[inline(always)]
    pub fn txbrken(&self) -> TXBRKEN_R {
        TXBRKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable address detect mode."]
    #[inline(always)]
    pub fn addrdet(&self) -> ADDRDET_R {
        ADDRDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Disable."]
    #[inline(always)]
    pub fn txdis(&self) -> TXDIS_R {
        TXDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear Continuous Clock."]
    #[inline(always)]
    pub fn clrcconrx(&self) -> CLRCCONRX_R {
        CLRCCONRX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Autobaud enable."]
    #[inline(always)]
    pub fn autobaud(&self) -> AUTOBAUD_R {
        AUTOBAUD_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Break Enable."]
    #[inline(always)]
    #[must_use]
    pub fn txbrken(&mut self) -> TXBRKEN_W<1> {
        TXBRKEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable address detect mode."]
    #[inline(always)]
    #[must_use]
    pub fn addrdet(&mut self) -> ADDRDET_W<2> {
        ADDRDET_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Disable."]
    #[inline(always)]
    #[must_use]
    pub fn txdis(&mut self) -> TXDIS_W<6> {
        TXDIS_W::new(self)
    }
    #[doc = "Bit 8 - Continuous Clock generation. By default, SCLK is only output while data is being transmitted in synchronous mode."]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CC_W<8> {
        CC_W::new(self)
    }
    #[doc = "Bit 9 - Clear Continuous Clock."]
    #[inline(always)]
    #[must_use]
    pub fn clrcconrx(&mut self) -> CLRCCONRX_W<9> {
        CLRCCONRX_W::new(self)
    }
    #[doc = "Bit 16 - Autobaud enable."]
    #[inline(always)]
    #[must_use]
    pub fn autobaud(&mut self) -> AUTOBAUD_W<16> {
        AUTOBAUD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Control register. USART control settings that are more likely to change during operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

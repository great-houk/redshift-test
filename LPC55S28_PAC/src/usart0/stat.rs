#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXIDLE` reader - Receiver Idle. When 0, indicates that the receiver is currently in the process of receiving data. When 1, indicates that the receiver is not currently in the process of receiving data."]
pub type RXIDLE_R = crate::BitReader<bool>;
#[doc = "Field `TXIDLE` reader - Transmitter Idle. When 0, indicates that the transmitter is currently in the process of sending data.When 1, indicate that the transmitter is not currently in the process of sending data."]
pub type TXIDLE_R = crate::BitReader<bool>;
#[doc = "Field `CTS` reader - This bit reflects the current state of the CTS signal, regardless of the setting of the CTSEN bit in the CFG register. This will be the value of the CTS input pin unless loopback mode is enabled."]
pub type CTS_R = crate::BitReader<bool>;
#[doc = "Field `DELTACTS` writer - This bit is set when a change in the state is detected for the CTS flag above. This bit is cleared by software."]
pub type DELTACTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `TXDISSTAT` reader - Transmitter Disabled Status flag. When 1, this bit indicates that the USART transmitter is fully idle after being disabled via the TXDIS bit in the CFG register (TXDIS = 1)."]
pub type TXDISSTAT_R = crate::BitReader<bool>;
#[doc = "Field `RXBRK` reader - Received Break. This bit reflects the current state of the receiver break detection logic. It is set when the Un_RXD pin remains low for 16 bit times. Note that FRAMERRINT will also be set when this condition occurs because the stop bit(s) for the character would be missing. RXBRK is cleared when the Un_RXD pin goes high."]
pub type RXBRK_R = crate::BitReader<bool>;
#[doc = "Field `DELTARXBRK` writer - This bit is set when a change in the state of receiver break detection occurs. Cleared by software."]
pub type DELTARXBRK_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `START` writer - This bit is set when a start is detected on the receiver input. Its purpose is primarily to allow wake-up from Deep-sleep or Power-down mode immediately when a start is detected. Cleared by software."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `FRAMERRINT` writer - Framing Error interrupt flag. This flag is set when a character is received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
pub type FRAMERRINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `PARITYERRINT` writer - Parity Error interrupt flag. This flag is set when a parity error is detected in a received character."]
pub type PARITYERRINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RXNOISEINT` writer - Received Noise interrupt flag. Three samples of received data are taken in order to determine the value of each received data bit, except in synchronous mode. This acts as a noise filter if one sample disagrees. This flag is set when a received data bit contains one disagreeing sample. This could indicate line noise, a baud rate or character format mismatch, or loss of synchronization during data reception."]
pub type RXNOISEINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `ABERR` writer - Auto baud Error. An auto baud error can occur if the BRG counts to its limit before the end of the start bit that is being measured, essentially an auto baud time-out."]
pub type ABERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Receiver Idle. When 0, indicates that the receiver is currently in the process of receiving data. When 1, indicates that the receiver is not currently in the process of receiving data."]
    #[inline(always)]
    pub fn rxidle(&self) -> RXIDLE_R {
        RXIDLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Idle. When 0, indicates that the transmitter is currently in the process of sending data.When 1, indicate that the transmitter is not currently in the process of sending data."]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit reflects the current state of the CTS signal, regardless of the setting of the CTSEN bit in the CFG register. This will be the value of the CTS input pin unless loopback mode is enabled."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Disabled Status flag. When 1, this bit indicates that the USART transmitter is fully idle after being disabled via the TXDIS bit in the CFG register (TXDIS = 1)."]
    #[inline(always)]
    pub fn txdisstat(&self) -> TXDISSTAT_R {
        TXDISSTAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Received Break. This bit reflects the current state of the receiver break detection logic. It is set when the Un_RXD pin remains low for 16 bit times. Note that FRAMERRINT will also be set when this condition occurs because the stop bit(s) for the character would be missing. RXBRK is cleared when the Un_RXD pin goes high."]
    #[inline(always)]
    pub fn rxbrk(&self) -> RXBRK_R {
        RXBRK_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - This bit is set when a change in the state is detected for the CTS flag above. This bit is cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn deltacts(&mut self) -> DELTACTS_W<5> {
        DELTACTS_W::new(self)
    }
    #[doc = "Bit 11 - This bit is set when a change in the state of receiver break detection occurs. Cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn deltarxbrk(&mut self) -> DELTARXBRK_W<11> {
        DELTARXBRK_W::new(self)
    }
    #[doc = "Bit 12 - This bit is set when a start is detected on the receiver input. Its purpose is primarily to allow wake-up from Deep-sleep or Power-down mode immediately when a start is detected. Cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<12> {
        START_W::new(self)
    }
    #[doc = "Bit 13 - Framing Error interrupt flag. This flag is set when a character is received with a missing stop bit at the expected location. This could be an indication of a baud rate or configuration mismatch with the transmitting source."]
    #[inline(always)]
    #[must_use]
    pub fn framerrint(&mut self) -> FRAMERRINT_W<13> {
        FRAMERRINT_W::new(self)
    }
    #[doc = "Bit 14 - Parity Error interrupt flag. This flag is set when a parity error is detected in a received character."]
    #[inline(always)]
    #[must_use]
    pub fn parityerrint(&mut self) -> PARITYERRINT_W<14> {
        PARITYERRINT_W::new(self)
    }
    #[doc = "Bit 15 - Received Noise interrupt flag. Three samples of received data are taken in order to determine the value of each received data bit, except in synchronous mode. This acts as a noise filter if one sample disagrees. This flag is set when a received data bit contains one disagreeing sample. This could indicate line noise, a baud rate or character format mismatch, or loss of synchronization during data reception."]
    #[inline(always)]
    #[must_use]
    pub fn rxnoiseint(&mut self) -> RXNOISEINT_W<15> {
        RXNOISEINT_W::new(self)
    }
    #[doc = "Bit 16 - Auto baud Error. An auto baud error can occur if the BRG counts to its limit before the end of the start bit that is being measured, essentially an auto baud time-out."]
    #[inline(always)]
    #[must_use]
    pub fn aberr(&mut self) -> ABERR_W<16> {
        ABERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0x0a"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}

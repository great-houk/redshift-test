#[doc = "Register `FIFOCFG` reader"]
pub struct R(crate::R<FIFOCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOCFG` writer"]
pub struct W(crate::W<FIFOCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOCFG_SPEC>;
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
impl From<crate::W<FIFOCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLETX` reader - Enable the transmit FIFO."]
pub type ENABLETX_R = crate::BitReader<ENABLETX_A>;
#[doc = "Enable the transmit FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLETX_A {
    #[doc = "0: The transmit FIFO is not enabled."]
    DISABLED = 0,
    #[doc = "1: The transmit FIFO is enabled."]
    ENABLED = 1,
}
impl From<ENABLETX_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLETX_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLETX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLETX_A {
        match self.bits {
            false => ENABLETX_A::DISABLED,
            true => ENABLETX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLETX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLETX_A::ENABLED
    }
}
#[doc = "Field `ENABLETX` writer - Enable the transmit FIFO."]
pub type ENABLETX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, ENABLETX_A, O>;
impl<'a, const O: u8> ENABLETX_W<'a, O> {
    #[doc = "The transmit FIFO is not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLETX_A::DISABLED)
    }
    #[doc = "The transmit FIFO is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLETX_A::ENABLED)
    }
}
#[doc = "Field `ENABLERX` reader - Enable the receive FIFO."]
pub type ENABLERX_R = crate::BitReader<ENABLERX_A>;
#[doc = "Enable the receive FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLERX_A {
    #[doc = "0: The receive FIFO is not enabled."]
    DISABLED = 0,
    #[doc = "1: The receive FIFO is enabled."]
    ENABLED = 1,
}
impl From<ENABLERX_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLERX_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLERX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLERX_A {
        match self.bits {
            false => ENABLERX_A::DISABLED,
            true => ENABLERX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLERX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLERX_A::ENABLED
    }
}
#[doc = "Field `ENABLERX` writer - Enable the receive FIFO."]
pub type ENABLERX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, ENABLERX_A, O>;
impl<'a, const O: u8> ENABLERX_W<'a, O> {
    #[doc = "The receive FIFO is not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLERX_A::DISABLED)
    }
    #[doc = "The receive FIFO is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLERX_A::ENABLED)
    }
}
#[doc = "Field `SIZE` reader - FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART."]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMATX` reader - DMA configuration for transmit."]
pub type DMATX_R = crate::BitReader<DMATX_A>;
#[doc = "DMA configuration for transmit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMATX_A {
    #[doc = "0: DMA is not used for the transmit function."]
    DISABLED = 0,
    #[doc = "1: Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED = 1,
}
impl From<DMATX_A> for bool {
    #[inline(always)]
    fn from(variant: DMATX_A) -> Self {
        variant as u8 != 0
    }
}
impl DMATX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMATX_A {
        match self.bits {
            false => DMATX_A::DISABLED,
            true => DMATX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMATX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMATX_A::ENABLED
    }
}
#[doc = "Field `DMATX` writer - DMA configuration for transmit."]
pub type DMATX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, DMATX_A, O>;
impl<'a, const O: u8> DMATX_W<'a, O> {
    #[doc = "DMA is not used for the transmit function."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMATX_A::DISABLED)
    }
    #[doc = "Trigger DMA for the transmit function if the FIFO is not full. Generally, data interrupts would be disabled if DMA is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMATX_A::ENABLED)
    }
}
#[doc = "Field `DMARX` reader - DMA configuration for receive."]
pub type DMARX_R = crate::BitReader<DMARX_A>;
#[doc = "DMA configuration for receive.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMARX_A {
    #[doc = "0: DMA is not used for the receive function."]
    DISABLED = 0,
    #[doc = "1: Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    ENABLED = 1,
}
impl From<DMARX_A> for bool {
    #[inline(always)]
    fn from(variant: DMARX_A) -> Self {
        variant as u8 != 0
    }
}
impl DMARX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMARX_A {
        match self.bits {
            false => DMARX_A::DISABLED,
            true => DMARX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMARX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMARX_A::ENABLED
    }
}
#[doc = "Field `DMARX` writer - DMA configuration for receive."]
pub type DMARX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, DMARX_A, O>;
impl<'a, const O: u8> DMARX_W<'a, O> {
    #[doc = "DMA is not used for the receive function."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMARX_A::DISABLED)
    }
    #[doc = "Trigger DMA for the receive function if the FIFO is not empty. Generally, data interrupts would be disabled if DMA is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMARX_A::ENABLED)
    }
}
#[doc = "Field `WAKETX` reader - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
pub type WAKETX_R = crate::BitReader<WAKETX_A>;
#[doc = "Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKETX_A {
    #[doc = "0: Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED = 0,
    #[doc = "1: A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    ENABLED = 1,
}
impl From<WAKETX_A> for bool {
    #[inline(always)]
    fn from(variant: WAKETX_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKETX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKETX_A {
        match self.bits {
            false => WAKETX_A::DISABLED,
            true => WAKETX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKETX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKETX_A::ENABLED
    }
}
#[doc = "Field `WAKETX` writer - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
pub type WAKETX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, WAKETX_A, O>;
impl<'a, const O: u8> WAKETX_W<'a, O> {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKETX_A::DISABLED)
    }
    #[doc = "A device wake-up for DMA will occur if the transmit FIFO level reaches the value specified by TXLVL in FIFOTRIG, even when the TXLVL interrupt is not enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKETX_A::ENABLED)
    }
}
#[doc = "Field `WAKERX` reader - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
pub type WAKERX_R = crate::BitReader<WAKERX_A>;
#[doc = "Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKERX_A {
    #[doc = "0: Only enabled interrupts will wake up the device form reduced power modes."]
    DISABLED = 0,
    #[doc = "1: A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    ENABLED = 1,
}
impl From<WAKERX_A> for bool {
    #[inline(always)]
    fn from(variant: WAKERX_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKERX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKERX_A {
        match self.bits {
            false => WAKERX_A::DISABLED,
            true => WAKERX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKERX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKERX_A::ENABLED
    }
}
#[doc = "Field `WAKERX` writer - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
pub type WAKERX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, WAKERX_A, O>;
impl<'a, const O: u8> WAKERX_W<'a, O> {
    #[doc = "Only enabled interrupts will wake up the device form reduced power modes."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKERX_A::DISABLED)
    }
    #[doc = "A device wake-up for DMA will occur if the receive FIFO level reaches the value specified by RXLVL in FIFOTRIG, even when the RXLVL interrupt is not enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKERX_A::ENABLED)
    }
}
#[doc = "Field `EMPTYTX` reader - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
pub type EMPTYTX_R = crate::BitReader<bool>;
#[doc = "Field `EMPTYTX` writer - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
pub type EMPTYTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, bool, O>;
#[doc = "Field `EMPTYRX` reader - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
pub type EMPTYRX_R = crate::BitReader<bool>;
#[doc = "Field `EMPTYRX` writer - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
pub type EMPTYRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable the transmit FIFO."]
    #[inline(always)]
    pub fn enabletx(&self) -> ENABLETX_R {
        ENABLETX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the receive FIFO."]
    #[inline(always)]
    pub fn enablerx(&self) -> ENABLERX_R {
        ENABLERX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 12 - DMA configuration for transmit."]
    #[inline(always)]
    pub fn dmatx(&self) -> DMATX_R {
        DMATX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA configuration for receive."]
    #[inline(always)]
    pub fn dmarx(&self) -> DMARX_R {
        DMARX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn waketx(&self) -> WAKETX_R {
        WAKETX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn wakerx(&self) -> WAKERX_R {
        WAKERX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline(always)]
    pub fn emptytx(&self) -> EMPTYTX_R {
        EMPTYTX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline(always)]
    pub fn emptyrx(&self) -> EMPTYRX_R {
        EMPTYRX_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn enabletx(&mut self) -> ENABLETX_W<0> {
        ENABLETX_W::new(self)
    }
    #[doc = "Bit 1 - Enable the receive FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn enablerx(&mut self) -> ENABLERX_W<1> {
        ENABLERX_W::new(self)
    }
    #[doc = "Bit 12 - DMA configuration for transmit."]
    #[inline(always)]
    #[must_use]
    pub fn dmatx(&mut self) -> DMATX_W<12> {
        DMATX_W::new(self)
    }
    #[doc = "Bit 13 - DMA configuration for receive."]
    #[inline(always)]
    #[must_use]
    pub fn dmarx(&mut self) -> DMARX_W<13> {
        DMARX_W::new(self)
    }
    #[doc = "Bit 14 - Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    #[must_use]
    pub fn waketx(&mut self) -> WAKETX_W<14> {
        WAKETX_W::new(self)
    }
    #[doc = "Bit 15 - Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    #[must_use]
    pub fn wakerx(&mut self) -> WAKERX_W<15> {
        WAKERX_W::new(self)
    }
    #[doc = "Bit 16 - Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline(always)]
    #[must_use]
    pub fn emptytx(&mut self) -> EMPTYTX_W<16> {
        EMPTYTX_W::new(self)
    }
    #[doc = "Bit 17 - Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline(always)]
    #[must_use]
    pub fn emptyrx(&mut self) -> EMPTYRX_W<17> {
        EMPTYRX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO configuration and enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifocfg](index.html) module"]
pub struct FIFOCFG_SPEC;
impl crate::RegisterSpec for FIFOCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifocfg::R](R) reader structure"]
impl crate::Readable for FIFOCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifocfg::W](W) writer structure"]
impl crate::Writable for FIFOCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOCFG to value 0"]
impl crate::Resettable for FIFOCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

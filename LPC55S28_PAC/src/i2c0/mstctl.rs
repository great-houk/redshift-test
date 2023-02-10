#[doc = "Register `MSTCTL` reader"]
pub struct R(crate::R<MSTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTCTL` writer"]
pub struct W(crate::W<MSTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTCTL_SPEC>;
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
impl From<crate::W<MSTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master Continue. This bit is write-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTCONTINUE_AW {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    CONTINUE = 1,
}
impl From<MSTCONTINUE_AW> for bool {
    #[inline(always)]
    fn from(variant: MSTCONTINUE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTCONTINUE` writer - Master Continue. This bit is write-only."]
pub type MSTCONTINUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTCTL_SPEC, MSTCONTINUE_AW, O>;
impl<'a, const O: u8> MSTCONTINUE_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTCONTINUE_AW::NO_EFFECT)
    }
    #[doc = "Continue. Informs the Master function to continue to the next operation. This must done after writing transmit data, reading received data, or any other housekeeping related to the next bus operation."]
    #[inline(always)]
    pub fn continue_(self) -> &'a mut W {
        self.variant(MSTCONTINUE_AW::CONTINUE)
    }
}
#[doc = "Field `MSTSTART` reader - Master Start control. This bit is write-only."]
pub type MSTSTART_R = crate::BitReader<MSTSTART_A>;
#[doc = "Master Start control. This bit is write-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTSTART_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Start. A Start will be generated on the I2C bus at the next allowed time."]
    START = 1,
}
impl From<MSTSTART_A> for bool {
    #[inline(always)]
    fn from(variant: MSTSTART_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSTART_A {
        match self.bits {
            false => MSTSTART_A::NO_EFFECT,
            true => MSTSTART_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTSTART_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == MSTSTART_A::START
    }
}
#[doc = "Field `MSTSTART` writer - Master Start control. This bit is write-only."]
pub type MSTSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTCTL_SPEC, MSTSTART_A, O>;
impl<'a, const O: u8> MSTSTART_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTSTART_A::NO_EFFECT)
    }
    #[doc = "Start. A Start will be generated on the I2C bus at the next allowed time."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(MSTSTART_A::START)
    }
}
#[doc = "Field `MSTSTOP` reader - Master Stop control. This bit is write-only."]
pub type MSTSTOP_R = crate::BitReader<MSTSTOP_A>;
#[doc = "Master Stop control. This bit is write-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTSTOP_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    STOP = 1,
}
impl From<MSTSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: MSTSTOP_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTSTOP_A {
        match self.bits {
            false => MSTSTOP_A::NO_EFFECT,
            true => MSTSTOP_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == MSTSTOP_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == MSTSTOP_A::STOP
    }
}
#[doc = "Field `MSTSTOP` writer - Master Stop control. This bit is write-only."]
pub type MSTSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTCTL_SPEC, MSTSTOP_A, O>;
impl<'a, const O: u8> MSTSTOP_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(MSTSTOP_A::NO_EFFECT)
    }
    #[doc = "Stop. A Stop will be generated on the I2C bus at the next allowed time, preceded by a NACK to the slave if the master is receiving data from the slave (Master Receiver mode)."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(MSTSTOP_A::STOP)
    }
}
#[doc = "Field `MSTDMA` reader - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
pub type MSTDMA_R = crate::BitReader<MSTDMA_A>;
#[doc = "Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTDMA_A {
    #[doc = "0: Disable. No DMA requests are generated for master operation."]
    DISABLED = 0,
    #[doc = "1: Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically."]
    ENABLED = 1,
}
impl From<MSTDMA_A> for bool {
    #[inline(always)]
    fn from(variant: MSTDMA_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTDMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTDMA_A {
        match self.bits {
            false => MSTDMA_A::DISABLED,
            true => MSTDMA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTDMA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTDMA_A::ENABLED
    }
}
#[doc = "Field `MSTDMA` writer - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
pub type MSTDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSTCTL_SPEC, MSTDMA_A, O>;
impl<'a, const O: u8> MSTDMA_W<'a, O> {
    #[doc = "Disable. No DMA requests are generated for master operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTDMA_A::DISABLED)
    }
    #[doc = "Enable. A DMA request is generated for I2C master data operations. When this I2C master is generating Acknowledge bits in Master Receiver mode, the acknowledge is generated automatically."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTDMA_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 1 - Master Start control. This bit is write-only."]
    #[inline(always)]
    pub fn mststart(&self) -> MSTSTART_R {
        MSTSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Stop control. This bit is write-only."]
    #[inline(always)]
    pub fn mststop(&self) -> MSTSTOP_R {
        MSTSTOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[inline(always)]
    pub fn mstdma(&self) -> MSTDMA_R {
        MSTDMA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Continue. This bit is write-only."]
    #[inline(always)]
    #[must_use]
    pub fn mstcontinue(&mut self) -> MSTCONTINUE_W<0> {
        MSTCONTINUE_W::new(self)
    }
    #[doc = "Bit 1 - Master Start control. This bit is write-only."]
    #[inline(always)]
    #[must_use]
    pub fn mststart(&mut self) -> MSTSTART_W<1> {
        MSTSTART_W::new(self)
    }
    #[doc = "Bit 2 - Master Stop control. This bit is write-only."]
    #[inline(always)]
    #[must_use]
    pub fn mststop(&mut self) -> MSTSTOP_W<2> {
        MSTSTOP_W::new(self)
    }
    #[doc = "Bit 3 - Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[inline(always)]
    #[must_use]
    pub fn mstdma(&mut self) -> MSTDMA_W<3> {
        MSTDMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstctl](index.html) module"]
pub struct MSTCTL_SPEC;
impl crate::RegisterSpec for MSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstctl::R](R) reader structure"]
impl crate::Readable for MSTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstctl::W](W) writer structure"]
impl crate::Writable for MSTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSTCTL to value 0"]
impl crate::Resettable for MSTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `CTLSTAT` reader"]
pub struct R(crate::R<CTLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALIDPENDING` reader - Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel."]
pub type VALIDPENDING_R = crate::BitReader<VALIDPENDING_A>;
#[doc = "Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VALIDPENDING_A {
    #[doc = "0: No effect. No effect on DMA operation."]
    NO_EFFECT = 0,
    #[doc = "1: Valid pending."]
    VALID_PENDING = 1,
}
impl From<VALIDPENDING_A> for bool {
    #[inline(always)]
    fn from(variant: VALIDPENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl VALIDPENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALIDPENDING_A {
        match self.bits {
            false => VALIDPENDING_A::NO_EFFECT,
            true => VALIDPENDING_A::VALID_PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == VALIDPENDING_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `VALID_PENDING`"]
    #[inline(always)]
    pub fn is_valid_pending(&self) -> bool {
        *self == VALIDPENDING_A::VALID_PENDING
    }
}
#[doc = "Field `TRIG` reader - Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1."]
pub type TRIG_R = crate::BitReader<TRIG_A>;
#[doc = "Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG_A {
    #[doc = "0: Not triggered. The trigger for this DMA channel is not set. DMA operations will not be carried out."]
    NOT_TRIGGERED = 0,
    #[doc = "1: Triggered. The trigger for this DMA channel is set. DMA operations will be carried out."]
    TRIGGERED = 1,
}
impl From<TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG_A {
        match self.bits {
            false => TRIG_A::NOT_TRIGGERED,
            true => TRIG_A::TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_TRIGGERED`"]
    #[inline(always)]
    pub fn is_not_triggered(&self) -> bool {
        *self == TRIG_A::NOT_TRIGGERED
    }
    #[doc = "Checks if the value of the field is `TRIGGERED`"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TRIG_A::TRIGGERED
    }
}
impl R {
    #[doc = "Bit 0 - Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel."]
    #[inline(always)]
    pub fn validpending(&self) -> VALIDPENDING_R {
        VALIDPENDING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1."]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Control and status register for DMA channel .\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlstat](index.html) module"]
pub struct CTLSTAT_SPEC;
impl crate::RegisterSpec for CTLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctlstat::R](R) reader structure"]
impl crate::Readable for CTLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTLSTAT to value 0"]
impl crate::Resettable for CTLSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

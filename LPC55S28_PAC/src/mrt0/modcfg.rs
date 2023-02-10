#[doc = "Register `MODCFG` reader"]
pub struct R(crate::R<MODCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODCFG` writer"]
pub struct W(crate::W<MODCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODCFG_SPEC>;
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
impl From<crate::W<MODCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NOC` reader - Identifies the number of channels in this MRT.(4 channels on this device.)"]
pub type NOC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NOC` writer - Identifies the number of channels in this MRT.(4 channels on this device.)"]
pub type NOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `NOB` reader - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
pub type NOB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NOB` writer - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
pub type NOB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODCFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `MULTITASK` reader - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
pub type MULTITASK_R = crate::BitReader<MULTITASK_A>;
#[doc = "Selects the operating mode for the INUSE flags and the IDLE_CH register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MULTITASK_A {
    #[doc = "0: Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    HARDWARE_STATUS_MODE = 0,
    #[doc = "1: Multi-task mode."]
    MULTI_TASK_MODE = 1,
}
impl From<MULTITASK_A> for bool {
    #[inline(always)]
    fn from(variant: MULTITASK_A) -> Self {
        variant as u8 != 0
    }
}
impl MULTITASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MULTITASK_A {
        match self.bits {
            false => MULTITASK_A::HARDWARE_STATUS_MODE,
            true => MULTITASK_A::MULTI_TASK_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `HARDWARE_STATUS_MODE`"]
    #[inline(always)]
    pub fn is_hardware_status_mode(&self) -> bool {
        *self == MULTITASK_A::HARDWARE_STATUS_MODE
    }
    #[doc = "Checks if the value of the field is `MULTI_TASK_MODE`"]
    #[inline(always)]
    pub fn is_multi_task_mode(&self) -> bool {
        *self == MULTITASK_A::MULTI_TASK_MODE
    }
}
#[doc = "Field `MULTITASK` writer - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
pub type MULTITASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODCFG_SPEC, MULTITASK_A, O>;
impl<'a, const O: u8> MULTITASK_W<'a, O> {
    #[doc = "Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    #[inline(always)]
    pub fn hardware_status_mode(self) -> &'a mut W {
        self.variant(MULTITASK_A::HARDWARE_STATUS_MODE)
    }
    #[doc = "Multi-task mode."]
    #[inline(always)]
    pub fn multi_task_mode(self) -> &'a mut W {
        self.variant(MULTITASK_A::MULTI_TASK_MODE)
    }
}
impl R {
    #[doc = "Bits 0:3 - Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline(always)]
    pub fn noc(&self) -> NOC_R {
        NOC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
    #[inline(always)]
    pub fn nob(&self) -> NOB_R {
        NOB_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[inline(always)]
    pub fn multitask(&self) -> MULTITASK_R {
        MULTITASK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline(always)]
    #[must_use]
    pub fn noc(&mut self) -> NOC_W<0> {
        NOC_W::new(self)
    }
    #[doc = "Bits 4:8 - Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
    #[inline(always)]
    #[must_use]
    pub fn nob(&mut self) -> NOB_W<4> {
        NOB_W::new(self)
    }
    #[doc = "Bit 31 - Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[inline(always)]
    #[must_use]
    pub fn multitask(&mut self) -> MULTITASK_W<31> {
        MULTITASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modcfg](index.html) module"]
pub struct MODCFG_SPEC;
impl crate::RegisterSpec for MODCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modcfg::R](R) reader structure"]
impl crate::Readable for MODCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modcfg::W](W) writer structure"]
impl crate::Writable for MODCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODCFG to value 0x0173"]
impl crate::Resettable for MODCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0173;
}

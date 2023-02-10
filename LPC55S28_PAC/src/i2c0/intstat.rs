#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MSTPENDING` reader - Master Pending."]
pub type MSTPENDING_R = crate::BitReader<bool>;
#[doc = "Field `MSTARBLOSS` reader - Master Arbitration Loss flag."]
pub type MSTARBLOSS_R = crate::BitReader<bool>;
#[doc = "Field `MSTSTSTPERR` reader - Master Start/Stop Error flag."]
pub type MSTSTSTPERR_R = crate::BitReader<bool>;
#[doc = "Field `SLVPENDING` reader - Slave Pending."]
pub type SLVPENDING_R = crate::BitReader<bool>;
#[doc = "Field `SLVNOTSTR` reader - Slave Not Stretching status."]
pub type SLVNOTSTR_R = crate::BitReader<bool>;
#[doc = "Field `SLVDESEL` reader - Slave Deselected flag."]
pub type SLVDESEL_R = crate::BitReader<bool>;
#[doc = "Field `MONRDY` reader - Monitor Ready."]
pub type MONRDY_R = crate::BitReader<bool>;
#[doc = "Field `MONOV` reader - Monitor Overflow flag."]
pub type MONOV_R = crate::BitReader<bool>;
#[doc = "Field `MONIDLE` reader - Monitor Idle flag."]
pub type MONIDLE_R = crate::BitReader<bool>;
#[doc = "Field `EVENTTIMEOUT` reader - Event time-out Interrupt flag."]
pub type EVENTTIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `SCLTIMEOUT` reader - SCL time-out Interrupt flag."]
pub type SCLTIMEOUT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Master Pending."]
    #[inline(always)]
    pub fn mstpending(&self) -> MSTPENDING_R {
        MSTPENDING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Master Arbitration Loss flag."]
    #[inline(always)]
    pub fn mstarbloss(&self) -> MSTARBLOSS_R {
        MSTARBLOSS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Master Start/Stop Error flag."]
    #[inline(always)]
    pub fn mstststperr(&self) -> MSTSTSTPERR_R {
        MSTSTSTPERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave Pending."]
    #[inline(always)]
    pub fn slvpending(&self) -> SLVPENDING_R {
        SLVPENDING_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Slave Not Stretching status."]
    #[inline(always)]
    pub fn slvnotstr(&self) -> SLVNOTSTR_R {
        SLVNOTSTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Slave Deselected flag."]
    #[inline(always)]
    pub fn slvdesel(&self) -> SLVDESEL_R {
        SLVDESEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Monitor Ready."]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Monitor Overflow flag."]
    #[inline(always)]
    pub fn monov(&self) -> MONOV_R {
        MONOV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Monitor Idle flag."]
    #[inline(always)]
    pub fn monidle(&self) -> MONIDLE_R {
        MONIDLE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Event time-out Interrupt flag."]
    #[inline(always)]
    pub fn eventtimeout(&self) -> EVENTTIMEOUT_R {
        EVENTTIMEOUT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCL time-out Interrupt flag."]
    #[inline(always)]
    pub fn scltimeout(&self) -> SCLTIMEOUT_R {
        SCLTIMEOUT_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTAT to value 0x0801"]
impl crate::Resettable for INTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0801;
}

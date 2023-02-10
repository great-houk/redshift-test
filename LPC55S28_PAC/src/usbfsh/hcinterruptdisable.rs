#[doc = "Register `HCINTERRUPTDISABLE` reader"]
pub struct R(crate::R<HCINTERRUPTDISABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINTERRUPTDISABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINTERRUPTDISABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINTERRUPTDISABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINTERRUPTDISABLE` writer"]
pub struct W(crate::W<HCINTERRUPTDISABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINTERRUPTDISABLE_SPEC>;
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
impl From<crate::W<HCINTERRUPTDISABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINTERRUPTDISABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SO` reader - Scheduling Overrun interrupt."]
pub type SO_R = crate::BitReader<bool>;
#[doc = "Field `SO` writer - Scheduling Overrun interrupt."]
pub type SO_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTDISABLE_SPEC, bool, O>;
#[doc = "Field `WDH` reader - HcDoneHead Writeback interrupt."]
pub type WDH_R = crate::BitReader<bool>;
#[doc = "Field `WDH` writer - HcDoneHead Writeback interrupt."]
pub type WDH_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTDISABLE_SPEC, bool, O>;
#[doc = "Field `SF` reader - Start of Frame interrupt."]
pub type SF_R = crate::BitReader<bool>;
#[doc = "Field `SF` writer - Start of Frame interrupt."]
pub type SF_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTDISABLE_SPEC, bool, O>;
#[doc = "Field `RD` reader - Resume Detect interrupt."]
pub type RD_R = crate::BitReader<bool>;
#[doc = "Field `RD` writer - Resume Detect interrupt."]
pub type RD_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTDISABLE_SPEC, bool, O>;
#[doc = "Field `UE` reader - Unrecoverable Error interrupt."]
pub type UE_R = crate::BitReader<bool>;
#[doc = "Field `UE` writer - Unrecoverable Error interrupt."]
pub type UE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTDISABLE_SPEC, bool, O>;
#[doc = "Field `FNO` reader - Frame Number Overflow interrupt."]
pub type FNO_R = crate::BitReader<bool>;
#[doc = "Field `FNO` writer - Frame Number Overflow interrupt."]
pub type FNO_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTDISABLE_SPEC, bool, O>;
#[doc = "Field `RHSC` reader - Root Hub Status Change interrupt."]
pub type RHSC_R = crate::BitReader<bool>;
#[doc = "Field `RHSC` writer - Root Hub Status Change interrupt."]
pub type RHSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTDISABLE_SPEC, bool, O>;
#[doc = "Field `OC` reader - Ownership Change interrupt."]
pub type OC_R = crate::BitReader<bool>;
#[doc = "Field `OC` writer - Ownership Change interrupt."]
pub type OC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTDISABLE_SPEC, bool, O>;
#[doc = "Field `MIE` reader - A 0 written to this field is ignored by HC."]
pub type MIE_R = crate::BitReader<bool>;
#[doc = "Field `MIE` writer - A 0 written to this field is ignored by HC."]
pub type MIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTERRUPTDISABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Scheduling Overrun interrupt."]
    #[inline(always)]
    pub fn so(&self) -> SO_R {
        SO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HcDoneHead Writeback interrupt."]
    #[inline(always)]
    pub fn wdh(&self) -> WDH_R {
        WDH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start of Frame interrupt."]
    #[inline(always)]
    pub fn sf(&self) -> SF_R {
        SF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Resume Detect interrupt."]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Unrecoverable Error interrupt."]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Frame Number Overflow interrupt."]
    #[inline(always)]
    pub fn fno(&self) -> FNO_R {
        FNO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Root Hub Status Change interrupt."]
    #[inline(always)]
    pub fn rhsc(&self) -> RHSC_R {
        RHSC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 30 - Ownership Change interrupt."]
    #[inline(always)]
    pub fn oc(&self) -> OC_R {
        OC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - A 0 written to this field is ignored by HC."]
    #[inline(always)]
    pub fn mie(&self) -> MIE_R {
        MIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scheduling Overrun interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn so(&mut self) -> SO_W<0> {
        SO_W::new(self)
    }
    #[doc = "Bit 1 - HcDoneHead Writeback interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn wdh(&mut self) -> WDH_W<1> {
        WDH_W::new(self)
    }
    #[doc = "Bit 2 - Start of Frame interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sf(&mut self) -> SF_W<2> {
        SF_W::new(self)
    }
    #[doc = "Bit 3 - Resume Detect interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<3> {
        RD_W::new(self)
    }
    #[doc = "Bit 4 - Unrecoverable Error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<4> {
        UE_W::new(self)
    }
    #[doc = "Bit 5 - Frame Number Overflow interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn fno(&mut self) -> FNO_W<5> {
        FNO_W::new(self)
    }
    #[doc = "Bit 6 - Root Hub Status Change interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rhsc(&mut self) -> RHSC_W<6> {
        RHSC_W::new(self)
    }
    #[doc = "Bit 30 - Ownership Change interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn oc(&mut self) -> OC_W<30> {
        OC_W::new(self)
    }
    #[doc = "Bit 31 - A 0 written to this field is ignored by HC."]
    #[inline(always)]
    #[must_use]
    pub fn mie(&mut self) -> MIE_W<31> {
        MIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcinterruptdisable](index.html) module"]
pub struct HCINTERRUPTDISABLE_SPEC;
impl crate::RegisterSpec for HCINTERRUPTDISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcinterruptdisable::R](R) reader structure"]
impl crate::Readable for HCINTERRUPTDISABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcinterruptdisable::W](W) writer structure"]
impl crate::Writable for HCINTERRUPTDISABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINTERRUPTDISABLE to value 0"]
impl crate::Resettable for HCINTERRUPTDISABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

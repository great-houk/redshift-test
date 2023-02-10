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
#[doc = "Register `INTSTAT` writer"]
pub struct W(crate::W<INTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTSTAT_SPEC>;
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
impl From<crate::W<INTSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP0OUT` reader - Interrupt status register bit for the Control EP0 OUT direction."]
pub type EP0OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP0OUT` writer - Interrupt status register bit for the Control EP0 OUT direction."]
pub type EP0OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `EP0IN` reader - Interrupt status register bit for the Control EP0 IN direction."]
pub type EP0IN_R = crate::BitReader<bool>;
#[doc = "Field `EP0IN` writer - Interrupt status register bit for the Control EP0 IN direction."]
pub type EP0IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `EP1OUT` reader - Interrupt status register bit for the EP1 OUT direction."]
pub type EP1OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP1OUT` writer - Interrupt status register bit for the EP1 OUT direction."]
pub type EP1OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `EP1IN` reader - Interrupt status register bit for the EP1 IN direction."]
pub type EP1IN_R = crate::BitReader<bool>;
#[doc = "Field `EP1IN` writer - Interrupt status register bit for the EP1 IN direction."]
pub type EP1IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `EP2OUT` reader - Interrupt status register bit for the EP2 OUT direction."]
pub type EP2OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP2OUT` writer - Interrupt status register bit for the EP2 OUT direction."]
pub type EP2OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `EP2IN` reader - Interrupt status register bit for the EP2 IN direction."]
pub type EP2IN_R = crate::BitReader<bool>;
#[doc = "Field `EP2IN` writer - Interrupt status register bit for the EP2 IN direction."]
pub type EP2IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `EP3OUT` reader - Interrupt status register bit for the EP3 OUT direction."]
pub type EP3OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP3OUT` writer - Interrupt status register bit for the EP3 OUT direction."]
pub type EP3OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `EP3IN` reader - Interrupt status register bit for the EP3 IN direction."]
pub type EP3IN_R = crate::BitReader<bool>;
#[doc = "Field `EP3IN` writer - Interrupt status register bit for the EP3 IN direction."]
pub type EP3IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `EP4OUT` reader - Interrupt status register bit for the EP4 OUT direction."]
pub type EP4OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP4OUT` writer - Interrupt status register bit for the EP4 OUT direction."]
pub type EP4OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `EP4IN` reader - Interrupt status register bit for the EP4 IN direction."]
pub type EP4IN_R = crate::BitReader<bool>;
#[doc = "Field `EP4IN` writer - Interrupt status register bit for the EP4 IN direction."]
pub type EP4IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `EP5OUT` reader - Interrupt status register bit for the EP5 OUT direction."]
pub type EP5OUT_R = crate::BitReader<bool>;
#[doc = "Field `EP5OUT` writer - Interrupt status register bit for the EP5 OUT direction."]
pub type EP5OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `EP5IN` reader - Interrupt status register bit for the EP5 IN direction."]
pub type EP5IN_R = crate::BitReader<bool>;
#[doc = "Field `EP5IN` writer - Interrupt status register bit for the EP5 IN direction."]
pub type EP5IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `FRAME_INT` reader - Frame interrupt."]
pub type FRAME_INT_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_INT` writer - Frame interrupt."]
pub type FRAME_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
#[doc = "Field `DEV_INT` reader - Device status interrupt."]
pub type DEV_INT_R = crate::BitReader<bool>;
#[doc = "Field `DEV_INT` writer - Device status interrupt."]
pub type DEV_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt status register bit for the Control EP0 OUT direction."]
    #[inline(always)]
    pub fn ep0out(&self) -> EP0OUT_R {
        EP0OUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status register bit for the Control EP0 IN direction."]
    #[inline(always)]
    pub fn ep0in(&self) -> EP0IN_R {
        EP0IN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status register bit for the EP1 OUT direction."]
    #[inline(always)]
    pub fn ep1out(&self) -> EP1OUT_R {
        EP1OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status register bit for the EP1 IN direction."]
    #[inline(always)]
    pub fn ep1in(&self) -> EP1IN_R {
        EP1IN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status register bit for the EP2 OUT direction."]
    #[inline(always)]
    pub fn ep2out(&self) -> EP2OUT_R {
        EP2OUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt status register bit for the EP2 IN direction."]
    #[inline(always)]
    pub fn ep2in(&self) -> EP2IN_R {
        EP2IN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt status register bit for the EP3 OUT direction."]
    #[inline(always)]
    pub fn ep3out(&self) -> EP3OUT_R {
        EP3OUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt status register bit for the EP3 IN direction."]
    #[inline(always)]
    pub fn ep3in(&self) -> EP3IN_R {
        EP3IN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt status register bit for the EP4 OUT direction."]
    #[inline(always)]
    pub fn ep4out(&self) -> EP4OUT_R {
        EP4OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt status register bit for the EP4 IN direction."]
    #[inline(always)]
    pub fn ep4in(&self) -> EP4IN_R {
        EP4IN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt status register bit for the EP5 OUT direction."]
    #[inline(always)]
    pub fn ep5out(&self) -> EP5OUT_R {
        EP5OUT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt status register bit for the EP5 IN direction."]
    #[inline(always)]
    pub fn ep5in(&self) -> EP5IN_R {
        EP5IN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 30 - Frame interrupt."]
    #[inline(always)]
    pub fn frame_int(&self) -> FRAME_INT_R {
        FRAME_INT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Device status interrupt."]
    #[inline(always)]
    pub fn dev_int(&self) -> DEV_INT_R {
        DEV_INT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt status register bit for the Control EP0 OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep0out(&mut self) -> EP0OUT_W<0> {
        EP0OUT_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt status register bit for the Control EP0 IN direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep0in(&mut self) -> EP0IN_W<1> {
        EP0IN_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt status register bit for the EP1 OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep1out(&mut self) -> EP1OUT_W<2> {
        EP1OUT_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt status register bit for the EP1 IN direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep1in(&mut self) -> EP1IN_W<3> {
        EP1IN_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt status register bit for the EP2 OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep2out(&mut self) -> EP2OUT_W<4> {
        EP2OUT_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt status register bit for the EP2 IN direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep2in(&mut self) -> EP2IN_W<5> {
        EP2IN_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt status register bit for the EP3 OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep3out(&mut self) -> EP3OUT_W<6> {
        EP3OUT_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt status register bit for the EP3 IN direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep3in(&mut self) -> EP3IN_W<7> {
        EP3IN_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt status register bit for the EP4 OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep4out(&mut self) -> EP4OUT_W<8> {
        EP4OUT_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt status register bit for the EP4 IN direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep4in(&mut self) -> EP4IN_W<9> {
        EP4IN_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt status register bit for the EP5 OUT direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep5out(&mut self) -> EP5OUT_W<10> {
        EP5OUT_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt status register bit for the EP5 IN direction."]
    #[inline(always)]
    #[must_use]
    pub fn ep5in(&mut self) -> EP5IN_W<11> {
        EP5IN_W::new(self)
    }
    #[doc = "Bit 30 - Frame interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn frame_int(&mut self) -> FRAME_INT_W<30> {
        FRAME_INT_W::new(self)
    }
    #[doc = "Bit 31 - Device status interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dev_int(&mut self) -> DEV_INT_W<31> {
        DEV_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intstat::W](W) writer structure"]
impl crate::Writable for INTSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

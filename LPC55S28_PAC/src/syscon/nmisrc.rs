#[doc = "Register `NMISRC` reader"]
pub struct R(crate::R<NMISRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMISRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMISRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMISRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMISRC` writer"]
pub struct W(crate::W<NMISRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMISRC_SPEC>;
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
impl From<crate::W<NMISRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMISRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQCPU0` reader - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
pub type IRQCPU0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQCPU0` writer - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
pub type IRQCPU0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NMISRC_SPEC, u8, u8, 6, O>;
#[doc = "Field `IRQCPU1` reader - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU1, if enabled by NMIENCPU1."]
pub type IRQCPU1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQCPU1` writer - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU1, if enabled by NMIENCPU1."]
pub type IRQCPU1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NMISRC_SPEC, u8, u8, 6, O>;
#[doc = "Field `NMIENCPU1` reader - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
pub type NMIENCPU1_R = crate::BitReader<bool>;
#[doc = "Field `NMIENCPU1` writer - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
pub type NMIENCPU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, NMISRC_SPEC, bool, O>;
#[doc = "Field `NMIENCPU0` reader - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
pub type NMIENCPU0_R = crate::BitReader<bool>;
#[doc = "Field `NMIENCPU0` writer - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
pub type NMIENCPU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NMISRC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    pub fn irqcpu0(&self) -> IRQCPU0_R {
        IRQCPU0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU1, if enabled by NMIENCPU1."]
    #[inline(always)]
    pub fn irqcpu1(&self) -> IRQCPU1_R {
        IRQCPU1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
    #[inline(always)]
    pub fn nmiencpu1(&self) -> NMIENCPU1_R {
        NMIENCPU1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[inline(always)]
    pub fn nmiencpu0(&self) -> NMIENCPU0_R {
        NMIENCPU0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    #[must_use]
    pub fn irqcpu0(&mut self) -> IRQCPU0_W<0> {
        IRQCPU0_W::new(self)
    }
    #[doc = "Bits 8:13 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU1, if enabled by NMIENCPU1."]
    #[inline(always)]
    #[must_use]
    pub fn irqcpu1(&mut self) -> IRQCPU1_W<8> {
        IRQCPU1_W::new(self)
    }
    #[doc = "Bit 30 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
    #[inline(always)]
    #[must_use]
    pub fn nmiencpu1(&mut self) -> NMIENCPU1_W<30> {
        NMIENCPU1_W::new(self)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[inline(always)]
    #[must_use]
    pub fn nmiencpu0(&mut self) -> NMIENCPU0_W<31> {
        NMIENCPU0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NMI Source Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmisrc](index.html) module"]
pub struct NMISRC_SPEC;
impl crate::RegisterSpec for NMISRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmisrc::R](R) reader structure"]
impl crate::Readable for NMISRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmisrc::W](W) writer structure"]
impl crate::Writable for NMISRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NMISRC to value 0"]
impl crate::Resettable for NMISRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

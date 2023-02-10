#[doc = "Register `HALT` reader"]
pub struct R(crate::R<HALT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HALT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HALT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HALT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HALT` writer"]
pub struct W(crate::W<HALT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HALT_SPEC>;
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
impl From<crate::W<HALT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HALT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HALTMSK_L` reader - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type HALTMSK_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HALTMSK_L` writer - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type HALTMSK_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HALT_SPEC, u16, u16, 16, O>;
#[doc = "Field `HALTMSK_H` reader - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type HALTMSK_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HALTMSK_H` writer - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type HALTMSK_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HALT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_l(&self) -> HALTMSK_L_R {
        HALTMSK_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn haltmsk_h(&self) -> HALTMSK_H_R {
        HALTMSK_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn haltmsk_l(&mut self) -> HALTMSK_L_W<0> {
        HALTMSK_L_W::new(self)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn haltmsk_h(&mut self) -> HALTMSK_H_W<16> {
        HALTMSK_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT halt event select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [halt](index.html) module"]
pub struct HALT_SPEC;
impl crate::RegisterSpec for HALT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [halt::R](R) reader structure"]
impl crate::Readable for HALT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [halt::W](W) writer structure"]
impl crate::Writable for HALT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HALT to value 0"]
impl crate::Resettable for HALT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

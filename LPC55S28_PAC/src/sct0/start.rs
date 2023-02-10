#[doc = "Register `START` reader"]
pub struct R(crate::R<START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `START` writer"]
pub struct W(crate::W<START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<START_SPEC>;
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
impl From<crate::W<START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTMSK_L` reader - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type STARTMSK_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STARTMSK_L` writer - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type STARTMSK_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, START_SPEC, u16, u16, 16, O>;
#[doc = "Field `STARTMSK_H` reader - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type STARTMSK_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STARTMSK_H` writer - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
pub type STARTMSK_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, START_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn startmsk_l(&self) -> STARTMSK_L_R {
        STARTMSK_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn startmsk_h(&self) -> STARTMSK_H_R {
        STARTMSK_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn startmsk_l(&mut self) -> STARTMSK_L_W<0> {
        STARTMSK_L_W::new(self)
    }
    #[doc = "Bits 16:31 - If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn startmsk_h(&mut self) -> STARTMSK_H_W<16> {
        STARTMSK_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT start event select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [start](index.html) module"]
pub struct START_SPEC;
impl crate::RegisterSpec for START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [start::R](R) reader structure"]
impl crate::Readable for START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [start::W](W) writer structure"]
impl crate::Writable for START_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

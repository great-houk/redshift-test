#[doc = "Register `EVEN` reader"]
pub struct R(crate::R<EVEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVEN` writer"]
pub struct W(crate::W<EVEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVEN_SPEC>;
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
impl From<crate::W<EVEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEN` reader - The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type IEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IEN` writer - The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
pub type IEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IEN_W<0> {
        IEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT event interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [even](index.html) module"]
pub struct EVEN_SPEC;
impl crate::RegisterSpec for EVEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [even::R](R) reader structure"]
impl crate::Readable for EVEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [even::W](W) writer structure"]
impl crate::Writable for EVEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVEN to value 0"]
impl crate::Resettable for EVEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

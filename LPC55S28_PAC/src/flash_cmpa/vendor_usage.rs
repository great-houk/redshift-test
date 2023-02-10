#[doc = "Register `VENDOR_USAGE` reader"]
pub struct R(crate::R<VENDOR_USAGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VENDOR_USAGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VENDOR_USAGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VENDOR_USAGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VENDOR_USAGE` writer"]
pub struct W(crate::W<VENDOR_USAGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VENDOR_USAGE_SPEC>;
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
impl From<crate::W<VENDOR_USAGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VENDOR_USAGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VENDOR_USAGE` reader - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
pub type VENDOR_USAGE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VENDOR_USAGE` writer - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
pub type VENDOR_USAGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VENDOR_USAGE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
    #[inline(always)]
    pub fn vendor_usage(&self) -> VENDOR_USAGE_R {
        VENDOR_USAGE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
    #[inline(always)]
    #[must_use]
    pub fn vendor_usage(&mut self) -> VENDOR_USAGE_W<16> {
        VENDOR_USAGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vendor_usage](index.html) module"]
pub struct VENDOR_USAGE_SPEC;
impl crate::RegisterSpec for VENDOR_USAGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vendor_usage::R](R) reader structure"]
impl crate::Readable for VENDOR_USAGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vendor_usage::W](W) writer structure"]
impl crate::Writable for VENDOR_USAGE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VENDOR_USAGE to value 0"]
impl crate::Resettable for VENDOR_USAGE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `EPBUFCFG` reader"]
pub struct R(crate::R<EPBUFCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EPBUFCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EPBUFCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EPBUFCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EPBUFCFG` writer"]
pub struct W(crate::W<EPBUFCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EPBUFCFG_SPEC>;
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
impl From<crate::W<EPBUFCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EPBUFCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF_SB` reader - Buffer usage: This register has one bit per physical endpoint."]
pub type BUF_SB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BUF_SB` writer - Buffer usage: This register has one bit per physical endpoint."]
pub type BUF_SB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EPBUFCFG_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 2:11 - Buffer usage: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub fn buf_sb(&self) -> BUF_SB_R {
        BUF_SB_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:11 - Buffer usage: This register has one bit per physical endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn buf_sb(&mut self) -> BUF_SB_W<2> {
        BUF_SB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Buffer Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epbufcfg](index.html) module"]
pub struct EPBUFCFG_SPEC;
impl crate::RegisterSpec for EPBUFCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [epbufcfg::R](R) reader structure"]
impl crate::Readable for EPBUFCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [epbufcfg::W](W) writer structure"]
impl crate::Writable for EPBUFCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EPBUFCFG to value 0"]
impl crate::Resettable for EPBUFCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `HCDONEHEAD` reader"]
pub struct R(crate::R<HCDONEHEAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDONEHEAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDONEHEAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDONEHEAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCDONEHEAD` writer"]
pub struct W(crate::W<HCDONEHEAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCDONEHEAD_SPEC>;
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
impl From<crate::W<HCDONEHEAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCDONEHEAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DH` reader - DoneHead When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD."]
pub type DH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 4:31 - DoneHead When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD."]
    #[inline(always)]
    pub fn dh(&self) -> DH_R {
        DH_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the physical address of the last transfer descriptor added to the 'Done' queue\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdonehead](index.html) module"]
pub struct HCDONEHEAD_SPEC;
impl crate::RegisterSpec for HCDONEHEAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdonehead::R](R) reader structure"]
impl crate::Readable for HCDONEHEAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcdonehead::W](W) writer structure"]
impl crate::Writable for HCDONEHEAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCDONEHEAD to value 0"]
impl crate::Resettable for HCDONEHEAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

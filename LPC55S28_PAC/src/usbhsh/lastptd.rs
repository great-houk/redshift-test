#[doc = "Register `LASTPTD` reader"]
pub struct R(crate::R<LASTPTD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LASTPTD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LASTPTD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LASTPTD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LASTPTD` writer"]
pub struct W(crate::W<LASTPTD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LASTPTD_SPEC>;
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
impl From<crate::W<LASTPTD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LASTPTD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATL_LAST` reader - If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
pub type ATL_LAST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATL_LAST` writer - If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
pub type ATL_LAST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LASTPTD_SPEC, u8, u8, 5, O>;
#[doc = "Field `ISO_LAST` reader - This indicates the last PTD in the ISO list."]
pub type ISO_LAST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISO_LAST` writer - This indicates the last PTD in the ISO list."]
pub type ISO_LAST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LASTPTD_SPEC, u8, u8, 5, O>;
#[doc = "Field `INT_LAST` reader - This indicates the last PTD in the INT list."]
pub type INT_LAST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_LAST` writer - This indicates the last PTD in the INT list."]
pub type INT_LAST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LASTPTD_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
    #[inline(always)]
    pub fn atl_last(&self) -> ATL_LAST_R {
        ATL_LAST_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - This indicates the last PTD in the ISO list."]
    #[inline(always)]
    pub fn iso_last(&self) -> ISO_LAST_R {
        ISO_LAST_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - This indicates the last PTD in the INT list."]
    #[inline(always)]
    pub fn int_last(&self) -> INT_LAST_R {
        INT_LAST_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
    #[inline(always)]
    #[must_use]
    pub fn atl_last(&mut self) -> ATL_LAST_W<0> {
        ATL_LAST_W::new(self)
    }
    #[doc = "Bits 8:12 - This indicates the last PTD in the ISO list."]
    #[inline(always)]
    #[must_use]
    pub fn iso_last(&mut self) -> ISO_LAST_W<8> {
        ISO_LAST_W::new(self)
    }
    #[doc = "Bits 16:20 - This indicates the last PTD in the INT list."]
    #[inline(always)]
    #[must_use]
    pub fn int_last(&mut self) -> INT_LAST_W<16> {
        INT_LAST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Marks the last PTD in the list for ISO, INT and ATL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lastptd](index.html) module"]
pub struct LASTPTD_SPEC;
impl crate::RegisterSpec for LASTPTD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lastptd::R](R) reader structure"]
impl crate::Readable for LASTPTD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lastptd::W](W) writer structure"]
impl crate::Writable for LASTPTD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LASTPTD to value 0"]
impl crate::Resettable for LASTPTD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

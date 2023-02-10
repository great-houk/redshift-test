#[doc = "Register `ATLPTD` reader"]
pub struct R(crate::R<ATLPTD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATLPTD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATLPTD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATLPTD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATLPTD` writer"]
pub struct W(crate::W<ATLPTD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATLPTD_SPEC>;
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
impl From<crate::W<ATLPTD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATLPTD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATL_CUR` reader - This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
pub type ATL_CUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATL_CUR` writer - This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
pub type ATL_CUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATLPTD_SPEC, u8, u8, 5, O>;
#[doc = "Field `ATL_BASE` reader - Base address to be used by the hardware to find the start of the ATL list."]
pub type ATL_BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ATL_BASE` writer - Base address to be used by the hardware to find the start of the ATL list."]
pub type ATL_BASE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATLPTD_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bits 4:8 - This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
    #[inline(always)]
    pub fn atl_cur(&self) -> ATL_CUR_R {
        ATL_CUR_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:31 - Base address to be used by the hardware to find the start of the ATL list."]
    #[inline(always)]
    pub fn atl_base(&self) -> ATL_BASE_R {
        ATL_BASE_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 4:8 - This indicates the current PTD that is used by the hardware when it is processing the ATL list."]
    #[inline(always)]
    #[must_use]
    pub fn atl_cur(&mut self) -> ATL_CUR_W<4> {
        ATL_CUR_W::new(self)
    }
    #[doc = "Bits 9:31 - Base address to be used by the hardware to find the start of the ATL list."]
    #[inline(always)]
    #[must_use]
    pub fn atl_base(&mut self) -> ATL_BASE_W<9> {
        ATL_BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory base address where ATL PTD0 is stored\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atlptd](index.html) module"]
pub struct ATLPTD_SPEC;
impl crate::RegisterSpec for ATLPTD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atlptd::R](R) reader structure"]
impl crate::Readable for ATLPTD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atlptd::W](W) writer structure"]
impl crate::Writable for ATLPTD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATLPTD to value 0"]
impl crate::Resettable for ATLPTD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

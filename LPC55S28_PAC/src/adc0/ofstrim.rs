#[doc = "Register `OFSTRIM` reader"]
pub struct R(crate::R<OFSTRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFSTRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFSTRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFSTRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFSTRIM` writer"]
pub struct W(crate::W<OFSTRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFSTRIM_SPEC>;
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
impl From<crate::W<OFSTRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFSTRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFSTRIM_A` reader - Trim for offset"]
pub type OFSTRIM_A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFSTRIM_A` writer - Trim for offset"]
pub type OFSTRIM_A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFSTRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `OFSTRIM_B` reader - Trim for offset"]
pub type OFSTRIM_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFSTRIM_B` writer - Trim for offset"]
pub type OFSTRIM_B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFSTRIM_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_a(&self) -> OFSTRIM_A_R {
        OFSTRIM_A_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Trim for offset"]
    #[inline(always)]
    pub fn ofstrim_b(&self) -> OFSTRIM_B_R {
        OFSTRIM_B_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for offset"]
    #[inline(always)]
    #[must_use]
    pub fn ofstrim_a(&mut self) -> OFSTRIM_A_W<0> {
        OFSTRIM_A_W::new(self)
    }
    #[doc = "Bits 16:20 - Trim for offset"]
    #[inline(always)]
    #[must_use]
    pub fn ofstrim_b(&mut self) -> OFSTRIM_B_W<16> {
        OFSTRIM_B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Offset Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofstrim](index.html) module"]
pub struct OFSTRIM_SPEC;
impl crate::RegisterSpec for OFSTRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofstrim::R](R) reader structure"]
impl crate::Readable for OFSTRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofstrim::W](W) writer structure"]
impl crate::Writable for OFSTRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFSTRIM to value 0"]
impl crate::Resettable for OFSTRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

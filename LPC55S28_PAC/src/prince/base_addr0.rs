#[doc = "Register `BASE_ADDR0` reader"]
pub struct R(crate::R<BASE_ADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASE_ADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASE_ADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASE_ADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BASE_ADDR0` writer"]
pub struct W(crate::W<BASE_ADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BASE_ADDR0_SPEC>;
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
impl From<crate::W<BASE_ADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BASE_ADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR_FIXED` reader - Fixed portion of the base address of region 0."]
pub type ADDR_FIXED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR_PRG` reader - Programmable portion of the base address of region 0."]
pub type ADDR_PRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR_PRG` writer - Programmable portion of the base address of region 0."]
pub type ADDR_PRG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BASE_ADDR0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:17 - Fixed portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr_fixed(&self) -> ADDR_FIXED_R {
        ADDR_FIXED_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr_prg(&self) -> ADDR_PRG_R {
        ADDR_PRG_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    #[must_use]
    pub fn addr_prg(&mut self) -> ADDR_PRG_W<18> {
        ADDR_PRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Base Address for region 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base_addr0](index.html) module"]
pub struct BASE_ADDR0_SPEC;
impl crate::RegisterSpec for BASE_ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [base_addr0::R](R) reader structure"]
impl crate::Readable for BASE_ADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [base_addr0::W](W) writer structure"]
impl crate::Writable for BASE_ADDR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BASE_ADDR0 to value 0"]
impl crate::Resettable for BASE_ADDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

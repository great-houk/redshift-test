#[doc = "Register `DIR0` reader"]
pub struct R(crate::R<DIR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIR0` writer"]
pub struct W(crate::W<DIR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIR0_SPEC>;
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
impl From<crate::W<DIR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRP` reader - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
pub type DIRP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DIRP` writer - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
pub type DIRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[inline(always)]
    pub fn dirp(&self) -> DIRP_R {
        DIRP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[inline(always)]
    #[must_use]
    pub fn dirp(&mut self) -> DIRP_W<0> {
        DIRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direction registers for all port GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir0](index.html) module"]
pub struct DIR0_SPEC;
impl crate::RegisterSpec for DIR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dir0::R](R) reader structure"]
impl crate::Readable for DIR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dir0::W](W) writer structure"]
impl crate::Writable for DIR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIR0 to value 0"]
impl crate::Resettable for DIR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

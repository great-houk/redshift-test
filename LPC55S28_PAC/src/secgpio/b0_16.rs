#[doc = "Register `B0_16` reader"]
pub struct R(crate::R<B0_16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B0_16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B0_16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B0_16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `B0_16` writer"]
pub struct W(crate::W<B0_16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<B0_16_SPEC>;
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
impl From<crate::W<B0_16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<B0_16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBYTE` reader - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub type PBYTE_R = crate::BitReader<bool>;
#[doc = "Field `PBYTE` writer - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub type PBYTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, B0_16_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pbyte(&self) -> PBYTE_R {
        PBYTE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    #[must_use]
    pub fn pbyte(&mut self) -> PBYTE_W<0> {
        PBYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Byte pin registers for all port GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0_16](index.html) module"]
pub struct B0_16_SPEC;
impl crate::RegisterSpec for B0_16_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [b0_16::R](R) reader structure"]
impl crate::Readable for B0_16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [b0_16::W](W) writer structure"]
impl crate::Writable for B0_16_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets B0_16 to value 0"]
impl crate::Resettable for B0_16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

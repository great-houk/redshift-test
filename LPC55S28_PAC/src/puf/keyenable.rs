#[doc = "Register `KEYENABLE` reader"]
pub struct R(crate::R<KEYENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYENABLE` writer"]
pub struct W(crate::W<KEYENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYENABLE_SPEC>;
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
impl From<crate::W<KEYENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY0` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
pub type KEY0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY0` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
pub type KEY0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYENABLE_SPEC, u8, u8, 2, O>;
#[doc = "Field `KEY1` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
pub type KEY1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY1` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
pub type KEY1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYENABLE_SPEC, u8, u8, 2, O>;
#[doc = "Field `KEY2` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
pub type KEY2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY2` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
pub type KEY2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYENABLE_SPEC, u8, u8, 2, O>;
#[doc = "Field `KEY3` reader - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
pub type KEY3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY3` writer - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
pub type KEY3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYENABLE_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
    #[inline(always)]
    pub fn key0(&self) -> KEY0_R {
        KEY0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
    #[inline(always)]
    pub fn key1(&self) -> KEY1_R {
        KEY1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
    #[inline(always)]
    pub fn key2(&self) -> KEY2_R {
        KEY2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
    #[inline(always)]
    pub fn key3(&self) -> KEY3_R {
        KEY3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\""]
    #[inline(always)]
    #[must_use]
    pub fn key0(&mut self) -> KEY0_W<0> {
        KEY0_W::new(self)
    }
    #[doc = "Bits 2:3 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\""]
    #[inline(always)]
    #[must_use]
    pub fn key1(&mut self) -> KEY1_W<2> {
        KEY1_W::new(self)
    }
    #[doc = "Bits 4:5 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\""]
    #[inline(always)]
    #[must_use]
    pub fn key2(&mut self) -> KEY2_W<4> {
        KEY2_W::new(self)
    }
    #[doc = "Bits 6:7 - \"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\""]
    #[inline(always)]
    #[must_use]
    pub fn key3(&mut self) -> KEY3_W<6> {
        KEY3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyenable](index.html) module"]
pub struct KEYENABLE_SPEC;
impl crate::RegisterSpec for KEYENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyenable::R](R) reader structure"]
impl crate::Readable for KEYENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyenable::W](W) writer structure"]
impl crate::Writable for KEYENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYENABLE to value 0x55"]
impl crate::Resettable for KEYENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0x55;
}

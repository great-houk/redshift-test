#[doc = "Register `IDXBLK_H` reader"]
pub struct R(crate::R<IDXBLK_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDXBLK_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDXBLK_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDXBLK_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDXBLK_H` writer"]
pub struct W(crate::W<IDXBLK_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDXBLK_H_SPEC>;
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
impl From<crate::W<IDXBLK_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDXBLK_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDX8` reader - Use to block PUF index 8"]
pub type IDX8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX8` writer - Use to block PUF index 8"]
pub type IDX8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDX9` reader - Use to block PUF index 9"]
pub type IDX9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX9` writer - Use to block PUF index 9"]
pub type IDX9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDX10` reader - Use to block PUF index 10"]
pub type IDX10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX10` writer - Use to block PUF index 10"]
pub type IDX10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDX11` reader - Use to block PUF index 11"]
pub type IDX11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX11` writer - Use to block PUF index 11"]
pub type IDX11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDX12` reader - Use to block PUF index 12"]
pub type IDX12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX12` writer - Use to block PUF index 12"]
pub type IDX12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDX13` reader - Use to block PUF index 13"]
pub type IDX13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX13` writer - Use to block PUF index 13"]
pub type IDX13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDX14` reader - Use to block PUF index 14"]
pub type IDX14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX14` writer - Use to block PUF index 14"]
pub type IDX14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDX15` reader - Use to block PUF index 15"]
pub type IDX15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX15` writer - Use to block PUF index 15"]
pub type IDX15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, u8, 2, O>;
#[doc = "Field `LOCK_IDX` writer - Lock 8 to 15 PUF key indexes"]
pub type LOCK_IDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_H_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Use to block PUF index 8"]
    #[inline(always)]
    pub fn idx8(&self) -> IDX8_R {
        IDX8_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Use to block PUF index 9"]
    #[inline(always)]
    pub fn idx9(&self) -> IDX9_R {
        IDX9_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Use to block PUF index 10"]
    #[inline(always)]
    pub fn idx10(&self) -> IDX10_R {
        IDX10_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Use to block PUF index 11"]
    #[inline(always)]
    pub fn idx11(&self) -> IDX11_R {
        IDX11_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Use to block PUF index 12"]
    #[inline(always)]
    pub fn idx12(&self) -> IDX12_R {
        IDX12_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Use to block PUF index 13"]
    #[inline(always)]
    pub fn idx13(&self) -> IDX13_R {
        IDX13_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Use to block PUF index 14"]
    #[inline(always)]
    pub fn idx14(&self) -> IDX14_R {
        IDX14_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Use to block PUF index 15"]
    #[inline(always)]
    pub fn idx15(&self) -> IDX15_R {
        IDX15_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Use to block PUF index 8"]
    #[inline(always)]
    #[must_use]
    pub fn idx8(&mut self) -> IDX8_W<0> {
        IDX8_W::new(self)
    }
    #[doc = "Bits 2:3 - Use to block PUF index 9"]
    #[inline(always)]
    #[must_use]
    pub fn idx9(&mut self) -> IDX9_W<2> {
        IDX9_W::new(self)
    }
    #[doc = "Bits 4:5 - Use to block PUF index 10"]
    #[inline(always)]
    #[must_use]
    pub fn idx10(&mut self) -> IDX10_W<4> {
        IDX10_W::new(self)
    }
    #[doc = "Bits 6:7 - Use to block PUF index 11"]
    #[inline(always)]
    #[must_use]
    pub fn idx11(&mut self) -> IDX11_W<6> {
        IDX11_W::new(self)
    }
    #[doc = "Bits 8:9 - Use to block PUF index 12"]
    #[inline(always)]
    #[must_use]
    pub fn idx12(&mut self) -> IDX12_W<8> {
        IDX12_W::new(self)
    }
    #[doc = "Bits 10:11 - Use to block PUF index 13"]
    #[inline(always)]
    #[must_use]
    pub fn idx13(&mut self) -> IDX13_W<10> {
        IDX13_W::new(self)
    }
    #[doc = "Bits 12:13 - Use to block PUF index 14"]
    #[inline(always)]
    #[must_use]
    pub fn idx14(&mut self) -> IDX14_W<12> {
        IDX14_W::new(self)
    }
    #[doc = "Bits 14:15 - Use to block PUF index 15"]
    #[inline(always)]
    #[must_use]
    pub fn idx15(&mut self) -> IDX15_W<14> {
        IDX15_W::new(self)
    }
    #[doc = "Bits 30:31 - Lock 8 to 15 PUF key indexes"]
    #[inline(always)]
    #[must_use]
    pub fn lock_idx(&mut self) -> LOCK_IDX_W<30> {
        LOCK_IDX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idxblk_h](index.html) module"]
pub struct IDXBLK_H_SPEC;
impl crate::RegisterSpec for IDXBLK_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idxblk_h::R](R) reader structure"]
impl crate::Readable for IDXBLK_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idxblk_h::W](W) writer structure"]
impl crate::Writable for IDXBLK_H_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDXBLK_H to value 0x8000_aaaa"]
impl crate::Resettable for IDXBLK_H_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_aaaa;
}

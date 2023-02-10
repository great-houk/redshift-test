#[doc = "Register `IDXBLK_L_DP` reader"]
pub struct R(crate::R<IDXBLK_L_DP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDXBLK_L_DP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDXBLK_L_DP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDXBLK_L_DP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDXBLK_L_DP` writer"]
pub struct W(crate::W<IDXBLK_L_DP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDXBLK_L_DP_SPEC>;
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
impl From<crate::W<IDXBLK_L_DP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDXBLK_L_DP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDX1` reader - Use to block PUF index 1"]
pub type IDX1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX1` writer - Use to block PUF index 1"]
pub type IDX1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDX2` reader - Use to block PUF index 2"]
pub type IDX2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX2` writer - Use to block PUF index 2"]
pub type IDX2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDX3` reader - Use to block PUF index 3"]
pub type IDX3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX3` writer - Use to block PUF index 3"]
pub type IDX3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDX4` reader - Use to block PUF index 4"]
pub type IDX4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX4` writer - Use to block PUF index 4"]
pub type IDX4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDX5` reader - Use to block PUF index 5"]
pub type IDX5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX5` writer - Use to block PUF index 5"]
pub type IDX5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDX6` reader - Use to block PUF index 6"]
pub type IDX6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX6` writer - Use to block PUF index 6"]
pub type IDX6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, u8, 2, O>;
#[doc = "Field `IDX7` reader - Use to block PUF index 7"]
pub type IDX7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX7` writer - Use to block PUF index 7"]
pub type IDX7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDXBLK_L_DP_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 2:3 - Use to block PUF index 1"]
    #[inline(always)]
    pub fn idx1(&self) -> IDX1_R {
        IDX1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Use to block PUF index 2"]
    #[inline(always)]
    pub fn idx2(&self) -> IDX2_R {
        IDX2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Use to block PUF index 3"]
    #[inline(always)]
    pub fn idx3(&self) -> IDX3_R {
        IDX3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Use to block PUF index 4"]
    #[inline(always)]
    pub fn idx4(&self) -> IDX4_R {
        IDX4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Use to block PUF index 5"]
    #[inline(always)]
    pub fn idx5(&self) -> IDX5_R {
        IDX5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Use to block PUF index 6"]
    #[inline(always)]
    pub fn idx6(&self) -> IDX6_R {
        IDX6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Use to block PUF index 7"]
    #[inline(always)]
    pub fn idx7(&self) -> IDX7_R {
        IDX7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3 - Use to block PUF index 1"]
    #[inline(always)]
    #[must_use]
    pub fn idx1(&mut self) -> IDX1_W<2> {
        IDX1_W::new(self)
    }
    #[doc = "Bits 4:5 - Use to block PUF index 2"]
    #[inline(always)]
    #[must_use]
    pub fn idx2(&mut self) -> IDX2_W<4> {
        IDX2_W::new(self)
    }
    #[doc = "Bits 6:7 - Use to block PUF index 3"]
    #[inline(always)]
    #[must_use]
    pub fn idx3(&mut self) -> IDX3_W<6> {
        IDX3_W::new(self)
    }
    #[doc = "Bits 8:9 - Use to block PUF index 4"]
    #[inline(always)]
    #[must_use]
    pub fn idx4(&mut self) -> IDX4_W<8> {
        IDX4_W::new(self)
    }
    #[doc = "Bits 10:11 - Use to block PUF index 5"]
    #[inline(always)]
    #[must_use]
    pub fn idx5(&mut self) -> IDX5_W<10> {
        IDX5_W::new(self)
    }
    #[doc = "Bits 12:13 - Use to block PUF index 6"]
    #[inline(always)]
    #[must_use]
    pub fn idx6(&mut self) -> IDX6_W<12> {
        IDX6_W::new(self)
    }
    #[doc = "Bits 14:15 - Use to block PUF index 7"]
    #[inline(always)]
    #[must_use]
    pub fn idx7(&mut self) -> IDX7_W<14> {
        IDX7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idxblk_l_dp](index.html) module"]
pub struct IDXBLK_L_DP_SPEC;
impl crate::RegisterSpec for IDXBLK_L_DP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idxblk_l_dp::R](R) reader structure"]
impl crate::Readable for IDXBLK_L_DP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idxblk_l_dp::W](W) writer structure"]
impl crate::Writable for IDXBLK_L_DP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDXBLK_L_DP to value 0xaaaa"]
impl crate::Resettable for IDXBLK_L_DP_SPEC {
    const RESET_VALUE: Self::Ux = 0xaaaa;
}

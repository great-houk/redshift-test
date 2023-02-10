#[doc = "Register `ROTKH_REVOKE` reader"]
pub struct R(crate::R<ROTKH_REVOKE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROTKH_REVOKE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROTKH_REVOKE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROTKH_REVOKE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROTKH_REVOKE` writer"]
pub struct W(crate::W<ROTKH_REVOKE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROTKH_REVOKE_SPEC>;
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
impl From<crate::W<ROTKH_REVOKE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROTKH_REVOKE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RoTK0_EN` reader - RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RO_TK0_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RoTK0_EN` writer - RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RO_TK0_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROTKH_REVOKE_SPEC, u8, u8, 2, O>;
#[doc = "Field `RoTK1_EN` reader - RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RO_TK1_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RoTK1_EN` writer - RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RO_TK1_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROTKH_REVOKE_SPEC, u8, u8, 2, O>;
#[doc = "Field `RoTK2_EN` reader - RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RO_TK2_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RoTK2_EN` writer - RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RO_TK2_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROTKH_REVOKE_SPEC, u8, u8, 2, O>;
#[doc = "Field `RoTK3_EN` reader - RoT Key 3 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RO_TK3_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RoTK3_EN` writer - RoT Key 3 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
pub type RO_TK3_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROTKH_REVOKE_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk0_en(&self) -> RO_TK0_EN_R {
        RO_TK0_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk1_en(&self) -> RO_TK1_EN_R {
        RO_TK1_EN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk2_en(&self) -> RO_TK2_EN_R {
        RO_TK2_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - RoT Key 3 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub fn ro_tk3_en(&self) -> RO_TK3_EN_R {
        RO_TK3_EN_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    #[must_use]
    pub fn ro_tk0_en(&mut self) -> RO_TK0_EN_W<0> {
        RO_TK0_EN_W::new(self)
    }
    #[doc = "Bits 2:3 - RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    #[must_use]
    pub fn ro_tk1_en(&mut self) -> RO_TK1_EN_W<2> {
        RO_TK1_EN_W::new(self)
    }
    #[doc = "Bits 4:5 - RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    #[must_use]
    pub fn ro_tk2_en(&mut self) -> RO_TK2_EN_W<4> {
        RO_TK2_EN_W::new(self)
    }
    #[doc = "Bits 6:7 - RoT Key 3 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    #[must_use]
    pub fn ro_tk3_en(&mut self) -> RO_TK3_EN_W<6> {
        RO_TK3_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rotkh_revoke](index.html) module"]
pub struct ROTKH_REVOKE_SPEC;
impl crate::RegisterSpec for ROTKH_REVOKE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rotkh_revoke::R](R) reader structure"]
impl crate::Readable for ROTKH_REVOKE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rotkh_revoke::W](W) writer structure"]
impl crate::Writable for ROTKH_REVOKE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROTKH_REVOKE to value 0"]
impl crate::Resettable for ROTKH_REVOKE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

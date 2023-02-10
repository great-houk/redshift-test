#[doc = "Register `SECURE_BOOT_CFG` reader"]
pub struct R(crate::R<SECURE_BOOT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECURE_BOOT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECURE_BOOT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECURE_BOOT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECURE_BOOT_CFG` writer"]
pub struct W(crate::W<SECURE_BOOT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECURE_BOOT_CFG_SPEC>;
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
impl From<crate::W<SECURE_BOOT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECURE_BOOT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSA4K` reader - Use RSA4096 keys only. 00- RSA2048 keys 01, 10, 11 - RSA4096 keys"]
pub type RSA4K_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSA4K` writer - Use RSA4096 keys only. 00- RSA2048 keys 01, 10, 11 - RSA4096 keys"]
pub type RSA4K_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECURE_BOOT_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `DICE_ENC_NXP_CFG` reader - Include NXP area in DICE computation. 00 - not included 01, 10, 11 - included"]
pub type DICE_ENC_NXP_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DICE_ENC_NXP_CFG` writer - Include NXP area in DICE computation. 00 - not included 01, 10, 11 - included"]
pub type DICE_ENC_NXP_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_BOOT_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `DICE_CUST_CFG` reader - Include Customer factory area (including keys) in DICE computation. 00 - not included 01, 10, 11 - included"]
pub type DICE_CUST_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DICE_CUST_CFG` writer - Include Customer factory area (including keys) in DICE computation. 00 - not included 01, 10, 11 - included"]
pub type DICE_CUST_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_BOOT_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SKIP_DICE` reader - Skip DICE computation. 00 - Enable DICE 01,10,11 - Disable DICE"]
pub type SKIP_DICE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SKIP_DICE` writer - Skip DICE computation. 00 - Enable DICE 01,10,11 - Disable DICE"]
pub type SKIP_DICE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_BOOT_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `TZM_IMAGE_TYPE` reader - TrustZone-M mode. 00 - TZM mode in image header. 01 - Disable TZ-M. Boots to NonSecure. 10 - TZ-M enable boots to secure mode. 11 - Preset TZM checker from image header."]
pub type TZM_IMAGE_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZM_IMAGE_TYPE` writer - TrustZone-M mode. 00 - TZM mode in image header. 01 - Disable TZ-M. Boots to NonSecure. 10 - TZ-M enable boots to secure mode. 11 - Preset TZM checker from image header."]
pub type TZM_IMAGE_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_BOOT_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `BLOCK_SET_KEY` reader - Block PUF key code generation. 00 - Enable Key code generation 01, 10, 11 - Disable key code generation"]
pub type BLOCK_SET_KEY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLOCK_SET_KEY` writer - Block PUF key code generation. 00 - Enable Key code generation 01, 10, 11 - Disable key code generation"]
pub type BLOCK_SET_KEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_BOOT_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `BLOCK_ENROLL` reader - Block PUF enrollement. 00 - Enable enrollment mode 01, 10, 11 - Disable further enrollmnet"]
pub type BLOCK_ENROLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLOCK_ENROLL` writer - Block PUF enrollement. 00 - Enable enrollment mode 01, 10, 11 - Disable further enrollmnet"]
pub type BLOCK_ENROLL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_BOOT_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `DICE_INC_SEC_EPOCH` reader - Include security EPOCH in DICE"]
pub type DICE_INC_SEC_EPOCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DICE_INC_SEC_EPOCH` writer - Include security EPOCH in DICE"]
pub type DICE_INC_SEC_EPOCH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_BOOT_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SEC_BOOT_EN` reader - Secure boot enable. 00 - Plain image (internal flash with or without CRC) 01, 10, 11 - Boot signed images. (internal flash, RSA signed)"]
pub type SEC_BOOT_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC_BOOT_EN` writer - Secure boot enable. 00 - Plain image (internal flash with or without CRC) 01, 10, 11 - Boot signed images. (internal flash, RSA signed)"]
pub type SEC_BOOT_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECURE_BOOT_CFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Use RSA4096 keys only. 00- RSA2048 keys 01, 10, 11 - RSA4096 keys"]
    #[inline(always)]
    pub fn rsa4k(&self) -> RSA4K_R {
        RSA4K_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Include NXP area in DICE computation. 00 - not included 01, 10, 11 - included"]
    #[inline(always)]
    pub fn dice_enc_nxp_cfg(&self) -> DICE_ENC_NXP_CFG_R {
        DICE_ENC_NXP_CFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Include Customer factory area (including keys) in DICE computation. 00 - not included 01, 10, 11 - included"]
    #[inline(always)]
    pub fn dice_cust_cfg(&self) -> DICE_CUST_CFG_R {
        DICE_CUST_CFG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Skip DICE computation. 00 - Enable DICE 01,10,11 - Disable DICE"]
    #[inline(always)]
    pub fn skip_dice(&self) -> SKIP_DICE_R {
        SKIP_DICE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TrustZone-M mode. 00 - TZM mode in image header. 01 - Disable TZ-M. Boots to NonSecure. 10 - TZ-M enable boots to secure mode. 11 - Preset TZM checker from image header."]
    #[inline(always)]
    pub fn tzm_image_type(&self) -> TZM_IMAGE_TYPE_R {
        TZM_IMAGE_TYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Block PUF key code generation. 00 - Enable Key code generation 01, 10, 11 - Disable key code generation"]
    #[inline(always)]
    pub fn block_set_key(&self) -> BLOCK_SET_KEY_R {
        BLOCK_SET_KEY_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Block PUF enrollement. 00 - Enable enrollment mode 01, 10, 11 - Disable further enrollmnet"]
    #[inline(always)]
    pub fn block_enroll(&self) -> BLOCK_ENROLL_R {
        BLOCK_ENROLL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Include security EPOCH in DICE"]
    #[inline(always)]
    pub fn dice_inc_sec_epoch(&self) -> DICE_INC_SEC_EPOCH_R {
        DICE_INC_SEC_EPOCH_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Secure boot enable. 00 - Plain image (internal flash with or without CRC) 01, 10, 11 - Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    pub fn sec_boot_en(&self) -> SEC_BOOT_EN_R {
        SEC_BOOT_EN_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Use RSA4096 keys only. 00- RSA2048 keys 01, 10, 11 - RSA4096 keys"]
    #[inline(always)]
    #[must_use]
    pub fn rsa4k(&mut self) -> RSA4K_W<0> {
        RSA4K_W::new(self)
    }
    #[doc = "Bits 2:3 - Include NXP area in DICE computation. 00 - not included 01, 10, 11 - included"]
    #[inline(always)]
    #[must_use]
    pub fn dice_enc_nxp_cfg(&mut self) -> DICE_ENC_NXP_CFG_W<2> {
        DICE_ENC_NXP_CFG_W::new(self)
    }
    #[doc = "Bits 4:5 - Include Customer factory area (including keys) in DICE computation. 00 - not included 01, 10, 11 - included"]
    #[inline(always)]
    #[must_use]
    pub fn dice_cust_cfg(&mut self) -> DICE_CUST_CFG_W<4> {
        DICE_CUST_CFG_W::new(self)
    }
    #[doc = "Bits 6:7 - Skip DICE computation. 00 - Enable DICE 01,10,11 - Disable DICE"]
    #[inline(always)]
    #[must_use]
    pub fn skip_dice(&mut self) -> SKIP_DICE_W<6> {
        SKIP_DICE_W::new(self)
    }
    #[doc = "Bits 8:9 - TrustZone-M mode. 00 - TZM mode in image header. 01 - Disable TZ-M. Boots to NonSecure. 10 - TZ-M enable boots to secure mode. 11 - Preset TZM checker from image header."]
    #[inline(always)]
    #[must_use]
    pub fn tzm_image_type(&mut self) -> TZM_IMAGE_TYPE_W<8> {
        TZM_IMAGE_TYPE_W::new(self)
    }
    #[doc = "Bits 10:11 - Block PUF key code generation. 00 - Enable Key code generation 01, 10, 11 - Disable key code generation"]
    #[inline(always)]
    #[must_use]
    pub fn block_set_key(&mut self) -> BLOCK_SET_KEY_W<10> {
        BLOCK_SET_KEY_W::new(self)
    }
    #[doc = "Bits 12:13 - Block PUF enrollement. 00 - Enable enrollment mode 01, 10, 11 - Disable further enrollmnet"]
    #[inline(always)]
    #[must_use]
    pub fn block_enroll(&mut self) -> BLOCK_ENROLL_W<12> {
        BLOCK_ENROLL_W::new(self)
    }
    #[doc = "Bits 14:15 - Include security EPOCH in DICE"]
    #[inline(always)]
    #[must_use]
    pub fn dice_inc_sec_epoch(&mut self) -> DICE_INC_SEC_EPOCH_W<14> {
        DICE_INC_SEC_EPOCH_W::new(self)
    }
    #[doc = "Bits 30:31 - Secure boot enable. 00 - Plain image (internal flash with or without CRC) 01, 10, 11 - Boot signed images. (internal flash, RSA signed)"]
    #[inline(always)]
    #[must_use]
    pub fn sec_boot_en(&mut self) -> SEC_BOOT_EN_W<30> {
        SEC_BOOT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secure_boot_cfg](index.html) module"]
pub struct SECURE_BOOT_CFG_SPEC;
impl crate::RegisterSpec for SECURE_BOOT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secure_boot_cfg::R](R) reader structure"]
impl crate::Readable for SECURE_BOOT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secure_boot_cfg::W](W) writer structure"]
impl crate::Writable for SECURE_BOOT_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECURE_BOOT_CFG to value 0"]
impl crate::Resettable for SECURE_BOOT_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

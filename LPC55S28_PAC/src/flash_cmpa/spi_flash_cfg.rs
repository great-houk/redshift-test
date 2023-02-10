#[doc = "Register `SPI_FLASH_CFG` reader"]
pub struct R(crate::R<SPI_FLASH_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FLASH_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_FLASH_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_FLASH_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_FLASH_CFG` writer"]
pub struct W(crate::W<SPI_FLASH_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_FLASH_CFG_SPEC>;
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
impl From<crate::W<SPI_FLASH_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_FLASH_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_RECOVERY_BOOT_EN` reader - SPI flash recovery boot is enabled, if non-zero value is written to this field."]
pub type SPI_RECOVERY_BOOT_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_RECOVERY_BOOT_EN` writer - SPI flash recovery boot is enabled, if non-zero value is written to this field."]
pub type SPI_RECOVERY_BOOT_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_FLASH_CFG_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - SPI flash recovery boot is enabled, if non-zero value is written to this field."]
    #[inline(always)]
    pub fn spi_recovery_boot_en(&self) -> SPI_RECOVERY_BOOT_EN_R {
        SPI_RECOVERY_BOOT_EN_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SPI flash recovery boot is enabled, if non-zero value is written to this field."]
    #[inline(always)]
    #[must_use]
    pub fn spi_recovery_boot_en(&mut self) -> SPI_RECOVERY_BOOT_EN_W<0> {
        SPI_RECOVERY_BOOT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_flash_cfg](index.html) module"]
pub struct SPI_FLASH_CFG_SPEC;
impl crate::RegisterSpec for SPI_FLASH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_flash_cfg::R](R) reader structure"]
impl crate::Readable for SPI_FLASH_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_flash_cfg::W](W) writer structure"]
impl crate::Writable for SPI_FLASH_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_FLASH_CFG to value 0"]
impl crate::Resettable for SPI_FLASH_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

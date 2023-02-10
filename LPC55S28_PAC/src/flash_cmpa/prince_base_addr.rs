#[doc = "Register `PRINCE_BASE_ADDR` reader"]
pub struct R(crate::R<PRINCE_BASE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRINCE_BASE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRINCE_BASE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRINCE_BASE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRINCE_BASE_ADDR` writer"]
pub struct W(crate::W<PRINCE_BASE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRINCE_BASE_ADDR_SPEC>;
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
impl From<crate::W<PRINCE_BASE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRINCE_BASE_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR0_PRG` reader - Programmable portion of the base address of region 0."]
pub type ADDR0_PRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR0_PRG` writer - Programmable portion of the base address of region 0."]
pub type ADDR0_PRG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDR1_PRG` reader - Programmable portion of the base address of region 1."]
pub type ADDR1_PRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR1_PRG` writer - Programmable portion of the base address of region 1."]
pub type ADDR1_PRG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDR2_PRG` reader - Programmable portion of the base address of region 2."]
pub type ADDR2_PRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR2_PRG` writer - Programmable portion of the base address of region 2."]
pub type ADDR2_PRG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `LOCK_REG0` reader - Lock PRINCE region0 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
pub type LOCK_REG0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK_REG0` writer - Lock PRINCE region0 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
pub type LOCK_REG0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LOCK_REG1` reader - Lock PRINCE region1 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
pub type LOCK_REG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK_REG1` writer - Lock PRINCE region1 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
pub type LOCK_REG1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LOCK_REG2` reader - Lock PRINCE region2 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
pub type LOCK_REG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK_REG2` writer - Lock PRINCE region2 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
pub type LOCK_REG2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `REG0_ERASE_CHECK_EN` reader - For PRINCE region0 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
pub type REG0_ERASE_CHECK_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REG0_ERASE_CHECK_EN` writer - For PRINCE region0 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
pub type REG0_ERASE_CHECK_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `REG1_ERASE_CHECK_EN` reader - For PRINCE region1 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
pub type REG1_ERASE_CHECK_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REG1_ERASE_CHECK_EN` writer - For PRINCE region1 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
pub type REG1_ERASE_CHECK_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, u8, 2, O>;
#[doc = "Field `REG2_ERASE_CHECK_EN` reader - For PRINCE region2 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
pub type REG2_ERASE_CHECK_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REG2_ERASE_CHECK_EN` writer - For PRINCE region2 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
pub type REG2_ERASE_CHECK_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRINCE_BASE_ADDR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub fn addr0_prg(&self) -> ADDR0_PRG_R {
        ADDR0_PRG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Programmable portion of the base address of region 1."]
    #[inline(always)]
    pub fn addr1_prg(&self) -> ADDR1_PRG_R {
        ADDR1_PRG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Programmable portion of the base address of region 2."]
    #[inline(always)]
    pub fn addr2_prg(&self) -> ADDR2_PRG_R {
        ADDR2_PRG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Lock PRINCE region0 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline(always)]
    pub fn lock_reg0(&self) -> LOCK_REG0_R {
        LOCK_REG0_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Lock PRINCE region1 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline(always)]
    pub fn lock_reg1(&self) -> LOCK_REG1_R {
        LOCK_REG1_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Lock PRINCE region2 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline(always)]
    pub fn lock_reg2(&self) -> LOCK_REG2_R {
        LOCK_REG2_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - For PRINCE region0 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline(always)]
    pub fn reg0_erase_check_en(&self) -> REG0_ERASE_CHECK_EN_R {
        REG0_ERASE_CHECK_EN_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - For PRINCE region1 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline(always)]
    pub fn reg1_erase_check_en(&self) -> REG1_ERASE_CHECK_EN_R {
        REG1_ERASE_CHECK_EN_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - For PRINCE region2 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline(always)]
    pub fn reg2_erase_check_en(&self) -> REG2_ERASE_CHECK_EN_R {
        REG2_ERASE_CHECK_EN_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Programmable portion of the base address of region 0."]
    #[inline(always)]
    #[must_use]
    pub fn addr0_prg(&mut self) -> ADDR0_PRG_W<0> {
        ADDR0_PRG_W::new(self)
    }
    #[doc = "Bits 4:7 - Programmable portion of the base address of region 1."]
    #[inline(always)]
    #[must_use]
    pub fn addr1_prg(&mut self) -> ADDR1_PRG_W<4> {
        ADDR1_PRG_W::new(self)
    }
    #[doc = "Bits 8:11 - Programmable portion of the base address of region 2."]
    #[inline(always)]
    #[must_use]
    pub fn addr2_prg(&mut self) -> ADDR2_PRG_W<8> {
        ADDR2_PRG_W::new(self)
    }
    #[doc = "Bits 16:17 - Lock PRINCE region0 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline(always)]
    #[must_use]
    pub fn lock_reg0(&mut self) -> LOCK_REG0_W<16> {
        LOCK_REG0_W::new(self)
    }
    #[doc = "Bits 18:19 - Lock PRINCE region1 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline(always)]
    #[must_use]
    pub fn lock_reg1(&mut self) -> LOCK_REG1_W<18> {
        LOCK_REG1_W::new(self)
    }
    #[doc = "Bits 20:21 - Lock PRINCE region2 settings. 00 - Region is not locked. 01, 10, 11 - Region is locked."]
    #[inline(always)]
    #[must_use]
    pub fn lock_reg2(&mut self) -> LOCK_REG2_W<20> {
        LOCK_REG2_W::new(self)
    }
    #[doc = "Bits 24:25 - For PRINCE region0 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn reg0_erase_check_en(&mut self) -> REG0_ERASE_CHECK_EN_W<24> {
        REG0_ERASE_CHECK_EN_W::new(self)
    }
    #[doc = "Bits 26:27 - For PRINCE region1 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn reg1_erase_check_en(&mut self) -> REG1_ERASE_CHECK_EN_W<26> {
        REG1_ERASE_CHECK_EN_W::new(self)
    }
    #[doc = "Bits 28:29 - For PRINCE region2 enable checking whether all encrypted pages are erased together. 00 - Check is disabled. 01, 10, 11 - Check is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn reg2_erase_check_en(&mut self) -> REG2_ERASE_CHECK_EN_W<28> {
        REG2_ERASE_CHECK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prince_base_addr](index.html) module"]
pub struct PRINCE_BASE_ADDR_SPEC;
impl crate::RegisterSpec for PRINCE_BASE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prince_base_addr::R](R) reader structure"]
impl crate::Readable for PRINCE_BASE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prince_base_addr::W](W) writer structure"]
impl crate::Writable for PRINCE_BASE_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRINCE_BASE_ADDR to value 0"]
impl crate::Resettable for PRINCE_BASE_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

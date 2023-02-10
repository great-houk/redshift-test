#[doc = "Register `PORTMODE` reader"]
pub struct R(crate::R<PORTMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTMODE` writer"]
pub struct W(crate::W<PORTMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTMODE_SPEC>;
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
impl From<crate::W<PORTMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - Port ID pin value."]
pub type ID_R = crate::BitReader<bool>;
#[doc = "Field `ID` writer - Port ID pin value."]
pub type ID_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTMODE_SPEC, bool, O>;
#[doc = "Field `ID_EN` reader - Port ID pin pull-up enable."]
pub type ID_EN_R = crate::BitReader<bool>;
#[doc = "Field `ID_EN` writer - Port ID pin pull-up enable."]
pub type ID_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTMODE_SPEC, bool, O>;
#[doc = "Field `DEV_ENABLE` reader - 1: device 0: host."]
pub type DEV_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DEV_ENABLE` writer - 1: device 0: host."]
pub type DEV_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PORTMODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port ID pin value."]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Port ID pin pull-up enable."]
    #[inline(always)]
    pub fn id_en(&self) -> ID_EN_R {
        ID_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: device 0: host."]
    #[inline(always)]
    pub fn dev_enable(&self) -> DEV_ENABLE_R {
        DEV_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port ID pin value."]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<0> {
        ID_W::new(self)
    }
    #[doc = "Bit 8 - Port ID pin pull-up enable."]
    #[inline(always)]
    #[must_use]
    pub fn id_en(&mut self) -> ID_EN_W<8> {
        ID_EN_W::new(self)
    }
    #[doc = "Bit 16 - 1: device 0: host."]
    #[inline(always)]
    #[must_use]
    pub fn dev_enable(&mut self) -> DEV_ENABLE_W<16> {
        DEV_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the port if it is attached to the host block or the device block\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portmode](index.html) module"]
pub struct PORTMODE_SPEC;
impl crate::RegisterSpec for PORTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portmode::R](R) reader structure"]
impl crate::Readable for PORTMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portmode::W](W) writer structure"]
impl crate::Writable for PORTMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PORTMODE to value 0"]
impl crate::Resettable for PORTMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

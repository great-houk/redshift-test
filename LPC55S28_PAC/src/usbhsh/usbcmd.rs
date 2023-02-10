#[doc = "Register `USBCMD` reader"]
pub struct R(crate::R<USBCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCMD` writer"]
pub struct W(crate::W<USBCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCMD_SPEC>;
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
impl From<crate::W<USBCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS` reader - Run/Stop: 1b = Run."]
pub type RS_R = crate::BitReader<bool>;
#[doc = "Field `RS` writer - Run/Stop: 1b = Run."]
pub type RS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `HCRESET` reader - Host Controller Reset: This control bit is used by the software to reset the host controller."]
pub type HCRESET_R = crate::BitReader<bool>;
#[doc = "Field `HCRESET` writer - Host Controller Reset: This control bit is used by the software to reset the host controller."]
pub type HCRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `FLS` reader - Frame List Size: This field specifies the size of the frame list."]
pub type FLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLS` writer - Frame List Size: This field specifies the size of the frame list."]
pub type FLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBCMD_SPEC, u8, u8, 2, O>;
#[doc = "Field `LHCR` reader - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
pub type LHCR_R = crate::BitReader<bool>;
#[doc = "Field `LHCR` writer - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
pub type LHCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `ATL_EN` reader - ATL List enabled."]
pub type ATL_EN_R = crate::BitReader<bool>;
#[doc = "Field `ATL_EN` writer - ATL List enabled."]
pub type ATL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `ISO_EN` reader - ISO List enabled."]
pub type ISO_EN_R = crate::BitReader<bool>;
#[doc = "Field `ISO_EN` writer - ISO List enabled."]
pub type ISO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
#[doc = "Field `INT_EN` reader - INT List enabled."]
pub type INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `INT_EN` writer - INT List enabled."]
pub type INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Run/Stop: 1b = Run."]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[inline(always)]
    pub fn hcreset(&self) -> HCRESET_R {
        HCRESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Frame List Size: This field specifies the size of the frame list."]
    #[inline(always)]
    pub fn fls(&self) -> FLS_R {
        FLS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[inline(always)]
    pub fn lhcr(&self) -> LHCR_R {
        LHCR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ATL List enabled."]
    #[inline(always)]
    pub fn atl_en(&self) -> ATL_EN_R {
        ATL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ISO List enabled."]
    #[inline(always)]
    pub fn iso_en(&self) -> ISO_EN_R {
        ISO_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - INT List enabled."]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run/Stop: 1b = Run."]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<0> {
        RS_W::new(self)
    }
    #[doc = "Bit 1 - Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[inline(always)]
    #[must_use]
    pub fn hcreset(&mut self) -> HCRESET_W<1> {
        HCRESET_W::new(self)
    }
    #[doc = "Bits 2:3 - Frame List Size: This field specifies the size of the frame list."]
    #[inline(always)]
    #[must_use]
    pub fn fls(&mut self) -> FLS_W<2> {
        FLS_W::new(self)
    }
    #[doc = "Bit 7 - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[inline(always)]
    #[must_use]
    pub fn lhcr(&mut self) -> LHCR_W<7> {
        LHCR_W::new(self)
    }
    #[doc = "Bit 8 - ATL List enabled."]
    #[inline(always)]
    #[must_use]
    pub fn atl_en(&mut self) -> ATL_EN_W<8> {
        ATL_EN_W::new(self)
    }
    #[doc = "Bit 9 - ISO List enabled."]
    #[inline(always)]
    #[must_use]
    pub fn iso_en(&mut self) -> ISO_EN_W<9> {
        ISO_EN_W::new(self)
    }
    #[doc = "Bit 10 - INT List enabled."]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> INT_EN_W<10> {
        INT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcmd](index.html) module"]
pub struct USBCMD_SPEC;
impl crate::RegisterSpec for USBCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbcmd::R](R) reader structure"]
impl crate::Readable for USBCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbcmd::W](W) writer structure"]
impl crate::Writable for USBCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCMD to value 0"]
impl crate::Resettable for USBCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

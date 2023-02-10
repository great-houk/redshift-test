#[doc = "Register `USBHS_PHY_CTRL` reader"]
pub struct R(crate::R<USBHS_PHY_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_PHY_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_PHY_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_PHY_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_PHY_CTRL` writer"]
pub struct W(crate::W<USBHS_PHY_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_PHY_CTRL_SPEC>;
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
impl From<crate::W<USBHS_PHY_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_PHY_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `usb_vbusvalid_ext` reader - Override value for Vbus if using external detectors."]
pub type USB_VBUSVALID_EXT_R = crate::BitReader<bool>;
#[doc = "Field `usb_vbusvalid_ext` writer - Override value for Vbus if using external detectors."]
pub type USB_VBUSVALID_EXT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USBHS_PHY_CTRL_SPEC, bool, O>;
#[doc = "Field `usb_id_ext` reader - Override value for ID if using external detectors."]
pub type USB_ID_EXT_R = crate::BitReader<bool>;
#[doc = "Field `usb_id_ext` writer - Override value for ID if using external detectors."]
pub type USB_ID_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBHS_PHY_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Override value for Vbus if using external detectors."]
    #[inline(always)]
    pub fn usb_vbusvalid_ext(&self) -> USB_VBUSVALID_EXT_R {
        USB_VBUSVALID_EXT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Override value for ID if using external detectors."]
    #[inline(always)]
    pub fn usb_id_ext(&self) -> USB_ID_EXT_R {
        USB_ID_EXT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override value for Vbus if using external detectors."]
    #[inline(always)]
    #[must_use]
    pub fn usb_vbusvalid_ext(&mut self) -> USB_VBUSVALID_EXT_W<0> {
        USB_VBUSVALID_EXT_W::new(self)
    }
    #[doc = "Bit 1 - Override value for ID if using external detectors."]
    #[inline(always)]
    #[must_use]
    pub fn usb_id_ext(&mut self) -> USB_ID_EXT_W<1> {
        USB_ID_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB High Speed Phy Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_phy_ctrl](index.html) module"]
pub struct USBHS_PHY_CTRL_SPEC;
impl crate::RegisterSpec for USBHS_PHY_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_phy_ctrl::R](R) reader structure"]
impl crate::Readable for USBHS_PHY_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_phy_ctrl::W](W) writer structure"]
impl crate::Writable for USBHS_PHY_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBHS_PHY_CTRL to value 0x08"]
impl crate::Resettable for USBHS_PHY_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}

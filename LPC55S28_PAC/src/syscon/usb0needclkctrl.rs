#[doc = "Register `USB0NEEDCLKCTRL` reader"]
pub struct R(crate::R<USB0NEEDCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB0NEEDCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB0NEEDCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB0NEEDCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB0NEEDCLKCTRL` writer"]
pub struct W(crate::W<USB0NEEDCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB0NEEDCLKCTRL_SPEC>;
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
impl From<crate::W<USB0NEEDCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB0NEEDCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AP_FS_DEV_NEEDCLK` reader - USB0 Device USB0_NEEDCLK signal control:."]
pub type AP_FS_DEV_NEEDCLK_R = crate::BitReader<AP_FS_DEV_NEEDCLK_A>;
#[doc = "USB0 Device USB0_NEEDCLK signal control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AP_FS_DEV_NEEDCLK_A {
    #[doc = "0: Under hardware control."]
    HW_CTRL = 0,
    #[doc = "1: Forced high."]
    FORCED = 1,
}
impl From<AP_FS_DEV_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_FS_DEV_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl AP_FS_DEV_NEEDCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_FS_DEV_NEEDCLK_A {
        match self.bits {
            false => AP_FS_DEV_NEEDCLK_A::HW_CTRL,
            true => AP_FS_DEV_NEEDCLK_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `HW_CTRL`"]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == AP_FS_DEV_NEEDCLK_A::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == AP_FS_DEV_NEEDCLK_A::FORCED
    }
}
#[doc = "Field `AP_FS_DEV_NEEDCLK` writer - USB0 Device USB0_NEEDCLK signal control:."]
pub type AP_FS_DEV_NEEDCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB0NEEDCLKCTRL_SPEC, AP_FS_DEV_NEEDCLK_A, O>;
impl<'a, const O: u8> AP_FS_DEV_NEEDCLK_W<'a, O> {
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut W {
        self.variant(AP_FS_DEV_NEEDCLK_A::HW_CTRL)
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_FS_DEV_NEEDCLK_A::FORCED)
    }
}
#[doc = "Field `POL_FS_DEV_NEEDCLK` reader - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
pub type POL_FS_DEV_NEEDCLK_R = crate::BitReader<POL_FS_DEV_NEEDCLK_A>;
#[doc = "USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_FS_DEV_NEEDCLK_A {
    #[doc = "0: Falling edge of device USB0_NEEDCLK triggers wake-up."]
    FALLING = 0,
    #[doc = "1: Rising edge of device USB0_NEEDCLK triggers wake-up."]
    RISING = 1,
}
impl From<POL_FS_DEV_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_FS_DEV_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_FS_DEV_NEEDCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_FS_DEV_NEEDCLK_A {
        match self.bits {
            false => POL_FS_DEV_NEEDCLK_A::FALLING,
            true => POL_FS_DEV_NEEDCLK_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == POL_FS_DEV_NEEDCLK_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == POL_FS_DEV_NEEDCLK_A::RISING
    }
}
#[doc = "Field `POL_FS_DEV_NEEDCLK` writer - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
pub type POL_FS_DEV_NEEDCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB0NEEDCLKCTRL_SPEC, POL_FS_DEV_NEEDCLK_A, O>;
impl<'a, const O: u8> POL_FS_DEV_NEEDCLK_W<'a, O> {
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(POL_FS_DEV_NEEDCLK_A::FALLING)
    }
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(POL_FS_DEV_NEEDCLK_A::RISING)
    }
}
#[doc = "Field `AP_FS_HOST_NEEDCLK` reader - USB0 Host USB0_NEEDCLK signal control:."]
pub type AP_FS_HOST_NEEDCLK_R = crate::BitReader<AP_FS_HOST_NEEDCLK_A>;
#[doc = "USB0 Host USB0_NEEDCLK signal control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AP_FS_HOST_NEEDCLK_A {
    #[doc = "0: Under hardware control."]
    HW_CTRL = 0,
    #[doc = "1: Forced high."]
    FORCED = 1,
}
impl From<AP_FS_HOST_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_FS_HOST_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl AP_FS_HOST_NEEDCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_FS_HOST_NEEDCLK_A {
        match self.bits {
            false => AP_FS_HOST_NEEDCLK_A::HW_CTRL,
            true => AP_FS_HOST_NEEDCLK_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `HW_CTRL`"]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == AP_FS_HOST_NEEDCLK_A::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == AP_FS_HOST_NEEDCLK_A::FORCED
    }
}
#[doc = "Field `AP_FS_HOST_NEEDCLK` writer - USB0 Host USB0_NEEDCLK signal control:."]
pub type AP_FS_HOST_NEEDCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB0NEEDCLKCTRL_SPEC, AP_FS_HOST_NEEDCLK_A, O>;
impl<'a, const O: u8> AP_FS_HOST_NEEDCLK_W<'a, O> {
    #[doc = "Under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut W {
        self.variant(AP_FS_HOST_NEEDCLK_A::HW_CTRL)
    }
    #[doc = "Forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_FS_HOST_NEEDCLK_A::FORCED)
    }
}
#[doc = "Field `POL_FS_HOST_NEEDCLK` reader - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
pub type POL_FS_HOST_NEEDCLK_R = crate::BitReader<POL_FS_HOST_NEEDCLK_A>;
#[doc = "USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_FS_HOST_NEEDCLK_A {
    #[doc = "0: Falling edge of device USB0_NEEDCLK triggers wake-up."]
    FALLING = 0,
    #[doc = "1: Rising edge of device USB0_NEEDCLK triggers wake-up."]
    RISING = 1,
}
impl From<POL_FS_HOST_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_FS_HOST_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_FS_HOST_NEEDCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_FS_HOST_NEEDCLK_A {
        match self.bits {
            false => POL_FS_HOST_NEEDCLK_A::FALLING,
            true => POL_FS_HOST_NEEDCLK_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == POL_FS_HOST_NEEDCLK_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == POL_FS_HOST_NEEDCLK_A::RISING
    }
}
#[doc = "Field `POL_FS_HOST_NEEDCLK` writer - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
pub type POL_FS_HOST_NEEDCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB0NEEDCLKCTRL_SPEC, POL_FS_HOST_NEEDCLK_A, O>;
impl<'a, const O: u8> POL_FS_HOST_NEEDCLK_W<'a, O> {
    #[doc = "Falling edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(POL_FS_HOST_NEEDCLK_A::FALLING)
    }
    #[doc = "Rising edge of device USB0_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(POL_FS_HOST_NEEDCLK_A::RISING)
    }
}
impl R {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_dev_needclk(&self) -> AP_FS_DEV_NEEDCLK_R {
        AP_FS_DEV_NEEDCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_dev_needclk(&self) -> POL_FS_DEV_NEEDCLK_R {
        POL_FS_DEV_NEEDCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB0 Host USB0_NEEDCLK signal control:."]
    #[inline(always)]
    pub fn ap_fs_host_needclk(&self) -> AP_FS_HOST_NEEDCLK_R {
        AP_FS_HOST_NEEDCLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    pub fn pol_fs_host_needclk(&self) -> POL_FS_HOST_NEEDCLK_R {
        POL_FS_HOST_NEEDCLK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB0 Device USB0_NEEDCLK signal control:."]
    #[inline(always)]
    #[must_use]
    pub fn ap_fs_dev_needclk(&mut self) -> AP_FS_DEV_NEEDCLK_W<0> {
        AP_FS_DEV_NEEDCLK_W::new(self)
    }
    #[doc = "Bit 1 - USB0 Device USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    #[must_use]
    pub fn pol_fs_dev_needclk(&mut self) -> POL_FS_DEV_NEEDCLK_W<1> {
        POL_FS_DEV_NEEDCLK_W::new(self)
    }
    #[doc = "Bit 2 - USB0 Host USB0_NEEDCLK signal control:."]
    #[inline(always)]
    #[must_use]
    pub fn ap_fs_host_needclk(&mut self) -> AP_FS_HOST_NEEDCLK_W<2> {
        AP_FS_HOST_NEEDCLK_W::new(self)
    }
    #[doc = "Bit 3 - USB0 Host USB0_NEEDCLK polarity for triggering the USB0 wake-up interrupt:."]
    #[inline(always)]
    #[must_use]
    pub fn pol_fs_host_needclk(&mut self) -> POL_FS_HOST_NEEDCLK_W<3> {
        POL_FS_HOST_NEEDCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB0 need clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb0needclkctrl](index.html) module"]
pub struct USB0NEEDCLKCTRL_SPEC;
impl crate::RegisterSpec for USB0NEEDCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb0needclkctrl::R](R) reader structure"]
impl crate::Readable for USB0NEEDCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb0needclkctrl::W](W) writer structure"]
impl crate::Writable for USB0NEEDCLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB0NEEDCLKCTRL to value 0"]
impl crate::Resettable for USB0NEEDCLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
